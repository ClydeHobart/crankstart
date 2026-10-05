use {
    crate::{
        CrankstartAPI, define_crankstart_api, define_enum_from_pd_flags, ensure, eprintln,
        pd_api::{
            FileOptions as PDFileOptions, FileStat, SDFile, SEEK_CUR, SEEK_END, SEEK_SET,
            ctypes::{c_char, c_void},
            playdate_file,
        },
        q,
        util::{
            ptr::{PtrTrait, UntypedPtr},
            singleton::Singleton,
            string::{ArrayStringTrait, ErrorArrayString, LongTempString, TempString},
        },
    },
    anyhow::{Result, anyhow},
    core::{
        cell::{Ref, RefMut},
        fmt::Write,
        mem::transmute,
        ptr::NonNull,
    },
    static_assertions::const_assert_eq,
};

define_enum_from_pd_flags! {
    #[repr(u8)]
    #[flags(FileOptions, PDFileOptions)]
    /// A parallel definition of [`PDFileOptions`] for use in the [`FileOptions`] typed enum flag set.
    #[derive(Clone, Copy, PartialEq)]
    pub enum FileOption {
        #[pd_flag(kFileRead)]
        Read,

        #[pd_flag(kFileReadData)]
        ReadData,

        #[pd_flag(kFileWrite)]
        Write,

        #[pd_flag(kFileAppend)]
        Append,
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Whence {
    Set,
    Cur,
    End,
}

const_assert_eq!(Whence::Set as u32, SEEK_SET);
const_assert_eq!(Whence::Cur as u32, SEEK_CUR);
const_assert_eq!(Whence::End as u32, SEEK_END);

#[derive(Default)]
pub struct FileState {
    was_removed: bool,
}

#[derive(Clone, PartialEq)]
pub struct FilePtr(UntypedPtr);

impl From<UntypedPtr> for FilePtr {
    fn from(value: UntypedPtr) -> Self {
        Self(value)
    }
}

impl PtrTrait for FilePtr {
    type PDType = SDFile;

    type State = FileState;

    fn get_untyped_ptr(&self) -> &UntypedPtr {
        &self.0
    }

    fn remove_pd_ptr(pd_ptr: NonNull<Self::PDType>, state: &Self::State) {
        if !state.was_removed {
            // This function's signature is set by the trait, so we can't propagate the error up.
            CrankstartAPI::get().file.close_internal(pd_ptr).ok();
        }
    }
}

define_crankstart_api! {
    pub struct FileAPI => playdate_file {
        ; // No sub-API fields
        pub(crate) geterr: unsafe extern "C" fn() -> *const c_char,
        pub(crate) listfiles: unsafe extern "C" fn(
            path: *const c_char,
            callback: Option<unsafe extern "C" fn(path: *const c_char, userdata: *mut c_void)>,
            userdata: *mut c_void,
            showhidden: i32,
        ) -> i32,
        pub(crate) stat: unsafe extern "C" fn(path: *const c_char, stat: *mut FileStat) -> i32,
        pub(crate) mkdir: unsafe extern "C" fn(path: *const c_char) -> i32,
        pub(crate) unlink: unsafe extern "C" fn(
            name: *const c_char,
            recursive: i32,
        ) -> i32,
        pub(crate) rename: unsafe extern "C" fn(
            from: *const c_char,
            to: *const c_char,
        ) -> i32,
        pub(crate) open:
            unsafe extern "C" fn(name: *const c_char, mode: PDFileOptions) -> *mut SDFile,
        pub(crate) close: unsafe extern "C" fn(file: *mut SDFile) -> i32,
        pub(crate) read: unsafe extern "C" fn(
            file: *mut SDFile,
            buf: *mut c_void,
            len: u32,
        ) -> i32,
        pub(crate) write: unsafe extern "C" fn(
            file: *mut SDFile,
            buf: *const c_void,
            len: u32,
        ) -> i32,
        pub(crate) flush: unsafe extern "C" fn(file: *mut SDFile) -> i32,
        pub(crate) tell: unsafe extern "C" fn(file: *mut SDFile) -> i32,
        pub(crate) seek: unsafe extern "C" fn(
            file: *mut SDFile,
            pos: i32,
            whence: i32,
        ) -> i32;
    }
}

impl FileAPI {
    pub fn get_err<'s>(&self, out_error_string: &'s mut TempString) -> Result<&'s TempString> {
        out_error_string.clear();

        let error_c_string: *const c_char = unsafe { (self.geterr)() };
        let try_set_c_str_truncating_result: Result<()> =
            out_error_string.try_set_c_str_truncating(error_c_string);

        // If `try_set_c_str_truncating` needs to truncate, it'll return `Err`, but still mutate. We
        // swallow that case silently.
        if try_set_c_str_truncating_result.is_err() && out_error_string.is_empty() {
            try_set_c_str_truncating_result?;
        }

        Ok(out_error_string)
    }

    pub fn open(&self, name: &str, mode: FileOptions) -> Result<FilePtr> {
        let name: LongTempString = LongTempString::clone_null_terminated_truncating(name);
        let name: *const c_char = name.as_ptr() as *const c_char;
        let mode: PDFileOptions = mode.into();
        let pd_file: *mut SDFile = unsafe { (self.open)(name, mode) };
        let pd_file: NonNull<SDFile> = q!(NonNull::new(pd_file).ok_or(()));
        let file: FilePtr = FilePtr::new(pd_file, FileState::default());

        Ok(file)
    }

    pub fn close(&self, file: FilePtr) -> Result<()> {
        // One reference for parameter `file` and one reference for what's stored in
        // `CrankstartAPI::ptr_manager`.
        ensure!(file.get_strong_count() == 2_usize);

        let mut file_state: RefMut<FileState> =
            Self::try_get_borrowable_and_not_removed_file_mut(&file)?;

        self.close_internal(file.get_pd_ptr()).map(|_| {
            file_state.was_removed = true;
        })
    }

    /// Reads up to len bytes from the file into the buffer buf. Returns the number of bytes read
    /// (0 indicating end of file), or -1 in case of error.
    pub fn read(&self, file: &FilePtr, buf: &mut [u8]) -> Result<usize> {
        Self::try_get_borrowable_and_not_removed_file(file)?;

        let file: *mut SDFile = file.get_pd_ptr().as_ptr();
        let len: u32 = buf.len().try_into()?;
        let buf: *mut c_void = buf.as_mut_ptr() as *mut c_void;
        let read_return_value: i32 = unsafe { (self.read)(file, buf, len) };

        match read_return_value {
            0_i32..=i32::MAX => Ok(read_return_value as usize),
            Self::ERROR_RETURN_VALUE => self.get_err_internal(),
            _ => Self::handle_unexpected_return_value(read_return_value),
        }
    }

    /// Writes the buffer of bytes buf to the file. Returns the number of bytes written, or -1 in
    /// case of error.
    pub fn write(&self, file: &FilePtr, buf: &[u8]) -> Result<usize> {
        Self::try_get_borrowable_and_not_removed_file(file)?;

        let file: *mut SDFile = file.get_pd_ptr().as_ptr();
        let len: u32 = buf.len().try_into()?;
        let buf: *const c_void = buf.as_ptr() as *mut c_void;
        let write_return_value: i32 = unsafe { (self.write)(file, buf, len) };

        match write_return_value {
            0_i32..=i32::MAX => Ok(write_return_value as usize),
            Self::ERROR_RETURN_VALUE => self.get_err_internal(),
            _ => Self::handle_unexpected_return_value(write_return_value),
        }
    }

    pub fn flush(&self, file: &FilePtr) -> Result<usize> {
        Self::try_get_borrowable_and_not_removed_file(file)?;

        let file: *mut SDFile = file.get_pd_ptr().as_ptr();
        let flush_return_value: i32 = unsafe { (self.flush)(file) };

        match flush_return_value {
            0_i32..=i32::MAX => Ok(flush_return_value as usize),
            Self::ERROR_RETURN_VALUE => self.get_err_internal(),
            _ => Self::handle_unexpected_return_value(flush_return_value),
        }
    }

    pub fn tell(&self, file: &FilePtr) -> Result<usize> {
        Self::try_get_borrowable_and_not_removed_file(file)?;

        let file: *mut SDFile = file.get_pd_ptr().as_ptr();
        let tell_return_value: i32 = unsafe { (self.tell)(file) };

        match tell_return_value {
            0_i32..=i32::MAX => Ok(tell_return_value as usize),
            Self::ERROR_RETURN_VALUE => self.get_err_internal(),
            _ => Self::handle_unexpected_return_value(tell_return_value),
        }
    }

    pub fn seek(&self, file: &FilePtr, pos: isize, whence: Whence) -> Result<()> {
        Self::try_get_borrowable_and_not_removed_file(file)?;

        let file: *mut SDFile = file.get_pd_ptr().as_ptr();
        let pos: i32 = pos.try_into()?;
        let whence: i32 = whence as i32;
        let seek_return_value: i32 = unsafe { (self.seek)(file, pos, whence) };

        const SUCCESS_RETURN_VALUE: i32 = 0_i32;

        match seek_return_value {
            SUCCESS_RETURN_VALUE => Ok(()),
            Self::ERROR_RETURN_VALUE => self.get_err_internal(),
            _ => Self::handle_unexpected_return_value(seek_return_value),
        }
    }

    const ERROR_RETURN_VALUE: i32 = -1_i32;

    fn close_internal(&self, pd_file: NonNull<SDFile>) -> Result<()> {
        let close_return_value: i32 = unsafe { (self.close)(pd_file.as_ptr()) };

        const SUCCESS_RETURN_VALUE: i32 = 0_i32;

        match close_return_value {
            SUCCESS_RETURN_VALUE => Ok(()),
            Self::ERROR_RETURN_VALUE => self.get_err_internal(),
            _ => Self::handle_unexpected_return_value(close_return_value),
        }
    }

    fn try_get_borrowable_and_not_removed_file<'f>(
        file: &'f FilePtr,
    ) -> Result<Ref<'f, FileState>> {
        let file_state: Ref<FileState> = q!(file.try_borrow_state().ok_or(()));

        ensure!(!file_state.was_removed);

        Ok(file_state)
    }

    fn try_get_borrowable_and_not_removed_file_mut<'f>(
        file: &'f FilePtr,
    ) -> Result<RefMut<'f, FileState>> {
        let file_state: RefMut<FileState> = q!(file.try_borrow_state_mut().ok_or(()));

        ensure!(!file_state.was_removed);

        Ok(file_state)
    }

    fn get_err_internal<T>(&self) -> Result<T> {
        let mut error_string: TempString = TempString::new();

        self.get_err(&mut error_string)?;

        Err(anyhow!(ErrorArrayString::from(error_string)))
    }

    fn handle_unexpected_return_value<T>(return_value: i32) -> Result<T> {
        let mut string: TempString = TempString::new();

        write!(&mut string, "Unexpected return value [{return_value}].")?;

        Err(anyhow!(ErrorArrayString::from(string)))
    }

    extern "C" fn list_files_callback(path: *const c_char, user_data: *mut c_void) {
        let list_files_callback = || -> Result<()> {
            let path: LongTempString = LongTempString::try_clone_c_str_truncating(path)?;
            let path: &str = path.as_str();
            // let callback: *mut dyn FnMut(&str) =
            //     unsafe { transmute::<*mut c_void, *mut dyn FnMut(&str)>(user_data) };

            // ensure!(!callback.is_null());

            // // We've just explicitly verified that it's not null.
            // let callback: &mut dyn FnMut(&str) = unsafe { callback.as_mut_unchecked() };

            // callback(path);

            Ok(())
        };

        match list_files_callback() {
            Err(e) => {
                eprintln!("{e}");
            }
            _ => (),
        }
    }
}

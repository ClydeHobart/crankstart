use {
    crate::{
        CrankstartAPI, define_crankstart_api, ensure,
        pd_api::{
            FileOptions, FileStat, SDFile,
            ctypes::{c_char, c_void},
            playdate_file,
        },
        q,
        util::{
            ptr::{PtrTrait, UntypedPtr},
            singleton::Singleton,
            string::{ArrayStringTrait, ErrorArrayString, TempString},
        },
    },
    anyhow::{Result, anyhow},
    core::{cell::RefMut, ptr::NonNull},
};

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
            unsafe extern "C" fn(name: *const c_char, mode: FileOptions) -> *mut SDFile,
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
    pub fn geterr<'s>(&self, out_error_string: &'s mut TempString) -> Result<&'s TempString> {
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

    fn close_internal(&self, pd_file: NonNull<SDFile>) -> Result<()> {
        if unsafe { (self.close)(pd_file.as_ptr()) } == 0_i32 {
            Ok(())
        } else {
            let mut error_string: TempString = TempString::new();

            self.geterr(&mut error_string)?;

            Err(anyhow!(ErrorArrayString::from(error_string)))
        }
    }

    fn try_get_borrowable_and_not_removed_file_mut<'f>(
        file: &'f FilePtr,
    ) -> Result<RefMut<'f, FileState>> {
        let file_state: RefMut<FileState> = q!(file.try_borrow_state_mut().ok_or(()));

        ensure!(!file_state.was_removed);

        Ok(file_state)
    }
}

use {
    crate::{
        CrankstartAPI, define_crankstart_api, define_enum_from_pd_flags, ensure, eprintln,
        pd_api::{
            FileOptions as PDFileOptions, FileStat as PDFileStat, PDDateTime, SDFile, SEEK_CUR,
            SEEK_END, SEEK_SET,
            ctypes::{c_char, c_void},
            playdate_file,
        },
        q,
        util::{
            callback::TempCallback,
            ptr::{PtrTrait, UntypedPtr},
            singleton::Singleton,
            string::{ArrayStringTrait, ErrorArrayString, LongTempString, TempString},
        },
    },
    anyhow::{Result, anyhow},
    core::{
        cell::{Ref, RefMut},
        fmt::Write,
        ptr::NonNull,
    },
    static_assertions::const_assert_eq,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct FileStat {
    size: usize,
    is_dir: bool,
    date_time: PDDateTime,
}

impl From<PDFileStat> for FileStat {
    fn from(value: PDFileStat) -> Self {
        let is_dir: bool = value.isdir != 0_i32;
        let size: usize = value.size as usize;
        let date_time: PDDateTime = value.into();

        Self {
            size,
            is_dir,
            date_time,
        }
    }
}

define_enum_from_pd_flags! {
    #[repr(u8)]
    #[flags(FileOptions, PDFileOptions)]
    /// A parallel definition of [`PDFileOptions`] for use in the [`FileOptions`] typed enum flag
    /// set.
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
pub enum SeekWhence {
    Set,
    Cur,
    End,
}

const_assert_eq!(SeekWhence::Set as u32, SEEK_SET);
const_assert_eq!(SeekWhence::Cur as u32, SEEK_CUR);
const_assert_eq!(SeekWhence::End as u32, SEEK_END);

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
        // This function's signature is set by the trait, so we can't propagate the error up.
        CrankstartAPI::get().file.close_internal(pd_ptr, state).ok();
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
        pub(crate) stat: unsafe extern "C" fn(path: *const c_char, stat: *mut PDFileStat) -> i32,
        pub(crate) mkdir: unsafe extern "C" fn(path: *const c_char) -> i32,
        pub(crate) unlink: unsafe extern "C" fn(
            path: *const c_char,
            recursive: i32,
        ) -> i32,
        pub(crate) rename: unsafe extern "C" fn(
            from: *const c_char,
            to: *const c_char,
        ) -> i32,
        pub(crate) open:
            unsafe extern "C" fn(path: *const c_char, mode: PDFileOptions) -> *mut SDFile,
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
    /// Returns human-readable text describing the most recent internal filesystem error.
    ///
    /// Whatever this function returns was already returned in a `Result::Err` by the function call
    /// that caused the error.
    ///
    /// | Language | Equivalent function            |
    /// | :------- | :----------------------------- |
    /// | Rust     | `crankstart.file.get_err(...)` |
    /// | C        | `playdate->file->geterr()`     |
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

    /// Calls the given `callback` function for every file at `path`. Subfolders are indicated by a
    /// trailing slash '/' in the `str` provided to the `callback`. This function does not recurse
    /// into subfolders. If `show_hidden` is set, files beginning with a period will be included;
    /// otherwise, they are skipped.
    ///
    /// Returns an error if:
    ///
    /// * `path` isn't ASCII
    /// * no folder exists at `path`
    /// * the folder at `path` can't be opened
    ///
    /// | Language | Equivalent function               |
    /// | :------- | :-------------------------------- |
    /// | Rust     | `crankstart.file.list_files(...)` |
    /// | C        | `playdate->file->listfiles(...)`  |
    /// | Lua      | `playdate.file.listFiles(...)`    |
    pub fn list_files<C: FnMut(&str)>(
        &self,
        path: &str,
        callback: C,
        show_hidden: bool,
    ) -> Result<()> {
        ensure!(path.is_ascii());

        let path: LongTempString = LongTempString::clone_null_terminated_truncating(path);
        let path: *const c_char = path.as_ptr() as *const c_char;
        let mut callback: C = callback;
        let mut callback: TempCallback<&str> = TempCallback::from_temp_closure(&mut callback);
        let user_data: *mut c_void = (&mut callback) as *mut TempCallback<&str> as *mut c_void;

        extern "C" fn list_files_callback(path: *const c_char, user_data: *mut c_void) {
            let callback = || -> Result<()> {
                let path: LongTempString = LongTempString::try_clone_c_str_truncating(path)?;
                let path: &str = path.as_str();
                let callback: *const TempCallback<&str> = user_data as *const TempCallback<&str>;

                ensure!(!callback.is_null());
                ensure!(callback.is_aligned());

                // We've just explicitly verified that it's not null.
                let callback: &TempCallback<&str> = unsafe { callback.as_ref_unchecked() };

                callback.invoke(path);

                Ok(())
            };

            match callback() {
                Err(e) => {
                    eprintln!("{e}");
                }
                _ => (),
            }
        }

        let callback: Option<unsafe extern "C" fn(path: *const c_char, user_data: *mut c_void)> =
            Some(list_files_callback);
        let show_hidden: i32 = show_hidden as i32;

        self.handle_zero_ok_return_value(unsafe {
            (self.listfiles)(path, callback, user_data, show_hidden)
        })
    }

    /// Returns a `FileStat` with information about the file at `path`.
    ///
    /// Returns an error if:
    ///
    /// * `path` isn't ASCII
    /// * an internal filesystem error occurred
    ///
    /// | Language | Equivalent function         |
    /// | :------- | :-------------------------- |
    /// | Rust     | `crankstart.file.stat(...)` |
    /// | C        | `playdate->file->stat(...)` |
    pub fn stat(&self, path: &str) -> Result<FileStat> {
        ensure!(path.is_ascii());

        let path: LongTempString = LongTempString::clone_null_terminated_truncating(path);
        let path: *const c_char = path.as_ptr() as *const c_char;

        let mut pd_file_stat: PDFileStat = Default::default();

        let stat_return_value: i32 = {
            let pd_file_stat: *mut PDFileStat = &mut pd_file_stat as *mut PDFileStat;

            unsafe { (self.stat)(path, pd_file_stat) }
        };

        self.handle_zero_ok_return_value_with_ok(stat_return_value, |_| pd_file_stat.into())
    }

    /// Creates the given `path` in the `Data/<gameid>` folder. It does not create intermediate
    /// folders.
    ///
    /// Returns an error if:
    ///
    /// * `path` isn't ASCII
    /// * an internal filesystem error occurred
    ///
    /// | Language | Equivalent function             |
    /// | :------- | :------------------------------ |
    /// | Rust     | `crankstart.file.make_dir(...)` |
    /// | C        | `playdate->file->mkdir(...)`    |
    /// | Lua      | `playdate.file.mkdir(...)`      |
    pub fn make_dir(&self, path: &str) -> Result<()> {
        ensure!(path.is_ascii());

        let path: LongTempString = LongTempString::clone_null_terminated_truncating(path);
        let path: *const c_char = path.as_ptr() as *const c_char;

        self.handle_zero_ok_return_value(unsafe { (self.mkdir)(path) })
    }

    /// Deletes the file at path. If `recursive` is set and the target `path` is a folder, this
    /// deletes everything inside the folder (including folders, folders inside those, and so on) as
    /// well as the folder itself.
    ///
    /// Returns an error if:
    ///
    /// * `path` isn't ASCII
    /// * an internal filesystem error occurred
    ///
    /// | Language | Equivalent function           |
    /// | :------- | :---------------------------- |
    /// | Rust     | `crankstart.file.unlink(...)` |
    /// | C        | `playdate->file->unlink(...)` |
    pub fn unlink(&self, path: &str, recursive: bool) -> Result<()> {
        ensure!(path.is_ascii());

        let path: LongTempString = LongTempString::clone_null_terminated_truncating(path);
        let path: *const c_char = path.as_ptr() as *const c_char;
        let recursive: i32 = recursive as i32;

        self.handle_zero_ok_return_value(unsafe { (self.unlink)(path, recursive) })
    }

    /// Renames the file at `from` to `to`. It will overwrite the file at `to` without confirmation.
    /// It does not create intermediate folders.
    ///
    /// Returns an error if:
    ///
    /// * `from` isn't ASCII
    /// * `to` isn't ASCII
    /// * an internal filesystem error occurred
    ///
    /// | Language | Equivalent function           |
    /// | :------- | :---------------------------- |
    /// | Rust     | `crankstart.file.rename(...)` |
    /// | C        | `playdate->file->rename(...)` |
    /// | Lua      | `playdate.file.rename(...)`   |
    pub fn rename(&self, from: &str, to: &str) -> Result<()> {
        ensure!(from.is_ascii());
        ensure!(to.is_ascii());

        let from: LongTempString = LongTempString::clone_null_terminated_truncating(from);
        let from: *const c_char = from.as_ptr() as *const c_char;
        let to: LongTempString = LongTempString::clone_null_terminated_truncating(to);
        let to: *const c_char = to.as_ptr() as *const c_char;

        self.handle_zero_ok_return_value(unsafe { (self.rename)(from, to) })
    }

    /// Opens a handle for the file at `path`. The [`FileOption::Read`] mode opens a file in the
    /// game pdx, while [`FileOption::ReadData`] searches the game’s data folder; to search the data
    /// folder first then fall back on the game pdx, set both [`FileOption::Read`] and [`FileOption
    /// ::ReadData`] in `mode`. [`FileOption::Write`] and [`FileOption::Append`] always write to the
    /// data folder. The file will be closed on drop, but function [`Self::close`] is provided for
    /// the sake of clarity.
    ///
    /// Returns an error if:
    ///
    /// * `path` isn't ASCII
    /// * a file at `path` cannot be opened (the filesystem has a limit of 64 simultaneous open
    ///   files)
    ///
    /// | Language | Equivalent function         |
    /// | :------- | :-------------------------- |
    /// | Rust     | `crankstart.file.open(...)` |
    /// | C        | `playdate->file->open(...)` |
    /// | Lua      | `playdate.file.open(...)`   |
    pub fn open(&self, path: &str, mode: FileOptions) -> Result<FilePtr> {
        ensure!(path.is_ascii());

        let path: LongTempString = LongTempString::clone_null_terminated_truncating(path);
        let path: *const c_char = path.as_ptr() as *const c_char;
        let mode: PDFileOptions = mode.into();
        let pd_file: *mut SDFile = unsafe { (self.open)(path, mode) };
        let pd_file: NonNull<SDFile> = NonNull::new(pd_file)
            .ok_or_else(|| self.handle_err_return_value::<()>().unwrap_err())?;
        let file: FilePtr = FilePtr::new(pd_file, FileState::default());

        Ok(file)
    }

    /// Closes the given `file` handle.
    ///
    /// Returns an error if:
    ///
    /// * the user still has another reference to `file` (determined by an `Rc` inside [`FilePtr`]),
    ///   in which case the `file` is left open
    /// * an internal filesystem error occurred
    ///
    /// | Language | Equivalent function          |
    /// | :------- | :--------------------------- |
    /// | Rust     | `crankstart.file.close(...)` |
    /// | C        | `playdate->file->close(...)` |
    /// | Lua      | `playdate.file.close(...)`   |
    pub fn close(&self, file: FilePtr) -> Result<()> {
        // One reference for parameter `file` and one reference for what's stored in
        // `CrankstartAPI::ptr_manager`.
        ensure!(file.get_strong_count() == 2_usize);

        let mut file_state: RefMut<FileState> =
            Self::try_get_borrowable_and_not_removed_file_mut(&file)?;

        self.close_internal(file.get_pd_ptr(), &*file_state)
            .map(|_| {
                file_state.was_removed = true;
            })
    }

    /// Reads up to `buf.len()` bytes from the `file` into the buffer `buf`. Returns the number of
    /// bytes read (0 indicating end of file).
    ///
    /// Returns an error if:
    ///
    /// * `file` isn't borrowable or was removed
    /// * `buf.len() > u32::MAX`
    /// * an internal filesystem error occurred
    ///
    /// | Language | Equivalent function            |
    /// | :------- | :----------------------------- |
    /// | Rust     | `crankstart.file.read(...)`    |
    /// | C        | `playdate->file->read(...)`    |
    /// | Lua      | `playdate.file.file:read(...)` |
    pub fn read(&self, file: &FilePtr, buf: &mut [u8]) -> Result<usize> {
        Self::try_get_borrowable_and_not_removed_file(file)?;

        let file: *mut SDFile = file.get_pd_ptr().as_ptr();
        let len: u32 = buf.len().try_into()?;
        let buf: *mut c_void = buf.as_mut_ptr() as *mut c_void;

        self.handle_non_negative_ok_return_value(unsafe { (self.read)(file, buf, len) })
    }

    /// Writes the buffer of bytes `buf` to the file. Returns the number of bytes written.
    ///
    /// Returns an error if:
    ///
    /// * `file` isn't borrowable or was removed
    /// * `buf.len() > u32::MAX`
    /// * an internal filesystem error occurred
    ///
    /// | Language | Equivalent function             |
    /// | :------- | :------------------------------ |
    /// | Rust     | `crankstart.file.write(...)`    |
    /// | C        | `playdate->file->write(...)`    |
    /// | Lua      | `playdate.file.file:write(...)` |
    pub fn write(&self, file: &FilePtr, buf: &[u8]) -> Result<usize> {
        Self::try_get_borrowable_and_not_removed_file(file)?;

        let file: *mut SDFile = file.get_pd_ptr().as_ptr();
        let len: u32 = buf.len().try_into()?;
        let buf: *const c_void = buf.as_ptr() as *mut c_void;

        self.handle_non_negative_ok_return_value(unsafe { (self.write)(file, buf, len) })
    }

    /// Flushes the output buffer of file immediately. Returns the number of bytes written.
    ///
    /// Returns an error if:
    ///
    /// * `file` isn't borrowable or was removed
    /// * an internal filesystem error occurred
    ///
    /// | Language | Equivalent function          |
    /// | :------- | :--------------------------- |
    /// | Rust     | `crankstart.file.flush(...)` |
    /// | C        | `playdate->file->flush(...)` |
    /// | Lua      | `playdate.file.flush(...)`   |
    pub fn flush(&self, file: &FilePtr) -> Result<usize> {
        Self::try_get_borrowable_and_not_removed_file(file)?;

        let file: *mut SDFile = file.get_pd_ptr().as_ptr();

        self.handle_non_negative_ok_return_value(unsafe { (self.flush)(file) })
    }

    /// Returns the current read/write offset in the given file handle.
    ///
    /// Returns an error if:
    ///
    /// * `file` isn't borrowable or was removed
    /// * an internal filesystem error occurred
    ///
    /// | Language | Equivalent function              |
    /// | :------- | :------------------------------- |
    /// | Rust     | `crankstart.file.tell(...)`      |
    /// | C        | `playdate->file->tell(...)`      |
    /// | Lua      | `playdate.file.file:tell(...)`   |
    pub fn tell(&self, file: &FilePtr) -> Result<usize> {
        Self::try_get_borrowable_and_not_removed_file(file)?;

        let file: *mut SDFile = file.get_pd_ptr().as_ptr();

        self.handle_non_negative_ok_return_value(unsafe { (self.tell)(file) })
    }

    /// Sets the read/write offset in the given file handle to `pos`, relative to the `whence` enum:
    ///
    /// * [`SeekWhence::Set`] is relative to the beginning of the file.
    /// * [`SeekWhence::Cur`] is relative to the current position of the file pointer.
    /// * [`SeekWhence::End`] is relative to the end of the file.
    ///
    /// Returns an error if:
    ///
    /// * `file` isn't borrowable or was removed
    /// * `pos < i32::MIN || pos > i32::MAX`
    /// * an internal filesystem error occurred
    ///
    /// | Language | Equivalent function              |
    /// | :------- | :------------------------------- |
    /// | Rust     | `crankstart.file.tell(...)`      |
    /// | C        | `playdate->file->tell(...)`      |
    /// | Lua      | `playdate.file.file:tell(...)`   |
    pub fn seek(&self, file: &FilePtr, pos: isize, whence: SeekWhence) -> Result<()> {
        Self::try_get_borrowable_and_not_removed_file(file)?;

        let file: *mut SDFile = file.get_pd_ptr().as_ptr();
        let pos: i32 = pos.try_into()?;
        let whence: i32 = whence as i32;

        self.handle_zero_ok_return_value(unsafe { (self.seek)(file, pos, whence) })
    }

    /// Performs the actual work of calling the raw C function to close the file.
    ///
    /// Returns an error if:
    ///
    /// * the file was already removed
    /// * an internal filesystem error occurred
    fn close_internal(&self, pd_file: NonNull<SDFile>, file_state: &FileState) -> Result<()> {
        ensure!(!file_state.was_removed);

        self.handle_zero_ok_return_value(unsafe { (self.close)(pd_file.as_ptr()) })
    }

    /// Validates that a file is borrowable (immutably) and wasn't already removed, returning a
    /// [`Ref`] to the [`FileState`] on success.
    fn try_get_borrowable_and_not_removed_file<'f>(
        file: &'f FilePtr,
    ) -> Result<Ref<'f, FileState>> {
        let file_state: Ref<FileState> = q!(file.try_borrow_state().ok_or(()));

        ensure!(!file_state.was_removed);

        Ok(file_state)
    }

    /// Validates that a file is borrowable (mutably) and wasn't already removed, returning a
    /// [`RefMut`] to the [`FileState`] on success.
    fn try_get_borrowable_and_not_removed_file_mut<'f>(
        file: &'f FilePtr,
    ) -> Result<RefMut<'f, FileState>> {
        let file_state: RefMut<FileState> = q!(file.try_borrow_state_mut().ok_or(()));

        ensure!(!file_state.was_removed);

        Ok(file_state)
    }

    /// Handle a return value that is expected to be 0. If -1 was returned, the error message is
    /// returned.
    fn handle_zero_ok_return_value(&self, return_value: i32) -> Result<()> {
        self.handle_zero_ok_return_value_with_ok(return_value, |_| ())
    }

    /// Handle a return value that is expected to be 0, with a provided handler for the case where
    /// the return value is ok (0). If -1 was returned, the error message is returned.
    fn handle_zero_ok_return_value_with_ok<T, F: FnOnce(i32) -> T>(
        &self,
        return_value: i32,
        handle_ok_return_value: F,
    ) -> Result<T> {
        self.handle_return_value(
            return_value,
            |return_value| return_value == 0_i32,
            handle_ok_return_value,
        )
    }

    /// Handle a return value that is expected to be non-negative, returning the return value on
    /// success. If -1 was returned, the error message is returned.
    fn handle_non_negative_ok_return_value(&self, return_value: i32) -> Result<usize> {
        self.handle_return_value(
            return_value,
            |return_value| return_value >= 0_i32,
            |return_value| return_value as usize,
        )
    }

    /// Handle a return value, evaluating a predicate to determine whether or not it's `Ok`,
    /// otherwise compare it against the error sentinel value (-1), and return the error message in
    /// that case. When its neither of these cases, a general message containing the return value is
    /// returned as an error.
    fn handle_return_value<T, P: FnOnce(i32) -> bool, F: FnOnce(i32) -> T>(
        &self,
        return_value: i32,
        is_return_value_ok: P,
        handle_ok_return_value: F,
    ) -> Result<T> {
        const ERR_RETURN_VALUE: i32 = -1_i32;

        if is_return_value_ok(return_value) {
            Ok(handle_ok_return_value(return_value))
        } else if return_value == ERR_RETURN_VALUE {
            self.handle_err_return_value()
        } else {
            let mut string: TempString = TempString::new();

            write!(&mut string, "Unexpected return value [{return_value}].")?;

            Err(anyhow!(ErrorArrayString::from(string)))
        }
    }

    /// A helper for calling `get_err` and returning the `Ok`-wrapped error message in an `Err`, for
    /// use when handling an error return value from one of the C functions.
    fn handle_err_return_value<T>(&self) -> Result<T> {
        let mut error_string: TempString = TempString::new();

        self.get_err(&mut error_string)?;

        Err(anyhow!(ErrorArrayString::from(error_string)))
    }
}

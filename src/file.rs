use crate::{
    define_crankstart_api,
    pd_api::{
        ctypes::{c_char, c_void},
        playdate_file, FileOptions, FileStat, SDFile,
    },
};

define_crankstart_api! {
    #[allow(dead_code)]
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
        ) -> i32,
    }
}

use crate::{
    ctypes::{c_char, c_void},
    define_crankstart_api, playdate_file, FileOptions, FileStat, SDFile,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct FileAPI => playdate_file {
        ; // No sub-API fields
        pub geterr: unsafe extern "C" fn() -> *const c_char,
        pub listfiles: unsafe extern "C" fn(
            path: *const c_char,
            callback: Option<unsafe extern "C" fn(path: *const c_char, userdata: *mut c_void)>,
            userdata: *mut c_void,
            showhidden: i32,
        ) -> i32,
        pub stat: unsafe extern "C" fn(path: *const c_char, stat: *mut FileStat) -> i32,
        pub mkdir: unsafe extern "C" fn(path: *const c_char) -> i32,
        pub unlink: unsafe extern "C" fn(
            name: *const c_char,
            recursive: i32,
        ) -> i32,
        pub rename: unsafe extern "C" fn(
            from: *const c_char,
            to: *const c_char,
        ) -> i32,
        pub open: unsafe extern "C" fn(name: *const c_char, mode: FileOptions) -> *mut SDFile,
        pub close: unsafe extern "C" fn(file: *mut SDFile) -> i32,
        pub read: unsafe extern "C" fn(
            file: *mut SDFile,
            buf: *mut c_void,
            len: u32,
        ) -> i32,
        pub write: unsafe extern "C" fn(
            file: *mut SDFile,
            buf: *const c_void,
            len: u32,
        ) -> i32,
        pub flush: unsafe extern "C" fn(file: *mut SDFile) -> i32,
        pub tell: unsafe extern "C" fn(file: *mut SDFile) -> i32,
        pub seek: unsafe extern "C" fn(
            file: *mut SDFile,
            pos: i32,
            whence: i32,
        ) -> i32,
    }
}

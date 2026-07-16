use crate::{ctypes::c_char, define_crankstart_api, playdate_video, LCDBitmap, LCDVideoPlayer};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct VideoAPI => playdate_video {
        ; // No sub-API fields
        pub loadVideo: unsafe extern "C" fn(path: *const c_char) -> *mut LCDVideoPlayer,
        pub freePlayer: unsafe extern "C" fn(p: *mut LCDVideoPlayer),
        pub setContext:
            unsafe extern "C" fn(p: *mut LCDVideoPlayer, context: *mut LCDBitmap) -> i32,
        pub useScreenContext: unsafe extern "C" fn(p: *mut LCDVideoPlayer),
        pub renderFrame: unsafe extern "C" fn(p: *mut LCDVideoPlayer, n: i32) -> i32,
        pub getError: unsafe extern "C" fn(p: *mut LCDVideoPlayer) -> *const c_char,
        pub getInfo: unsafe extern "C" fn(
            p: *mut LCDVideoPlayer,
            outWidth: *mut i32,
            outHeight: *mut i32,
            outFrameRate: *mut f32,
            outFrameCount: *mut i32,
            outCurrentFrame: *mut i32,
        ),
        pub getContext: unsafe extern "C" fn(p: *mut LCDVideoPlayer) -> *mut LCDBitmap,
    }
}

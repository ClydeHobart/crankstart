use crate::{
    define_crankstart_api,
    pd_api::{ctypes::c_char, playdate_video, LCDBitmap, LCDVideoPlayer},
};

define_crankstart_api! {
    #[allow(dead_code, non_snake_case)]
    pub struct VideoAPI => playdate_video {
        ; // No sub-API fields
        pub(crate) loadVideo: unsafe extern "C" fn(path: *const c_char) -> *mut LCDVideoPlayer,
        pub(crate) freePlayer: unsafe extern "C" fn(p: *mut LCDVideoPlayer),
        pub(crate) setContext:
            unsafe extern "C" fn(p: *mut LCDVideoPlayer, context: *mut LCDBitmap) -> i32,
        pub(crate) useScreenContext: unsafe extern "C" fn(p: *mut LCDVideoPlayer),
        pub(crate) renderFrame: unsafe extern "C" fn(p: *mut LCDVideoPlayer, n: i32) -> i32,
        pub(crate) getError: unsafe extern "C" fn(p: *mut LCDVideoPlayer) -> *const c_char,
        pub(crate) getInfo: unsafe extern "C" fn(
            p: *mut LCDVideoPlayer,
            outWidth: *mut i32,
            outHeight: *mut i32,
            outFrameRate: *mut f32,
            outFrameCount: *mut i32,
            outCurrentFrame: *mut i32,
        ),
        pub(crate) getContext: unsafe extern "C" fn(p: *mut LCDVideoPlayer) -> *mut LCDBitmap,
    }
}

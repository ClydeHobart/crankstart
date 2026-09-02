use crate::{
    define_crankstart_api,
    pd_api::{ctypes::c_void, playdate_sound_source, sndCallbackProc, SoundSource},
};

define_crankstart_api! {
    #[allow(dead_code, non_snake_case)]
    pub struct SourceAPI => playdate_sound_source {
        ; // No sub-API fields
        pub(crate) setVolume: unsafe extern "C" fn(c: *mut SoundSource, lvol: f32, rvol: f32),
        pub(crate) getVolume:
            unsafe extern "C" fn(c: *mut SoundSource, outl: *mut f32, outr: *mut f32),
        pub(crate) isPlaying: unsafe extern "C" fn(c: *mut SoundSource) -> i32,
        pub(crate) setFinishCallback: unsafe extern "C" fn(
            c: *mut SoundSource,
            callback: sndCallbackProc,
            userdata: *mut c_void,
        ),
    }
}

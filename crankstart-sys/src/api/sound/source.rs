use crate::{
    ctypes::c_void, define_crankstart_api, playdate_sound_source, sndCallbackProc, SoundSource,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct SourceAPI => playdate_sound_source {
        ; // No sub-API fields
        pub setVolume: unsafe extern "C" fn(c: *mut SoundSource, lvol: f32, rvol: f32),
        pub getVolume: unsafe extern "C" fn(c: *mut SoundSource, outl: *mut f32, outr: *mut f32),
        pub isPlaying: unsafe extern "C" fn(c: *mut SoundSource) -> i32,
        pub setFinishCallback: unsafe extern "C" fn(
            c: *mut SoundSource,
            callback: sndCallbackProc,
            userdata: *mut c_void,
        ),
    }
}

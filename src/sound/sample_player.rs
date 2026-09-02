use crate::{
    define_crankstart_api,
    pd_api::{
        ctypes::c_void, playdate_sound_sampleplayer, sndCallbackProc, AudioSample, SamplePlayer,
    },
};

define_crankstart_api! {
    pub struct SamplePlayerAPI => playdate_sound_sampleplayer {
        ; // No sub-API fields
        pub(crate) newPlayer: unsafe extern "C" fn() -> *mut SamplePlayer,
        pub(crate) freePlayer: unsafe extern "C" fn(player: *mut SamplePlayer),
        pub(crate) setSample:
            unsafe extern "C" fn(player: *mut SamplePlayer, sample: *mut AudioSample),
        pub(crate) play: unsafe extern "C" fn(
            player: *mut SamplePlayer,
            repeat: i32,
            rate: f32,
        ) -> i32,
        pub(crate) isPlaying: unsafe extern "C" fn(player: *mut SamplePlayer) -> i32,
        pub(crate) stop: unsafe extern "C" fn(player: *mut SamplePlayer),
        pub(crate) setVolume:
            unsafe extern "C" fn(player: *mut SamplePlayer, left: f32, right: f32),
        pub(crate) getVolume:
            unsafe extern "C" fn(player: *mut SamplePlayer, left: *mut f32, right: *mut f32),
        pub(crate) getLength: unsafe extern "C" fn(player: *mut SamplePlayer) -> f32,
        pub(crate) setOffset: unsafe extern "C" fn(player: *mut SamplePlayer, offset: f32),
        pub(crate) setRate: unsafe extern "C" fn(player: *mut SamplePlayer, rate: f32),
        pub(crate) setPlayRange:
            unsafe extern "C" fn(player: *mut SamplePlayer, start: i32, end: i32),
        pub(crate) setFinishCallback: unsafe extern "C" fn(
            player: *mut SamplePlayer,
            callback: sndCallbackProc,
            userdata: *mut c_void,
        ),
        pub(crate) setLoopCallback: unsafe extern "C" fn(
            player: *mut SamplePlayer,
            callback: sndCallbackProc,
            userdata: *mut c_void,
        ),
        pub(crate) getOffset: unsafe extern "C" fn(player: *mut SamplePlayer) -> f32,
        pub(crate) getRate: unsafe extern "C" fn(player: *mut SamplePlayer) -> f32,
        pub(crate) setPaused: unsafe extern "C" fn(player: *mut SamplePlayer, flag: i32),
    }
}

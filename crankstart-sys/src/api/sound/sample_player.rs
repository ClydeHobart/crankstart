use crate::{
    ctypes::c_void, define_crankstart_api, playdate_sound_sampleplayer, sndCallbackProc,
    AudioSample, SamplePlayer,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct SamplePlayerAPI => playdate_sound_sampleplayer {
        ; // No sub-API fields
        pub newPlayer: unsafe extern "C" fn() -> *mut SamplePlayer,
        pub freePlayer: unsafe extern "C" fn(player: *mut SamplePlayer),
        pub setSample:
            unsafe extern "C" fn(player: *mut SamplePlayer, sample: *mut AudioSample),
        pub play: unsafe extern "C" fn(
            player: *mut SamplePlayer,
            repeat: i32,
            rate: f32,
        ) -> i32,
        pub isPlaying: unsafe extern "C" fn(player: *mut SamplePlayer) -> i32,
        pub stop: unsafe extern "C" fn(player: *mut SamplePlayer),
        pub setVolume: unsafe extern "C" fn(player: *mut SamplePlayer, left: f32, right: f32),
        pub getVolume:
            unsafe extern "C" fn(player: *mut SamplePlayer, left: *mut f32, right: *mut f32),
        pub getLength: unsafe extern "C" fn(player: *mut SamplePlayer) -> f32,
        pub setOffset: unsafe extern "C" fn(player: *mut SamplePlayer, offset: f32),
        pub setRate: unsafe extern "C" fn(player: *mut SamplePlayer, rate: f32),
        pub setPlayRange:
            unsafe extern "C" fn(player: *mut SamplePlayer, start: i32, end: i32),
        pub setFinishCallback: unsafe extern "C" fn(
            player: *mut SamplePlayer,
            callback: sndCallbackProc,
            userdata: *mut c_void,
        ),
        pub setLoopCallback: unsafe extern "C" fn(
            player: *mut SamplePlayer,
            callback: sndCallbackProc,
            userdata: *mut c_void,
        ),
        pub getOffset: unsafe extern "C" fn(player: *mut SamplePlayer) -> f32,
        pub getRate: unsafe extern "C" fn(player: *mut SamplePlayer) -> f32,
        pub setPaused: unsafe extern "C" fn(player: *mut SamplePlayer, flag: i32),
    }
}

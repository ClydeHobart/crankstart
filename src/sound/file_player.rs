use crate::{
    define_crankstart_api,
    pd_api::{
        ctypes::{c_char, c_void},
        playdate_sound_fileplayer, sndCallbackProc, FilePlayer,
    },
};

define_crankstart_api! {
    pub struct FilePlayerAPI => playdate_sound_fileplayer {
        ; // No sub-API fields
        pub(crate) newPlayer: unsafe extern "C" fn() -> *mut FilePlayer,
        pub(crate) freePlayer: unsafe extern "C" fn(player: *mut FilePlayer),
        pub(crate) loadIntoPlayer:
            unsafe extern "C" fn(player: *mut FilePlayer, path: *const c_char) -> i32,
        pub(crate) setBufferLength: unsafe extern "C" fn(player: *mut FilePlayer, bufferLen: f32),
        pub(crate) play: unsafe extern "C" fn(player: *mut FilePlayer, repeat: i32) -> i32,
        pub(crate) isPlaying: unsafe extern "C" fn(player: *mut FilePlayer) -> i32,
        pub(crate) pause: unsafe extern "C" fn(player: *mut FilePlayer),
        pub(crate) stop: unsafe extern "C" fn(player: *mut FilePlayer),
        pub(crate) setVolume: unsafe extern "C" fn(player: *mut FilePlayer, left: f32, right: f32),
        pub(crate) getVolume:
            unsafe extern "C" fn(player: *mut FilePlayer, left: *mut f32, right: *mut f32),
        pub(crate) getLength: unsafe extern "C" fn(player: *mut FilePlayer) -> f32,
        pub(crate) setOffset: unsafe extern "C" fn(player: *mut FilePlayer, offset: f32),
        pub(crate) setRate: unsafe extern "C" fn(player: *mut FilePlayer, rate: f32),
        pub(crate) setLoopRange:
            unsafe extern "C" fn(player: *mut FilePlayer, start: f32, end: f32),
        pub(crate) didUnderrun: unsafe extern "C" fn(player: *mut FilePlayer) -> i32,
        pub(crate) setFinishCallback: unsafe extern "C" fn(
            player: *mut FilePlayer,
            callback: sndCallbackProc,
            userdata: *mut c_void,
        ),
        pub(crate) setLoopCallback: unsafe extern "C" fn(
            player: *mut FilePlayer,
            callback: sndCallbackProc,
            userdata: *mut c_void,
        ),
        pub(crate) getOffset: unsafe extern "C" fn(player: *mut FilePlayer) -> f32,
        pub(crate) getRate: unsafe extern "C" fn(player: *mut FilePlayer) -> f32,
        pub(crate) setStopOnUnderrun: unsafe extern "C" fn(player: *mut FilePlayer, flag: i32),
        pub(crate) fadeVolume: unsafe extern "C" fn(
            player: *mut FilePlayer,
            left: f32,
            right: f32,
            len: i32,
            finishCallback: sndCallbackProc,
            userdata: *mut c_void,
        ),
        pub(crate) setMP3StreamSource: unsafe extern "C" fn(
            player: *mut FilePlayer,
            dataSource: Option<unsafe extern "C" fn(
                data: *mut u8,
                bytes: i32,
                userdata: *mut c_void,
            ) -> i32>,
            userdata: *mut c_void,
            bufferLen: f32,
        ),
    }
}

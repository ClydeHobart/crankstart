use crate::{
    ctypes::{c_char, c_void},
    define_crankstart_api, playdate_sound_fileplayer, sndCallbackProc, FilePlayer,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct FilePlayerAPI => playdate_sound_fileplayer {
        ; // No sub-API fields
        pub newPlayer: unsafe extern "C" fn() -> *mut FilePlayer,
        pub freePlayer: unsafe extern "C" fn(player: *mut FilePlayer),
        pub loadIntoPlayer:
            unsafe extern "C" fn(player: *mut FilePlayer, path: *const c_char) -> i32,
        pub setBufferLength: unsafe extern "C" fn(player: *mut FilePlayer, bufferLen: f32),
        pub play: unsafe extern "C" fn(player: *mut FilePlayer, repeat: i32) -> i32,
        pub isPlaying: unsafe extern "C" fn(player: *mut FilePlayer) -> i32,
        pub pause: unsafe extern "C" fn(player: *mut FilePlayer),
        pub stop: unsafe extern "C" fn(player: *mut FilePlayer),
        pub setVolume: unsafe extern "C" fn(player: *mut FilePlayer, left: f32, right: f32),
        pub getVolume:
            unsafe extern "C" fn(player: *mut FilePlayer, left: *mut f32, right: *mut f32),
        pub getLength: unsafe extern "C" fn(player: *mut FilePlayer) -> f32,
        pub setOffset: unsafe extern "C" fn(player: *mut FilePlayer, offset: f32),
        pub setRate: unsafe extern "C" fn(player: *mut FilePlayer, rate: f32),
        pub setLoopRange: unsafe extern "C" fn(player: *mut FilePlayer, start: f32, end: f32),
        pub didUnderrun: unsafe extern "C" fn(player: *mut FilePlayer) -> i32,
        pub setFinishCallback: unsafe extern "C" fn(
            player: *mut FilePlayer,
            callback: sndCallbackProc,
            userdata: *mut c_void,
        ),
        pub setLoopCallback: unsafe extern "C" fn(
            player: *mut FilePlayer,
            callback: sndCallbackProc,
            userdata: *mut c_void,
        ),
        pub getOffset: unsafe extern "C" fn(player: *mut FilePlayer) -> f32,
        pub getRate: unsafe extern "C" fn(player: *mut FilePlayer) -> f32,
        pub setStopOnUnderrun: unsafe extern "C" fn(player: *mut FilePlayer, flag: i32),
        pub fadeVolume: unsafe extern "C" fn(
            player: *mut FilePlayer,
            left: f32,
            right: f32,
            len: i32,
            finishCallback: sndCallbackProc,
            userdata: *mut c_void,
        ),
        pub setMP3StreamSource: unsafe extern "C" fn(
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

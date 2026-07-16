use crate::{
    define_crankstart_api, playdate_sound_effect_delayline, DelayLine, DelayLineTap,
    PDSynthSignalValue,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct DelayLineAPI => playdate_sound_effect_delayline {
        ; // No sub-API fields
        pub newDelayLine: unsafe extern "C" fn(length: i32, stereo: i32) -> *mut DelayLine,
        pub freeDelayLine: unsafe extern "C" fn(filter: *mut DelayLine),
        pub setLength: unsafe extern "C" fn(d: *mut DelayLine, frames: i32),
        pub setFeedback: unsafe extern "C" fn(d: *mut DelayLine, fb: f32),
        pub addTap: unsafe extern "C" fn(d: *mut DelayLine, delay: i32) -> *mut DelayLineTap,
        pub freeTap: unsafe extern "C" fn(tap: *mut DelayLineTap),
        pub setTapDelay: unsafe extern "C" fn(t: *mut DelayLineTap, frames: i32),
        pub setTapDelayModulator:
            unsafe extern "C" fn(t: *mut DelayLineTap, mod_: *mut PDSynthSignalValue),
        pub getTapDelayModulator:
            unsafe extern "C" fn(t: *mut DelayLineTap) -> *mut PDSynthSignalValue,
        pub setTapChannelsFlipped: unsafe extern "C" fn(t: *mut DelayLineTap, flip: i32),
    }
}

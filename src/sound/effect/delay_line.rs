use crate::{
    define_crankstart_api,
    pd_api::{playdate_sound_effect_delayline, DelayLine, DelayLineTap, PDSynthSignalValue},
};

define_crankstart_api! {
    #[allow(dead_code, non_snake_case)]
    pub struct DelayLineAPI => playdate_sound_effect_delayline {
        ; // No sub-API fields
        pub(crate) newDelayLine: unsafe extern "C" fn(length: i32, stereo: i32) -> *mut DelayLine,
        pub(crate) freeDelayLine: unsafe extern "C" fn(filter: *mut DelayLine),
        pub(crate) setLength: unsafe extern "C" fn(d: *mut DelayLine, frames: i32),
        pub(crate) setFeedback: unsafe extern "C" fn(d: *mut DelayLine, fb: f32),
        pub(crate) addTap: unsafe extern "C" fn(d: *mut DelayLine, delay: i32) -> *mut DelayLineTap,
        pub(crate) freeTap: unsafe extern "C" fn(tap: *mut DelayLineTap),
        pub(crate) setTapDelay: unsafe extern "C" fn(t: *mut DelayLineTap, frames: i32),
        pub(crate) setTapDelayModulator:
            unsafe extern "C" fn(t: *mut DelayLineTap, mod_: *mut PDSynthSignalValue),
        pub(crate) getTapDelayModulator:
            unsafe extern "C" fn(t: *mut DelayLineTap) -> *mut PDSynthSignalValue,
        pub(crate) setTapChannelsFlipped: unsafe extern "C" fn(t: *mut DelayLineTap, flip: i32),
    }
}

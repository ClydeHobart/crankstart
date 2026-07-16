use crate::{
    define_crankstart_api, playdate_sound_effect_onepolefilter, OnePoleFilter, PDSynthSignalValue,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct OnePoleFilterAPI => playdate_sound_effect_onepolefilter {
        ; // No sub-API fields
        pub newFilter: unsafe extern "C" fn() -> *mut OnePoleFilter,
        pub freeFilter: unsafe extern "C" fn(filter: *mut OnePoleFilter),
        pub setParameter: unsafe extern "C" fn(filter: *mut OnePoleFilter, parameter: f32),
        pub setParameterModulator:
            unsafe extern "C" fn(filter: *mut OnePoleFilter, signal: *mut PDSynthSignalValue),
        pub getParameterModulator:
            unsafe extern "C" fn(filter: *mut OnePoleFilter) -> *mut PDSynthSignalValue,
    }
}

use crate::{
    define_crankstart_api,
    pd_api::{playdate_sound_effect_onepolefilter, OnePoleFilter, PDSynthSignalValue},
};

define_crankstart_api! {
    #[allow(dead_code, non_snake_case)]
    pub struct OnePoleFilterAPI => playdate_sound_effect_onepolefilter {
        ; // No sub-API fields
        pub(crate) newFilter: unsafe extern "C" fn() -> *mut OnePoleFilter,
        pub(crate) freeFilter: unsafe extern "C" fn(filter: *mut OnePoleFilter),
        pub(crate) setParameter: unsafe extern "C" fn(filter: *mut OnePoleFilter, parameter: f32),
        pub(crate) setParameterModulator:
            unsafe extern "C" fn(filter: *mut OnePoleFilter, signal: *mut PDSynthSignalValue),
        pub(crate) getParameterModulator:
            unsafe extern "C" fn(filter: *mut OnePoleFilter) -> *mut PDSynthSignalValue,
    }
}

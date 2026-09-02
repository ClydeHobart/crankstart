use crate::{
    define_crankstart_api,
    pd_api::{
        playdate_sound_effect_twopolefilter, PDSynthSignalValue, TwoPoleFilter, TwoPoleFilterType,
    },
};

define_crankstart_api! {
    #[allow(dead_code, non_snake_case)]
    pub struct TwoPoleFilterAPI => playdate_sound_effect_twopolefilter {
        ; // No sub-API fields
        pub(crate) newFilter: unsafe extern "C" fn() -> *mut TwoPoleFilter,
        pub(crate) freeFilter: unsafe extern "C" fn(filter: *mut TwoPoleFilter),
        pub(crate) setType:
            unsafe extern "C" fn(filter: *mut TwoPoleFilter, type_: TwoPoleFilterType),
        pub(crate) setFrequency: unsafe extern "C" fn(filter: *mut TwoPoleFilter, frequency: f32),
        pub(crate) setFrequencyModulator:
            unsafe extern "C" fn(filter: *mut TwoPoleFilter, signal: *mut PDSynthSignalValue),
        pub(crate) getFrequencyModulator:
            unsafe extern "C" fn(filter: *mut TwoPoleFilter) -> *mut PDSynthSignalValue,
        pub(crate) setGain: unsafe extern "C" fn(filter: *mut TwoPoleFilter, gain: f32),
        pub(crate) setResonance: unsafe extern "C" fn(filter: *mut TwoPoleFilter, resonance: f32),
        pub(crate) setResonanceModulator:
            unsafe extern "C" fn(filter: *mut TwoPoleFilter, signal: *mut PDSynthSignalValue),
        pub(crate) getResonanceModulator:
            unsafe extern "C" fn(filter: *mut TwoPoleFilter) -> *mut PDSynthSignalValue,
    }
}

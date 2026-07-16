use crate::{
    define_crankstart_api, playdate_sound_effect_twopolefilter, PDSynthSignalValue, TwoPoleFilter,
    TwoPoleFilterType,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct TwoPoleFilterAPI => playdate_sound_effect_twopolefilter {
        ; // No sub-API fields
        pub newFilter: unsafe extern "C" fn() -> *mut TwoPoleFilter,
        pub freeFilter: unsafe extern "C" fn(filter: *mut TwoPoleFilter),
        pub setType: unsafe extern "C" fn(filter: *mut TwoPoleFilter, type_: TwoPoleFilterType),
        pub setFrequency: unsafe extern "C" fn(filter: *mut TwoPoleFilter, frequency: f32),
        pub setFrequencyModulator:
            unsafe extern "C" fn(filter: *mut TwoPoleFilter, signal: *mut PDSynthSignalValue),
        pub getFrequencyModulator:
            unsafe extern "C" fn(filter: *mut TwoPoleFilter) -> *mut PDSynthSignalValue,
        pub setGain: unsafe extern "C" fn(filter: *mut TwoPoleFilter, gain: f32),
        pub setResonance: unsafe extern "C" fn(filter: *mut TwoPoleFilter, resonance: f32),
        pub setResonanceModulator:
            unsafe extern "C" fn(filter: *mut TwoPoleFilter, signal: *mut PDSynthSignalValue),
        pub getResonanceModulator:
            unsafe extern "C" fn(filter: *mut TwoPoleFilter) -> *mut PDSynthSignalValue,
    }
}

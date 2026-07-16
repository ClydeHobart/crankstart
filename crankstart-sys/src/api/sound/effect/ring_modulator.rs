use crate::{
    define_crankstart_api, playdate_sound_effect_ringmodulator, PDSynthSignalValue, RingModulator,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct RingModulatorAPI => playdate_sound_effect_ringmodulator {
        ; // No sub-API fields
        pub newRingmod: unsafe extern "C" fn() -> *mut RingModulator,
        pub freeRingmod: unsafe extern "C" fn(filter: *mut RingModulator),
        pub setFrequency: unsafe extern "C" fn(filter: *mut RingModulator, frequency: f32),
        pub setFrequencyModulator:
            unsafe extern "C" fn(filter: *mut RingModulator, signal: *mut PDSynthSignalValue),
        pub getFrequencyModulator:
            unsafe extern "C" fn(filter: *mut RingModulator) -> *mut PDSynthSignalValue,
    }
}

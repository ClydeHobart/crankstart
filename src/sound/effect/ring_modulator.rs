use crate::{
    define_crankstart_api,
    pd_api::{playdate_sound_effect_ringmodulator, PDSynthSignalValue, RingModulator},
};

define_crankstart_api! {
    #[allow(dead_code, non_snake_case)]
    pub struct RingModulatorAPI => playdate_sound_effect_ringmodulator {
        ; // No sub-API fields
        pub(crate) newRingmod: unsafe extern "C" fn() -> *mut RingModulator,
        pub(crate) freeRingmod: unsafe extern "C" fn(filter: *mut RingModulator),
        pub(crate) setFrequency: unsafe extern "C" fn(filter: *mut RingModulator, frequency: f32),
        pub(crate) setFrequencyModulator:
            unsafe extern "C" fn(filter: *mut RingModulator, signal: *mut PDSynthSignalValue),
        pub(crate) getFrequencyModulator:
            unsafe extern "C" fn(filter: *mut RingModulator) -> *mut PDSynthSignalValue,
    }
}

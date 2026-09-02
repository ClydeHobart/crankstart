use crate::{
    define_crankstart_api,
    pd_api::{playdate_sound_effect_bitcrusher, BitCrusher, PDSynthSignalValue},
};

define_crankstart_api! {
    pub struct BitCrusherAPI => playdate_sound_effect_bitcrusher {
        ; // No sub-API fields
        pub(crate) newBitCrusher: unsafe extern "C" fn() -> *mut BitCrusher,
        pub(crate) freeBitCrusher: unsafe extern "C" fn(filter: *mut BitCrusher),
        pub(crate) setAmount: unsafe extern "C" fn(filter: *mut BitCrusher, amount: f32),
        pub(crate) setAmountModulator:
            unsafe extern "C" fn(filter: *mut BitCrusher, signal: *mut PDSynthSignalValue),
        pub(crate) getAmountModulator:
            unsafe extern "C" fn(filter: *mut BitCrusher) -> *mut PDSynthSignalValue,
        pub(crate) setUndersampling:
            unsafe extern "C" fn(filter: *mut BitCrusher, undersampling: f32),
        pub(crate) setUndersampleModulator:
            unsafe extern "C" fn(filter: *mut BitCrusher, signal: *mut PDSynthSignalValue),
        pub(crate) getUndersampleModulator:
            unsafe extern "C" fn(filter: *mut BitCrusher) -> *mut PDSynthSignalValue,
    }
}

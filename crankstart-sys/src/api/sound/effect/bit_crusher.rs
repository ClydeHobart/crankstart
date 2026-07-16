use crate::{
    define_crankstart_api, playdate_sound_effect_bitcrusher, BitCrusher, PDSynthSignalValue,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct BitCrusherAPI => playdate_sound_effect_bitcrusher {
        ; // No sub-API fields
        pub newBitCrusher: unsafe extern "C" fn() -> *mut BitCrusher,
        pub freeBitCrusher: unsafe extern "C" fn(filter: *mut BitCrusher),
        pub setAmount: unsafe extern "C" fn(filter: *mut BitCrusher, amount: f32),
        pub setAmountModulator:
            unsafe extern "C" fn(filter: *mut BitCrusher, signal: *mut PDSynthSignalValue),
        pub getAmountModulator:
            unsafe extern "C" fn(filter: *mut BitCrusher) -> *mut PDSynthSignalValue,
        pub setUndersampling: unsafe extern "C" fn(filter: *mut BitCrusher, undersampling: f32),
        pub setUndersampleModulator:
            unsafe extern "C" fn(filter: *mut BitCrusher, signal: *mut PDSynthSignalValue),
        pub getUndersampleModulator:
            unsafe extern "C" fn(filter: *mut BitCrusher) -> *mut PDSynthSignalValue,
    }
}

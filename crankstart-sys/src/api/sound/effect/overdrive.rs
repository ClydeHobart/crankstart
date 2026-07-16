use crate::{
    define_crankstart_api, playdate_sound_effect_overdrive, Overdrive, PDSynthSignalValue,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct OverdriveAPI => playdate_sound_effect_overdrive {
        ; // No sub-API fields
        pub newOverdrive: unsafe extern "C" fn() -> *mut Overdrive,
        pub freeOverdrive: unsafe extern "C" fn(filter: *mut Overdrive),
        pub setGain: unsafe extern "C" fn(o: *mut Overdrive, gain: f32),
        pub setLimit: unsafe extern "C" fn(o: *mut Overdrive, limit: f32),
        pub setLimitModulator:
            unsafe extern "C" fn(o: *mut Overdrive, mod_: *mut PDSynthSignalValue),
        pub getLimitModulator: unsafe extern "C" fn(o: *mut Overdrive) -> *mut PDSynthSignalValue,
        pub setOffset: unsafe extern "C" fn(o: *mut Overdrive, offset: f32),
        pub setOffsetModulator:
            unsafe extern "C" fn(o: *mut Overdrive, mod_: *mut PDSynthSignalValue),
        pub getOffsetModulator: unsafe extern "C" fn(o: *mut Overdrive) -> *mut PDSynthSignalValue,
    }
}

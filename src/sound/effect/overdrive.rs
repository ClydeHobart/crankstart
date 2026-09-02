use crate::{
    define_crankstart_api,
    pd_api::{playdate_sound_effect_overdrive, Overdrive, PDSynthSignalValue},
};

define_crankstart_api! {
    pub struct OverdriveAPI => playdate_sound_effect_overdrive {
        ; // No sub-API fields
        pub(crate) newOverdrive: unsafe extern "C" fn() -> *mut Overdrive,
        pub(crate) freeOverdrive: unsafe extern "C" fn(filter: *mut Overdrive),
        pub(crate) setGain: unsafe extern "C" fn(o: *mut Overdrive, gain: f32),
        pub(crate) setLimit: unsafe extern "C" fn(o: *mut Overdrive, limit: f32),
        pub(crate) setLimitModulator:
            unsafe extern "C" fn(o: *mut Overdrive, mod_: *mut PDSynthSignalValue),
        pub(crate) getLimitModulator:
            unsafe extern "C" fn(o: *mut Overdrive) -> *mut PDSynthSignalValue,
        pub(crate) setOffset: unsafe extern "C" fn(o: *mut Overdrive, offset: f32),
        pub(crate) setOffsetModulator:
            unsafe extern "C" fn(o: *mut Overdrive, mod_: *mut PDSynthSignalValue),
        pub(crate) getOffsetModulator:
            unsafe extern "C" fn(o: *mut Overdrive) -> *mut PDSynthSignalValue,
    }
}

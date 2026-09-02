use crate::{
    define_crankstart_api,
    pd_api::{ctypes::c_void, playdate_sound_lfo, LFOType, PDSynthLFO},
};

define_crankstart_api! {
    pub struct LFOAPI => playdate_sound_lfo {
        ; // No sub-API fields
        pub(crate) newLFO: unsafe extern "C" fn(type_: LFOType) -> *mut PDSynthLFO,
        pub(crate) freeLFO: unsafe extern "C" fn(lfo: *mut PDSynthLFO),
        pub(crate) setType: unsafe extern "C" fn(lfo: *mut PDSynthLFO, type_: LFOType),
        pub(crate) setRate: unsafe extern "C" fn(lfo: *mut PDSynthLFO, rate: f32),
        pub(crate) setPhase: unsafe extern "C" fn(lfo: *mut PDSynthLFO, phase: f32),
        pub(crate) setCenter: unsafe extern "C" fn(lfo: *mut PDSynthLFO, center: f32),
        pub(crate) setDepth: unsafe extern "C" fn(lfo: *mut PDSynthLFO, depth: f32),
        pub(crate) setArpeggiation:
            unsafe extern "C" fn(lfo: *mut PDSynthLFO, nSteps: i32, steps: *mut f32),
        pub(crate) setFunction: unsafe extern "C" fn(
            lfo: *mut PDSynthLFO,
            lfoFunc:
                Option<unsafe extern "C" fn(lfo: *mut PDSynthLFO, userdata: *mut c_void) -> f32>,
            userdata: *mut c_void,
            interpolate: i32,
        ),
        pub(crate) setDelay:
            unsafe extern "C" fn(lfo: *mut PDSynthLFO, holdoff: f32, ramptime: f32),
        pub(crate) setRetrigger: unsafe extern "C" fn(lfo: *mut PDSynthLFO, flag: i32),
        pub(crate) getValue: unsafe extern "C" fn(lfo: *mut PDSynthLFO) -> f32,
        pub(crate) setGlobal: unsafe extern "C" fn(lfo: *mut PDSynthLFO, global: i32),
        pub(crate) setStartPhase: unsafe extern "C" fn(lfo: *mut PDSynthLFO, phase: f32),
    }
}

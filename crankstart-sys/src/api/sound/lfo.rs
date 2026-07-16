use crate::{ctypes::c_void, define_crankstart_api, playdate_sound_lfo, LFOType, PDSynthLFO};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct LFOAPI => playdate_sound_lfo {
        ; // No sub-API fields
        pub newLFO: unsafe extern "C" fn(type_: LFOType) -> *mut PDSynthLFO,
        pub freeLFO: unsafe extern "C" fn(lfo: *mut PDSynthLFO),
        pub setType: unsafe extern "C" fn(lfo: *mut PDSynthLFO, type_: LFOType),
        pub setRate: unsafe extern "C" fn(lfo: *mut PDSynthLFO, rate: f32),
        pub setPhase: unsafe extern "C" fn(lfo: *mut PDSynthLFO, phase: f32),
        pub setCenter: unsafe extern "C" fn(lfo: *mut PDSynthLFO, center: f32),
        pub setDepth: unsafe extern "C" fn(lfo: *mut PDSynthLFO, depth: f32),
        pub setArpeggiation:
            unsafe extern "C" fn(lfo: *mut PDSynthLFO, nSteps: i32, steps: *mut f32),
        pub setFunction: unsafe extern "C" fn(
            lfo: *mut PDSynthLFO,
            lfoFunc:
                Option<unsafe extern "C" fn(lfo: *mut PDSynthLFO, userdata: *mut c_void) -> f32>,
            userdata: *mut c_void,
            interpolate: i32,
        ),
        pub setDelay: unsafe extern "C" fn(lfo: *mut PDSynthLFO, holdoff: f32, ramptime: f32),
        pub setRetrigger: unsafe extern "C" fn(lfo: *mut PDSynthLFO, flag: i32),
        pub getValue: unsafe extern "C" fn(lfo: *mut PDSynthLFO) -> f32,
        pub setGlobal: unsafe extern "C" fn(lfo: *mut PDSynthLFO, global: i32),
        pub setStartPhase: unsafe extern "C" fn(lfo: *mut PDSynthLFO, phase: f32),
    }
}

use crate::{
    define_crankstart_api,
    pd_api::{
        ctypes::c_void, playdate_sound_signal, signalDeallocFunc, signalNoteOffFunc,
        signalNoteOnFunc, signalStepFunc, PDSynthSignal,
    },
};

define_crankstart_api! {
    #[allow(dead_code, non_snake_case)]
    pub struct SignalAPI => playdate_sound_signal {
        ; // No sub-API fields
        pub(crate) newSignal: unsafe extern "C" fn(
            step: signalStepFunc,
            noteOn: signalNoteOnFunc,
            noteOff: signalNoteOffFunc,
            dealloc: signalDeallocFunc,
            userdata: *mut c_void,
        ) -> *mut PDSynthSignal,
        pub(crate) freeSignal: unsafe extern "C" fn(signal: *mut PDSynthSignal),
        pub(crate) getValue: unsafe extern "C" fn(signal: *mut PDSynthSignal) -> f32,
        pub(crate) setValueScale: unsafe extern "C" fn(signal: *mut PDSynthSignal, scale: f32),
        pub(crate) setValueOffset: unsafe extern "C" fn(signal: *mut PDSynthSignal, offset: f32),
    }
}

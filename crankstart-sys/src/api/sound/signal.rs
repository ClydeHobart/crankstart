use crate::{
    ctypes::c_void, define_crankstart_api, playdate_sound_signal, signalDeallocFunc,
    signalNoteOffFunc, signalNoteOnFunc, signalStepFunc, PDSynthSignal,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct SignalAPI => playdate_sound_signal {
        ; // No sub-API fields
        pub newSignal: unsafe extern "C" fn(
            step: signalStepFunc,
            noteOn: signalNoteOnFunc,
            noteOff: signalNoteOffFunc,
            dealloc: signalDeallocFunc,
            userdata: *mut c_void,
        ) -> *mut PDSynthSignal,
        pub freeSignal: unsafe extern "C" fn(signal: *mut PDSynthSignal),
        pub getValue: unsafe extern "C" fn(signal: *mut PDSynthSignal) -> f32,
        pub setValueScale: unsafe extern "C" fn(signal: *mut PDSynthSignal, scale: f32),
        pub setValueOffset: unsafe extern "C" fn(signal: *mut PDSynthSignal, offset: f32),
    }
}

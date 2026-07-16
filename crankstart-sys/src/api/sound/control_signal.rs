use crate::{define_crankstart_api, playdate_control_signal, ControlSignal};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct ControlSignalAPI => playdate_control_signal{
        ; // No sub-API fields
        pub newSignal: unsafe extern "C" fn() -> *mut ControlSignal,
        pub freeSignal: unsafe extern "C" fn(signal: *mut ControlSignal),
        pub clearEvents: unsafe extern "C" fn(control: *mut ControlSignal),
        pub addEvent: unsafe extern "C" fn(
            control: *mut ControlSignal,
            step: i32,
            value: f32,
            interpolate: i32,
        ),
        pub removeEvent: unsafe extern "C" fn(control: *mut ControlSignal, step: i32),
        pub getMIDIControllerNumber: unsafe extern "C" fn(control: *mut ControlSignal) -> i32,
    }
}

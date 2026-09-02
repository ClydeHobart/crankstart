use crate::{
    define_crankstart_api,
    pd_api::{playdate_control_signal, ControlSignal},
};

define_crankstart_api! {
    #[allow(dead_code, non_snake_case)]
    pub struct ControlSignalAPI => playdate_control_signal{
        ; // No sub-API fields
        pub(crate) newSignal: unsafe extern "C" fn() -> *mut ControlSignal,
        pub(crate) freeSignal: unsafe extern "C" fn(signal: *mut ControlSignal),
        pub(crate) clearEvents: unsafe extern "C" fn(control: *mut ControlSignal),
        pub(crate) addEvent: unsafe extern "C" fn(
            control: *mut ControlSignal,
            step: i32,
            value: f32,
            interpolate: i32,
        ),
        pub(crate) removeEvent: unsafe extern "C" fn(control: *mut ControlSignal, step: i32),
        pub(crate) getMIDIControllerNumber:
            unsafe extern "C" fn(control: *mut ControlSignal) -> i32,
    }
}

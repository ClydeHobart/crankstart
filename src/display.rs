use crate::{define_crankstart_api, pd_api::playdate_display};

define_crankstart_api! {
    pub struct DisplayAPI => playdate_display {
        ; // No sub-API fields
        pub(crate) getWidth: unsafe extern "C" fn() -> i32,
        pub(crate) getHeight: unsafe extern "C" fn() -> i32,
        pub(crate) setRefreshRate: unsafe extern "C" fn(rate: f32),
        pub(crate) setInverted: unsafe extern "C" fn(flag: i32),
        pub(crate) setScale: unsafe extern "C" fn(s: u32),
        pub(crate) setMosaic: unsafe extern "C" fn(x: u32, y: u32),
        pub(crate) setFlipped: unsafe extern "C" fn(x: i32, y: i32),
        pub(crate) setOffset: unsafe extern "C" fn(x: i32, y: i32),
    }
}

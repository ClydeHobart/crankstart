use crate::{define_crankstart_api, playdate_display};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct DisplayAPI => playdate_display {
        ; // No sub-API fields
        pub getWidth: unsafe extern "C" fn() -> i32,
        pub getHeight: unsafe extern "C" fn() -> i32,
        pub setRefreshRate: unsafe extern "C" fn(rate: f32),
        pub setInverted: unsafe extern "C" fn(flag: i32),
        pub setScale: unsafe extern "C" fn(s: u32),
        pub setMosaic: unsafe extern "C" fn(x: u32, y: u32),
        pub setFlipped: unsafe extern "C" fn(x: i32, y: i32),
        pub setOffset: unsafe extern "C" fn(x: i32, y: i32),
    }
}

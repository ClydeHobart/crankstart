#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unnecessary_transmutes)]
#![allow(unpredictable_function_pointer_comparisons)]

use {
    crate::{CrankstartAPI, util::singleton::Singleton},
    core::cell::Ref,
    euclid::{default::Rect, rect},
};

#[cfg(all(target_os = "macos", any(target_arch = "x86", target_arch = "x86_64")))]
pub mod ctypes {
    pub type c_ulong = u64;
    pub type c_int = i32;
    pub type c_char = i8;
    pub type c_uint = u32;
    pub type c_void = core::ffi::c_void;
    pub type realloc_size = u64;
}

#[cfg(all(target_os = "macos", any(target_arch = "aarch64", target_arch = "arm")))]
pub mod ctypes {
    pub type c_ulong = u64;
    pub type c_int = i32;
    pub type c_char = u8;
    pub type c_uint = u32;
    pub type c_void = core::ffi::c_void;
    pub type realloc_size = u64;
}

#[cfg(all(
    not(target_os = "macos"),
    any(target_arch = "x86", target_arch = "x86_64")
))]
pub mod ctypes {
    pub type c_ulong = u64;
    pub type c_int = i32;
    pub type c_char = i8;
    pub type c_uchar = u8;
    pub type c_uint = u32;
    pub type c_ushort = u16;
    pub type c_short = i16;
    pub type c_void = core::ffi::c_void;
    pub type realloc_size = u32;
}

#[cfg(all(
    not(target_os = "macos"),
    any(target_arch = "aarch64", target_arch = "arm")
))]
pub mod ctypes {
    pub type c_ulong = u64;
    pub type c_int = i32;
    pub type c_char = u8;
    pub type c_uchar = u8;
    pub type c_uint = u32;
    pub type c_ushort = u16;
    pub type c_short = i16;
    pub type c_void = core::ffi::c_void;
    pub type realloc_size = u32;
}

#[cfg(all(target_os = "windows", target_feature = "crt-static"))]
#[link(name = "libcmt")]
unsafe extern "C" {}
#[cfg(all(target_os = "windows", not(target_feature = "crt-static")))]
#[link(name = "msvcrt")]
unsafe extern "C" {}

#[cfg(all(
    not(target_os = "none"),
    any(target_arch = "x86", target_arch = "x86_64")
))]
include!("pd_api/bindings_x86.rs");
#[cfg(all(
    not(target_os = "none"),
    any(target_arch = "aarch64", target_arch = "arm")
))]
include!("pd_api/bindings_aarch64.rs");
#[cfg(target_os = "none")]
include!("pd_api/bindings_playdate.rs");

impl From<Rect<i32>> for LCDRect {
    fn from(r: Rect<i32>) -> Self {
        LCDRect {
            top: r.max_y(),
            bottom: r.min_y(),
            left: r.min_x(),
            right: r.max_x(),
        }
    }
}

impl From<LCDRect> for Rect<i32> {
    fn from(r: LCDRect) -> Self {
        rect(r.left, r.top, r.right - r.left, r.bottom - r.top)
    }
}

impl PDDateTime {
    fn fix_weekday(&mut self) -> u8 {
        let crankstart_api: Ref<CrankstartAPI> = CrankstartAPI::get();

        *self = crankstart_api.system.convert_duration_to_date_time(
            crankstart_api.system.convert_date_time_to_duration(*self),
        );

        self.weekday
    }
}

impl From<FileStat> for PDDateTime {
    fn from(value: FileStat) -> Self {
        let year: u16 = (value.m_year & u16::MAX as i32) as u16;
        let month: u8 = (value.m_month & u8::MAX as i32) as u8;
        let day: u8 = (value.m_day & u8::MAX as i32) as u8;
        let weekday: u8 = 0_u8;
        let hour: u8 = (value.m_hour & u8::MAX as i32) as u8;
        let minute: u8 = (value.m_minute & u8::MAX as i32) as u8;
        let second: u8 = (value.m_second & u8::MAX as i32) as u8;

        let mut pd_date_time: Self = Self {
            year,
            month,
            day,
            weekday,
            hour,
            minute,
            second,
        };

        // `FileStat` doesn't have a weekday field, but my best guess is the time since epoch just
        // uses YYYY, MM, DD, HH, MM, and SS, so the weekday is redundant information. Send it
        // through a rinse cycle that'll put the correct value in there.
        pd_date_time.fix_weekday();

        pd_date_time
    }
}

impl From<Rect<f32>> for PDRect {
    fn from(r: Rect<f32>) -> Self {
        PDRect {
            x: r.origin.x,
            y: r.origin.y,
            width: r.size.width,
            height: r.size.height,
        }
    }
}

impl From<PDRect> for Rect<f32> {
    fn from(r: PDRect) -> Self {
        rect(r.x, r.y, r.width, r.height)
    }
}

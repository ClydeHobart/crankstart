#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unnecessary_transmutes)]
#![allow(unpredictable_function_pointer_comparisons)]

use euclid::{default::Rect, rect};

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

impl Default for PDButtons {
    fn default() -> Self {
        Self(Default::default())
    }
}

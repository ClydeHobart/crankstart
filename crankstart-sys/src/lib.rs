#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unnecessary_transmutes)]
#![allow(unpredictable_function_pointer_comparisons)]

pub mod api;

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
extern "C" {}
#[cfg(all(target_os = "windows", not(target_feature = "crt-static")))]
#[link(name = "msvcrt")]
extern "C" {}

#[cfg(all(
    not(target_os = "none"),
    any(target_arch = "x86", target_arch = "x86_64")
))]
include!("bindings_x86.rs");
#[cfg(all(
    not(target_os = "none"),
    any(target_arch = "aarch64", target_arch = "arm")
))]
include!("bindings_aarch64.rs");
#[cfg(target_os = "none")]
include!("bindings_playdate.rs");

#[cfg(feature = "euclid")]
impl From<euclid::default::Rect<i32>> for LCDRect {
    fn from(r: euclid::default::Rect<i32>) -> Self {
        LCDRect {
            top: r.max_y(),
            bottom: r.min_y(),
            left: r.min_x(),
            right: r.max_x(),
        }
    }
}

#[cfg(feature = "euclid")]
impl From<LCDRect> for euclid::default::Rect<i32> {
    fn from(r: LCDRect) -> Self {
        euclid::rect(r.left, r.top, r.right - r.left, r.bottom - r.top)
    }
}

#[cfg(feature = "euclid")]
impl From<euclid::default::Rect<f32>> for PDRect {
    fn from(r: euclid::default::Rect<f32>) -> Self {
        PDRect {
            x: r.origin.x,
            y: r.origin.y,
            width: r.size.width,
            height: r.size.height,
        }
    }
}

#[cfg(feature = "euclid")]
impl From<PDRect> for euclid::default::Rect<f32> {
    fn from(r: PDRect) -> Self {
        euclid::rect(r.x, r.y, r.width, r.height)
    }
}

trait APITrait {
    const SUB_API_COUNT: usize;
    const FN_COUNT: usize;
}

#[macro_export]
macro_rules! define_crankstart_api {
    {
        $(#[$struct_attr:meta])*
        $struct_pub:vis struct $cs_api_ty:ident => $pd_api_ty:ty {
            $(
                $(#[$api_field_attr:meta])*
                $api_pub:vis $api_field:ident: $cs_sub_api_ty:ty
            ),*;
            $(
                $(#[$fn_field_attr:meta])*
                $fn_pub:vis $fn_field:ident: $fn_ty:ty
            ),* $(,)?
        }
    } => {
        $(#[$struct_attr])*
        $struct_pub struct $cs_api_ty {
            $(
                $(#[$api_field_attr])*
                $api_pub $api_field: $cs_sub_api_ty,
            )*
            $(
                $(#[$fn_field_attr])*
                $fn_pub $fn_field: $fn_ty,
            )*
        }

        impl $crate::APITrait for $cs_api_ty {
            const SUB_API_COUNT: usize = {
                #[allow(unused_imports)]
                use ::core::mem::size_of;

                #[allow(unused_mut)]
                let mut sub_api_count: usize = 0_usize;

                $(
                    // We just need something to specify which repeating symbol list we want to
                    // expand.
                    sub_api_count +=
                        (size_of::<$cs_sub_api_ty>() == size_of::<$cs_sub_api_ty>()) as usize;
                )*

                sub_api_count
            };
            const FN_COUNT: usize = {
                #[allow(unused_imports)]
                use ::core::mem::size_of;

                #[allow(unused_mut)]
                let mut fn_count: usize = 0_usize;

                $(
                    // We just need something that'll instruct which repeating symbol we're trying
                    // to use.
                    fn_count += (size_of::<$fn_ty>() == size_of::<fn()>()) as usize;
                )*

                fn_count
            };
        }

        ::static_assertions::const_assert!(
            (
                <$cs_api_ty as $crate::APITrait>::SUB_API_COUNT
                *
                ::core::mem::size_of::<*const ()>()
            ) + (
                <$cs_api_ty as $crate::APITrait>::FN_COUNT
                *
                ::core::mem::size_of::<Option<fn()>>()
            ) == ::core::mem::size_of::<$pd_api_ty>()
        );

        impl ::core::convert::TryFrom<&$pd_api_ty> for $cs_api_ty {
            type Error = &'static str;

            #[allow(non_snake_case)]
            fn try_from(pd_api: &$pd_api_ty) -> Result<Self, Self::Error> {
                $(
                    let $api_field = {
                        use core::convert::TryInto;

                        let pd_api_field_ptr = pd_api.$api_field;
                        let pd_api_field_ref = unsafe { pd_api_field_ptr.as_ref() }
                            .ok_or(concat!(
                                stringify!($pd_api_ty),
                                "::",
                                stringify!($api_field),
                                " was null"
                            ))?;
                        let cs_api_field = pd_api_field_ref.try_into()?;

                        cs_api_field
                    };
                )*

                $(
                    let $fn_field = pd_api.$fn_field.ok_or(concat!(
                        stringify!($pd_api_ty),
                        "::",
                        stringify!($fn_field),
                        " was null"
                    ))?;
                )*

                Ok(Self {
                    $($api_field,)*
                    $($fn_field,)*
                })
            }
        }
    };
}

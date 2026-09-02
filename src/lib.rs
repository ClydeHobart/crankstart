#![cfg_attr(not(any(test, doctest)), no_std)]
#![cfg_attr(not(any(test, doctest)), feature(alloc_error_handler))]
#![feature(core_intrinsics)]
#![allow(internal_features)]
#![allow(unused_variables, dead_code, unused_imports)]

#[cfg(not(any(test, doctest)))]
pub extern crate alloc;

#[cfg(any(test, doctest))]
pub use std as alloc;

use {
    self::{
        display::DisplayAPI,
        file::FileAPI,
        graphics::GraphicsAPI,
        json::JSONAPI,
        lua::LuaAPI,
        pd_api::{ctypes::c_void, PDSystemEvent, PlaydateAPI},
        scoreboards::ScoreboardsAPI,
        sound::SoundAPI,
        sprite::SpriteAPI,
        sys::SysAPI,
        util::{singleton::Singleton, string::TempString},
    },
    anyhow::{Error, Result},
    core::{
        cell::{Ref, RefCell, RefMut},
        convert::TryFrom,
        mem::MaybeUninit,
    },
};

#[cfg(not(any(test, doctest)))]
use core::{
    alloc::{GlobalAlloc, Layout},
    panic::PanicInfo,
};

pub mod display;
pub mod file;
pub mod graphics;
pub mod json;
pub mod lua;
pub mod pd_api;
pub mod scoreboards;
pub mod sound;
pub mod sprite;
pub mod sys;
pub mod util;

define_crankstart_api! {
    #[allow(dead_code)]
    pub struct CrankstartAPI => PlaydateAPI {
        pub display: DisplayAPI,
        pub file: FileAPI,
        pub graphics: GraphicsAPI,
        pub json: JSONAPI,
        pub lua: LuaAPI,
        pub scoreboards: ScoreboardsAPI,
        pub sound: SoundAPI,
        pub sprite: SpriteAPI,
        pub system: SysAPI;
        // No fn fields.
    }
}

impl CrankstartAPI {}

impl_singleton!(CrankstartAPI);

pub struct ShouldUpdateDisplay(bool);

impl From<bool> for ShouldUpdateDisplay {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

pub trait Game: Singleton {
    fn new() -> Result<Self>;

    fn update(&mut self) -> Result<ShouldUpdateDisplay>;

    fn handle_event(&mut self, event: PDSystemEvent, arg: u32);
}

pub fn game_handle_event<G: Game>(playdate: *mut PlaydateAPI, event: PDSystemEvent, arg: u32) {
    if event == PDSystemEvent::kEventInit {
        if let Err(error) = game_init::<G>(playdate) {
            // This might fail, if the `CrankstartAPI` failed to initialize.
            eprintln!("game_init failed: {error:?}");
        }
    }

    if let Some(mut game) = G::try_get_mut() {
        game.handle_event(event, arg);
    }
}

fn game_init<G: Game>(playdate_api: *mut PlaydateAPI) -> Result<()> {
    let playdate_api: &PlaydateAPI = q!(unsafe { playdate_api.as_ref() }.ok_or(()));

    CrankstartAPI::set(CrankstartAPI::try_from(playdate_api).map_err(Error::msg)?);

    G::set(G::new()?);

    CrankstartAPI::get()
        .system
        .set_update_callback::<G>(Some(game_update::<G>));

    Ok(())
}

extern "C" fn game_update<G: Game>(user_data: *mut c_void) -> i32 {
    game_update_internal::<G>(user_data).map_or_else(
        |error| {
            eprintln!("game_update_internal failed: {error:?}");

            0_i32
        },
        |should_update_display| should_update_display.0 as i32,
    )
}

fn game_update_internal<G: Game>(user_data: *mut c_void) -> Result<ShouldUpdateDisplay> {
    let game_mut_ptr: *mut G = user_data.cast();

    ensure!(game_mut_ptr.is_aligned());

    let game: &mut G = q!(unsafe { game_mut_ptr.as_mut().ok_or(()) });

    game.update()
}

#[macro_export]
macro_rules! crankstart_game {
    ($game_struct:ty) => {
        $crate::impl_singleton!($game_struct);

        #[no_mangle]
        extern "C" fn eventHandler(
            playdate: *mut $crate::pd_api::PlaydateAPI,
            event: $crate::pd_api::PDSystemEvent,
            arg: u32,
        ) -> i32 {
            $crate::game_handle_event::<$game_struct>(playdate, event, arg);

            0
        }
    };
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
            $(
                ;

                $(#[$data_field_attr:meta])*
                $data_pub:vis $data_field:ident: $data_ty:ty,
            )?
        }
    } => {
        $(#[$struct_attr])*
        #[allow(dead_code, non_snake_case)]
        $struct_pub struct $cs_api_ty {
            $(
                $(#[$api_field_attr])*
                $api_pub $api_field: $cs_sub_api_ty,
            )*
            $(
                $(#[$fn_field_attr])*
                $fn_pub $fn_field: $fn_ty,
            )*
            $(
                $(#[$data_field_attr])*
                $data_pub $data_field: $data_ty,
            )?
        }

        #[allow(non_snake_case)]
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

                $(
                    let $data_field: $data_ty = Default::default();
                )?

                Ok(Self {
                    $($api_field,)*
                    $($fn_field,)*
                    $($data_field,)?
                })
            }
        }
    };
}

#[macro_export]
macro_rules! str_lit {
    ($str_lit:literal) => {
        #[cfg(debug_assertions)]
        {
            $str_lit
        }

        #[cfg(not(debug_assertions))]
        {
            ""
        }
    };
}

#[macro_export]
macro_rules! ensure {
    ($expr:expr) => {
        if $expr {
            Ok(())
        } else {
            Err(::anyhow::Error::msg(
                #[cfg(debug_assertions)]
                ::core::concat!(
                    ::core::file!(),
                    ":",
                    ::core::line!(),
                    ": \"",
                    ::core::stringify!($expr),
                    "\" was false"
                ),
                #[cfg(not(debug_assertions))]
                "",
            ))
        }?
    };
}

#[macro_export]
macro_rules! q {
    ($expr:expr) => {
        ($expr).map_err(|_| {
            ::anyhow::Error::msg(
                #[cfg(debug_assertions)]
                ::core::concat!(
                    ::core::file!(),
                    ":",
                    ::core::line!(),
                    ": \"",
                    ::core::stringify!($expr),
                    "\" was an error"
                ),
                #[cfg(not(debug_assertions))]
                "",
            )
        })?
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::sys::SysAPI::print_internal(
            |temp_string| {
                $crate::write0!(temp_string, $($arg)*).ok();
            },
            $crate::sys::SysAPI::log_to_console
        );
    }
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        $crate::sys::SysAPI::print_internal(
            |temp_string| {
                $crate::write0!(temp_string, $($arg)*).ok();
            },
            $crate::sys::SysAPI::error
        );
    }
}

#[macro_export]
macro_rules! breakpoint_nop {
    () => {
        #[cfg(debug_assertions)]
        {
            ::core::hint::black_box(());
            ::core::intrinsics::breakpoint();
        }
    };
}

#[cfg(not(any(test, doctest)))]
fn abort_with_addr(addr: usize) -> ! {
    let p = addr as *mut i32;
    unsafe {
        *p = 0;
    }
    core::intrinsics::abort()
}

#[cfg(not(any(test, doctest)))]
#[panic_handler]
fn panic(#[allow(unused)] panic_info: &PanicInfo) -> ! {
    if let Some(location) = panic_info.location() {
        eprintln!(
            "panic: {} @ {}:{}",
            panic_info.message(),
            location.file(),
            location.line(),
        );
    } else {
        eprintln!("panic");
    }

    #[cfg(target_os = "macos")]
    unsafe {
        core::intrinsics::breakpoint();
    }

    abort_with_addr(0xdeadbeef);
}

#[cfg(not(any(test, doctest)))]
pub(crate) struct PlaydateAllocator;

#[cfg(not(any(test, doctest)))]
unsafe impl Sync for PlaydateAllocator {}

#[cfg(not(any(test, doctest)))]
unsafe impl GlobalAlloc for PlaydateAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        (CrankstartAPI::get().system.realloc)(core::ptr::null_mut(), layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        (CrankstartAPI::get().system.realloc)(ptr as *mut core::ffi::c_void, 0);
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        (CrankstartAPI::get().system.realloc)(ptr as *mut core::ffi::c_void, new_size) as *mut u8
    }
}

#[cfg(not(any(test, doctest)))]
#[global_allocator]
pub(crate) static mut A: PlaydateAllocator = PlaydateAllocator;

// define what happens in an Out Of Memory (OOM) condition

#[cfg(not(any(test, doctest)))]
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    eprintln!("Out of Memory");
    abort_with_addr(0xDEADFA11);
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.add(i) = *src.add(i);
        i += 1;
    }
    dest
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        // copy from end
        let mut i = n;
        while i != 0 {
            i -= 1;
            *dest.add(i) = *src.add(i);
        }
    } else {
        // copy from beginning
        let mut i = 0;
        while i < n {
            *dest.add(i) = *src.add(i);
            i += 1;
        }
    }
    dest
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.add(i);
        let b = *s2.add(i);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn bcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    memcmp(s1, s2, n)
}

#[cfg(target_os = "macos")]
pub unsafe fn memset_internal(s: *mut u8, c: crankstart_sys::ctypes::c_int, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.add(i) = c as u8;
        i += 1;
    }
    s
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: crankstart_sys::ctypes::c_int, n: usize) -> *mut u8 {
    memset_internal(s, c, n)
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn __bzero(s: *mut u8, n: usize) {
    memset_internal(s, 0, n);
}

#[no_mangle]
pub extern "C" fn _sbrk() {}

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub extern "C" fn _write() {}

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub extern "C" fn _close() {}

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub extern "C" fn _lseek() {}

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub extern "C" fn _read() {}

#[no_mangle]
pub extern "C" fn _fstat() {}

#[no_mangle]
pub extern "C" fn _isatty() {}

#[cfg(not(target_os = "windows"))]
#[no_mangle]
pub extern "C" fn _exit() {}

#[no_mangle]
pub extern "C" fn _open() {}

#[no_mangle]
pub extern "C" fn _kill() {}

#[no_mangle]
pub extern "C" fn _getpid() {}

#[cfg(not(any(test, doctest)))]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {
    unimplemented!();
}

#[cfg(target_os = "macos")]
#[no_mangle]
extern "C" fn _Unwind_Resume() {
    unimplemented!();
}

#[no_mangle]
extern "C" fn __exidx_start() {
    unimplemented!();
}

#[no_mangle]
extern "C" fn __exidx_end() {
    unimplemented!();
}

#[cfg(target_os = "macos")]
#[link(name = "System")]
extern "C" {}

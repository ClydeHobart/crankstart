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
        pd_api::{PDSystemEvent, PlaydateAPI, ctypes::c_void},
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

    fn handle_event(&mut self, event: PDSystemEvent, arg: u32) {}
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

        #[unsafe(no_mangle)]
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

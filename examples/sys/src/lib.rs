#![no_std]

use {
    anyhow::Result,
    core::{cell::Ref, fmt::Debug, mem::transmute},
    crankstart::{
        CrankstartAPI, Game, ShouldUpdateDisplay,
        alloc::{boxed::Box, vec::Vec},
        crankstart_game,
        pd_api::PDButtons,
        println,
        sys::MenuItemPtr,
        util::{callback::Callback, singleton::Singleton, string::SmallTempString},
        write0,
    },
};

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u8)]
enum SysGameState {
    AddDefaultMenuItem,
    AddCheckmarkMenuItem,
    AddOptionsMenuItem,
}

const SYS_GAME_STATE_COUNT: u8 = 3_u8;

impl SysGameState {
    fn get_short_name(self) -> &'static str {
        match self {
            SysGameState::AddDefaultMenuItem => "Default",
            SysGameState::AddCheckmarkMenuItem => "Checkmark",
            SysGameState::AddOptionsMenuItem => "Options",
        }
    }

    fn next(self) -> Self {
        unsafe { transmute::<u8, Self>((self as u8 + 1_u8) % SYS_GAME_STATE_COUNT) }
    }
}

#[derive(Default)]
struct SysGame {
    state: Option<SysGameState>,
    menu_items: Vec<MenuItemPtr>,
}

impl SysGame {
    fn build_title(&self, menu_item_index: usize) -> SmallTempString {
        let short_name: &str = self.state.unwrap().get_short_name();
        let mut title: SmallTempString = SmallTempString::new();

        write0!(&mut title, "{short_name} {menu_item_index}").unwrap();

        title
    }

    fn on_state_change(&mut self) {
        let crankstart_api: Ref<CrankstartAPI> = CrankstartAPI::get();

        crankstart_api.system.remove_all_menu_items();
        self.menu_items.clear();

        match self.state.clone() {
            Some(SysGameState::AddDefaultMenuItem) => self.add_default_menu_item(&crankstart_api),
            Some(SysGameState::AddCheckmarkMenuItem) => {
                self.add_checkmark_menu_item(&crankstart_api)
            }
            Some(SysGameState::AddOptionsMenuItem) => self.add_options_menu_item(&crankstart_api),
            None => (),
        }
    }

    fn add_default_menu_item(&mut self, crankstart_api: &CrankstartAPI) {
        for menu_item_index in 0_usize..3_usize {
            let title: SmallTempString = self.build_title(menu_item_index);

            self.menu_items.push(
                crankstart_api
                    .system
                    .add_default_menu_item(title.as_str(), Self::build_callback(&title))
                    .unwrap(),
            );
        }
    }

    fn add_checkmark_menu_item(&mut self, crankstart_api: &CrankstartAPI) {
        for menu_item_index in 0_usize..3_usize {
            let title: SmallTempString = self.build_title(menu_item_index);

            self.menu_items.push(
                crankstart_api
                    .system
                    .add_checkmark_menu_item(
                        title.as_str(),
                        menu_item_index % 2_usize == 1_usize,
                        Self::build_callback(&title),
                    )
                    .unwrap(),
            );
        }
    }

    fn add_options_menu_item(&mut self, crankstart_api: &CrankstartAPI) {
        for menu_item_index in 0_usize..3_usize {
            let title: SmallTempString = self.build_title(menu_item_index);

            self.menu_items.push(
                crankstart_api
                    .system
                    .add_options_menu_item(
                        title.as_str(),
                        &["Option 0", "Option 1", "Option 2"],
                        Self::build_callback(&title),
                    )
                    .unwrap(),
            );
        }
    }

    fn build_callback<I: Debug>(title: &SmallTempString) -> Callback<I> {
        let title: SmallTempString = title.clone();

        Callback::from_closure(Box::new(move |input: I| {
            println!("{title} selected: {input:?}");
        }))
    }
}

impl Game for SysGame {
    fn new() -> Result<Self> {
        let mut sys_game: Self = Default::default();

        sys_game.on_state_change();

        Ok(Default::default())
    }

    fn update(&mut self) -> Result<ShouldUpdateDisplay> {
        panic!();

        if (CrankstartAPI::get().system.get_button_state().pushed & PDButtons::kButtonA).0 != 0 {
            self.state = Some(
                self.state
                    .map_or(SysGameState::AddDefaultMenuItem, SysGameState::next),
            );
            self.on_state_change();
        }

        Ok(false.into())
    }
}

crankstart_game!(SysGame);

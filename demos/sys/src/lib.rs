#![no_std]

use {
    anyhow::Result,
    core::{cell::Ref, fmt::Debug, mem::transmute},
    crankstart::{
        CrankstartAPI, Game,
        alloc::{boxed::Box, vec::Vec},
        crankstart_game, define_enum_count, println,
        sys::{Button, menu_item::MenuItemPtr},
        util::{
            callback::Callback, enums::count::EnumCount, singleton::Singleton,
            string::ShortTempString,
        },
        write0,
    },
};

define_enum_count! {
    #[repr(u8)]
    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    enum SysDemoState {
        AddDefaultMenuItem,
        AddCheckmarkMenuItem,
        AddOptionsMenuItem,
    }
}

impl SysDemoState {
    fn get_short_name(self) -> &'static str {
        match self {
            SysDemoState::AddDefaultMenuItem => "Default",
            SysDemoState::AddCheckmarkMenuItem => "Checkmark",
            SysDemoState::AddOptionsMenuItem => "Options",
        }
    }

    fn next(self) -> Self {
        unsafe { transmute::<u8, Self>((self as u8 + 1_u8) % Self::COUNT as u8) }
    }
}

#[derive(Default)]
struct SysDemo {
    state: Option<SysDemoState>,
    menu_items: Vec<MenuItemPtr>,
}

impl SysDemo {
    fn build_title(&self, menu_item_index: usize) -> ShortTempString {
        let short_name: &str = self.state.unwrap().get_short_name();
        let mut title: ShortTempString = ShortTempString::new();

        write0!(&mut title, "{short_name} {menu_item_index}").unwrap();

        title
    }

    fn on_state_change(&mut self) {
        let crankstart_api: Ref<CrankstartAPI> = CrankstartAPI::get();

        crankstart_api.system.remove_all_menu_items();
        self.menu_items.clear();

        match self.state.clone() {
            Some(SysDemoState::AddDefaultMenuItem) => self.add_default_menu_item(&crankstart_api),
            Some(SysDemoState::AddCheckmarkMenuItem) => {
                self.add_checkmark_menu_item(&crankstart_api)
            }
            Some(SysDemoState::AddOptionsMenuItem) => self.add_options_menu_item(&crankstart_api),
            None => (),
        }
    }

    fn add_default_menu_item(&mut self, crankstart_api: &CrankstartAPI) {
        for menu_item_index in 0_usize..3_usize {
            let title: ShortTempString = self.build_title(menu_item_index);

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
            let title: ShortTempString = self.build_title(menu_item_index);

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
            let title: ShortTempString = self.build_title(menu_item_index);

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

    fn build_callback<I: Debug>(title: &ShortTempString) -> Callback<I> {
        let mut title: ShortTempString = title.clone();

        // `title` is null-terminated, which we don't want when printing it below.
        title.pop();

        Callback::from_boxed_closure(Box::new(move |input: I| {
            println!("{title} selected: {input:?}");
        }))
    }
}

impl Game for SysDemo {
    fn new() -> Result<Self> {
        let mut sys_demo: Self = Default::default();

        sys_demo.on_state_change();

        Ok(Default::default())
    }

    fn update(&mut self) -> Result<bool> {
        if CrankstartAPI::get()
            .system
            .get_button_state()
            .pushed
            .get(Button::A)
        {
            self.state = Some(
                self.state
                    .map_or(SysDemoState::AddDefaultMenuItem, SysDemoState::next),
            );
            self.on_state_change();
        }

        Ok(false)
    }
}

crankstart_game!(SysDemo);

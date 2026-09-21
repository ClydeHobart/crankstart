#![no_std]

use {
    anyhow::Result,
    crankstart::{Game, ShouldUpdateDisplay, alloc::vec::Vec, crankstart_game, sys::MenuItemPtr},
};

struct SysGame {
    _menu_items: Vec<MenuItemPtr>,
}

impl Game for SysGame {
    fn new() -> Result<Self> {
        todo!()
    }

    fn update(&mut self) -> Result<ShouldUpdateDisplay> {
        todo!()
    }
}

crankstart_game!(SysGame);

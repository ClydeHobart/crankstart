use {
    self::menu_item::{
        CheckmarkMenuItemCallback, DefaultMenuItemCallback, MAX_MENU_ITEM_COUNT, MAX_OPTION_COUNT,
        MenuItemKind, MenuItemPtr, MenuItemState, OptionsMenuItemCallback,
    },
    crate::{
        CrankstartAPI, Game, System, define_crankstart_api, define_enum_count, define_enum_flags,
        define_enum_from_pd_flags, ensure, eprintln,
        pd_api::{
            __va_list_tag, LCDBitmap, PDButtonCallbackFunction, PDButtons, PDCallbackFunction,
            PDDateTime, PDLanguage, PDMenuItem, PDMenuItemCallbackFunction, PDPeripherals,
            ctypes::{c_char, c_void},
            playdate_sys,
        },
        q, str_lit,
        util::{
            r#enum::count::EnumCount,
            euclid::IPxPoint2D,
            ptr::{PtrInner, PtrTrait},
            singleton::Singleton,
            string::{ArrayStringTrait, LongTempString, ShortTempString, TempString},
        },
    },
    anyhow::{Error, Result, anyhow},
    arrayvec::ArrayVec,
    core::{
        cell::{Ref, RefCell, RefMut},
        convert::TryFrom,
        mem::transmute,
        num::TryFromIntError,
        ptr::{NonNull, null_mut},
        result::Result as CoreResult,
        time::Duration as CoreDuration,
    },
    euclid::default::Vector3D,
    static_assertions::const_assert_eq,
};

pub mod menu_item;

define_enum_from_pd_flags! {
    #[repr(u8)]
    #[flags(Buttons, PDButtons)]
    /// A parallel definition of [`PDButtons`] for use in the [`Buttons`] typed enum flag set.
    #[derive(Clone, Copy, PartialEq)]
    pub enum Button {
        #[pd_flag(kButtonLeft)]
        Left,
        #[pd_flag(kButtonRight)]
        Right,
        #[pd_flag(kButtonUp)]
        Up,
        #[pd_flag(kButtonDown)]
        Down,
        #[pd_flag(kButtonB)]
        B,
        #[pd_flag(kButtonA)]
        A,
    }
}

#[derive(Default, Clone, Copy)]
pub struct ButtonState {
    /// Which buttons are currently down.
    pub current: Buttons,

    /// Which buttons were pushed over the previous update cycle.
    pub pushed: Buttons,

    /// Which buttons were released over the previous update cycle.
    pub released: Buttons,
}

define_enum_count! {
    #[repr(u8)]
    /// A parallel definition of [`PDPeripherals`] for use in the [`Peripherals`] typed enum flag
    /// set.
    #[derive(Clone, Copy, PartialEq)]
    pub enum Peripheral {
        Accelerometer,
    }
}

const_assert_eq!(
    1_u32 << Peripheral::Accelerometer as u32,
    PDPeripherals::kAccelerometer as u32
);

define_enum_flags! {
    #[flags(Peripheral)]
    /// A typed enum flag set for [`Peripheral`].
    pub struct Peripherals;
}

/// A tuple of seconds and sub-second milliseconds, capable of expressing at most ~136.16 years.
#[derive(Default, Clone, Copy)]
pub struct Duration {
    pub seconds: u32,
    pub milliseconds: u32,
}

impl From<Duration> for CoreDuration {
    fn from(value: Duration) -> Self {
        Self::from_secs(value.seconds as u64) + Self::from_millis(value.milliseconds as u64)
    }
}

impl TryFrom<CoreDuration> for Duration {
    type Error = TryFromIntError;

    fn try_from(value: CoreDuration) -> CoreResult<Self, Self::Error> {
        let seconds: u32 = u32::try_from(value.as_secs())?;
        let milliseconds: u32 = value.subsec_millis();

        Ok(Self {
            seconds,
            milliseconds,
        })
    }
}

#[derive(Default)]
struct MenuItemArrayVec(ArrayVec<MenuItemPtr, MAX_MENU_ITEM_COUNT>);

impl System for MenuItemArrayVec {}

define_crankstart_api! {
    /// `crankstart` wrapper around C's `playdate_sys`
    pub struct SysAPI => playdate_sys {
        ; // No sub-API fields
        realloc: unsafe extern "C" fn(ptr: *mut c_void, size: usize) -> *mut c_void,
        formatString: unsafe extern "C" fn(
            ret: *mut *mut c_char,
            fmt: *const c_char,
            ...
        ) -> i32,
        logToConsole: unsafe extern "C" fn(fmt: *const c_char, ...),
        error: unsafe extern "C" fn(fmt: *const c_char, ...),
        getLanguage: unsafe extern "C" fn() -> PDLanguage,
        getCurrentTimeMilliseconds: unsafe extern "C" fn() -> u32,
        getSecondsSinceEpoch: unsafe extern "C" fn(milliseconds: *mut u32) -> u32,
        drawFPS: unsafe extern "C" fn(x: i32, y: i32),
        setUpdateCallback:
            unsafe extern "C" fn(update: PDCallbackFunction, userdata: *mut c_void),
        getButtonState: unsafe extern "C" fn(
            current: *mut PDButtons,
            pushed: *mut PDButtons,
            released: *mut PDButtons,
        ),
        setPeripheralsEnabled: unsafe extern "C" fn(mask: PDPeripherals),
        getAccelerometer:
            unsafe extern "C" fn(outx: *mut f32, outy: *mut f32, outz: *mut f32),
        getCrankChange: unsafe extern "C" fn() -> f32,
        getCrankAngle: unsafe extern "C" fn() -> f32,
        isCrankDocked: unsafe extern "C" fn() -> i32,
        setCrankSoundsDisabled: unsafe extern "C" fn(flag: i32) -> i32,
        getFlipped: unsafe extern "C" fn() -> i32,
        setAutoLockDisabled: unsafe extern "C" fn(disable: i32),
        setMenuImage: unsafe extern "C" fn(bitmap: *mut LCDBitmap, xOffset: i32),
        addMenuItem: unsafe extern "C" fn(
            title: *const c_char,
            callback: PDMenuItemCallbackFunction,
            userdata: *mut c_void,
        ) -> *mut PDMenuItem,
        addCheckmarkMenuItem: unsafe extern "C" fn(
            title: *const c_char,
            value: i32,
            callback: PDMenuItemCallbackFunction,
            userdata: *mut c_void,
        ) -> *mut PDMenuItem,
        addOptionsMenuItem: unsafe extern "C" fn(
            title: *const c_char,
            optionTitles: *mut *const c_char,
            optionsCount: i32,
            callback: PDMenuItemCallbackFunction,
            userdata: *mut c_void,
        ) -> *mut PDMenuItem,
        removeAllMenuItems: unsafe extern "C" fn(),
        removeMenuItem: unsafe extern "C" fn(menuItem: *mut PDMenuItem),
        getMenuItemValue: unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> i32,
        setMenuItemValue:  unsafe extern "C" fn(menuItem: *mut PDMenuItem, value: i32),
        getMenuItemTitle: unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *const c_char,
        setMenuItemTitle: unsafe extern "C" fn(menuItem: *mut PDMenuItem, title: *const c_char),
        getMenuItemUserdata: unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *mut c_void,
        setMenuItemUserdata: unsafe extern "C" fn(menuItem: *mut PDMenuItem, ud: *mut c_void),
        getReduceFlashing: unsafe extern "C" fn() -> i32,
        getElapsedTime: unsafe extern "C" fn() -> f32,
        resetElapsedTime: unsafe extern "C" fn(),
        getBatteryPercentage: unsafe extern "C" fn() -> f32,
        getBatteryVoltage: unsafe extern "C" fn() -> f32,
        getTimezoneOffset: unsafe extern "C" fn() -> i32,
        shouldDisplay24HourTime: unsafe extern "C" fn() -> i32,
        convertEpochToDateTime: unsafe extern "C" fn(epoch: u32, datetime: *mut PDDateTime),
        convertDateTimeToEpoch: unsafe extern "C" fn(datetime: *mut PDDateTime) -> u32,
        clearICache: unsafe extern "C" fn(),
        setButtonCallback: unsafe extern "C" fn(
            cb: PDButtonCallbackFunction,
            buttonud: *mut c_void,
            queuesize: i32,
        ),
        setSerialMessageCallback: unsafe extern "C" fn(
            callback: Option<unsafe extern "C" fn(data: *const c_char)>,
        ),
        vaFormatString: unsafe extern "C" fn(
            outstr: *mut *mut c_char,
            fmt: *const c_char,
            args: *mut __va_list_tag,
        ) -> i32,
        parseString: unsafe extern "C" fn(
            str_: *const c_char,
            format: *const c_char,
            ...
        ) -> i32;
        menu_items: RefCell<MenuItemArrayVec>,
    }
}

impl SysAPI {
    /// A helper function used by macros `crankstart::println` and `crankstart::eprintln`.
    pub fn print_internal<F: Fn(&mut LongTempString), G: Fn(&SysAPI, &str)>(
        write_closure: F,
        log_fn: G,
    ) {
        // Don't use `CrankstartAPI::get` in here. If the singleton hasn't been setup yet, we'll get
        // a double-tap on startup that's difficult to debug.
        if let Some(crankstart_api) = CrankstartAPI::try_get() {
            let mut temp_string: LongTempString = LongTempString::new();

            write_closure(&mut temp_string);

            log_fn(&crankstart_api.system, &temp_string.as_str());
        }
    }

    /// Calls the log function.
    ///
    /// This accepts a `&str` to print. To format a string and then print it, see
    /// [`crankstart::println!`].
    ///
    /// Silently consumes any error encountered. To recover errors instead, see
    /// [Self::try_log_to_console].
    ///
    /// | Language | Equivalent function                               |
    /// | :------- | :------------------------------------------------ |
    /// | Rust     | `CrankstartAPI::get().system.log_to_console(...)` |
    /// | C        | `pd->system->logToConsole(...)`                   |
    /// | Lua      | `print(...)`                                      |
    pub fn log_to_console(&self, string: &str) {
        self.try_log_to_console(string).ok();
    }

    /// Calls the log function.
    ///
    /// Returns any error encountered. To silently consume errors instead, or for language
    /// translations, see [Self::log_to_console].
    pub fn try_log_to_console(&self, string: &str) -> Result<()> {
        self.try_log_internal(string, CrankstartAPI::get().system.logToConsole)
    }

    /// Calls the log function, outputting an error in red to the console, then pauses execution.
    ///
    /// This accepts a `&str` to print. To format a string and then print it, see
    /// [`crankstart::eprintln!`].
    ///
    /// Silently consumes any error encountered. To recover errors instead, see [Self::try_error].
    ///
    /// | Language | Equivalent function                               |
    /// | :------- | :------------------------------------------------ |
    /// | Rust     | `CrankstartAPI::get().system.error(...)`          |
    /// | C        | `pd->system->error(...)`                          |
    pub fn error(&self, string: &str) {
        self.try_error(string).ok();
    }

    /// Calls the log function, outputting an error in red to the console, then pauses execution.
    ///
    /// Returns any error encountered. To silently consume errors instead, or for language
    /// translations, see [Self::error].
    pub fn try_error(&self, string: &str) -> Result<()> {
        self.try_log_internal(string, CrankstartAPI::get().system.error)
    }

    /// Returns the current language of the system.
    ///
    /// | Language | Equivalent function                          |
    /// | :------- | :------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.get_language()` |
    /// | C        | `pd->system->getLanguage()`                  |
    pub fn get_language(&self) -> PDLanguage {
        unsafe { (self.getLanguage)() }
    }

    /// Returns the number of milliseconds since ​some arbitrary point in time. This should present a
    /// consistent timebase while a game is running, but the counter will be disabled when the
    /// device is sleeping.
    ///
    /// | Language | Equivalent function                                           |
    /// | :------- | :------------------------------------------------------------ |
    /// | Rust     | `CrankstartAPI::get().system.get_current_time_milliseconds()` |
    /// | C        | `pd->system->getCurrentTimeMilliseconds()`                    |
    pub fn get_current_time_milliseconds(&self) -> u32 {
        unsafe { (self.getCurrentTimeMilliseconds)() }
    }

    /// Returns the [`Duration`] elapsed since midnight (hour 0), January 1, 2000.
    ///
    /// | Language | Equivalent function                                      |
    /// | :------- | :------------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.get_duration_since_epoch()` |
    /// | C        | `pd->system->getSecondsSinceEpoch(...)`                  |
    pub fn get_duration_since_epoch(&self) -> Duration {
        let mut milliseconds: u32 = 0_u32;

        let seconds: u32 = unsafe { (self.getSecondsSinceEpoch)(&mut milliseconds) };

        Duration {
            seconds,
            milliseconds,
        }
    }

    /// Calculates the current frames per second and draws that value at `point`.
    ///
    /// | Language | Equivalent function                           |
    /// | :------- | :-------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.draw_fps(point)` |
    /// | C        | `pd->system->drawFPS(point.x, point.y)`       |
    pub fn draw_fps(&self, point: IPxPoint2D) {
        unsafe {
            (self.drawFPS)(point.x, point.y);
        }
    }

    /// Returns the state of the buttons. Field `current` is a typed bitmask indicating which
    /// buttons are currently down. Fields `pushed` and `released` reflect which buttons were pushed
    /// or released over the previous update cycle--at the nominal frame rate of 50 ms, fast button
    /// presses can be missed if you just poll the instantaneous state.
    ///
    /// | Language | Equivalent function                              |
    /// | :------- | :----------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.get_button_state()` |
    /// | C        | `pd->system->getButtonState(...)`                |
    pub fn get_button_state(&self) -> ButtonState {
        let mut current: PDButtons = PDButtons(0_u32);
        let mut pushed: PDButtons = PDButtons(0_u32);
        let mut released: PDButtons = PDButtons(0_u32);

        unsafe {
            (self.getButtonState)(&mut current, &mut pushed, &mut released);
        }

        ButtonState {
            current: current.into(),
            pushed: pushed.into(),
            released: released.into(),
        }
    }

    /// By default, the accelerometer is disabled to save (a small amount of) power. To use a
    /// peripheral, it must first be enabled via this function. Accelerometer data is not available
    /// until the next update cycle after it’s enabled.
    ///
    /// | Language | Equivalent function                                        |
    /// | :------- | :--------------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.set_peripherals_enabled(...)` |
    /// | C        | `pd->system->setPeripheralsEnabled(...)`                   |
    pub fn set_peripherals_enabled(&self, mask: Peripherals) {
        unsafe {
            let mask: PDPeripherals = transmute::<u32, PDPeripherals>(mask.into_inner() as u32);

            (self.setPeripheralsEnabled)(mask);
        }
    }

    /// Returns the last-read accelerometer data.
    ///
    /// | Language | Equivalent function                               |
    /// | :------- | :------------------------------------------------ |
    /// | Rust     | `CrankstartAPI::get().system.get_accelerometer()` |
    /// | C        | `pd->system->getAccelerometer(...)`               |
    pub fn get_accelerometer(&self) -> Vector3D<f32> {
        let mut vector_3d: Vector3D<f32> = Vector3D::zero();

        unsafe {
            (self.getAccelerometer)(&mut vector_3d.x, &mut vector_3d.y, &mut vector_3d.z);
        }

        vector_3d
    }

    /// Returns the angle change of the crank since the last time this function was called. Negative
    /// values are anti-clockwise.
    ///
    /// | Language | Equivalent function                              |
    /// | :------- | :----------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.get_crank_change()` |
    /// | C        | `pd->system->getCrankChange()`                   |
    pub fn get_crank_change(&self) -> f32 {
        unsafe { (self.getCrankChange)() }
    }

    /// Returns the current position of the crank, in the range 0-360. Zero is pointing up, and the
    /// value increases as the crank moves clockwise, as viewed from the right side of the device.
    ///
    /// | Language | Equivalent function                             |
    /// | :------- | :---------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.get_crank_angle()` |
    /// | C        | `pd->system->getCrankAngle()`                   |
    pub fn get_crank_angle(&self) -> f32 {
        unsafe { (self.getCrankAngle)() }
    }

    /// Returns whether or not the crank is folded into the unit.
    ///
    /// | Language | Equivalent function                             |
    /// | :------- | :---------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.is_crank_docked()` |
    /// | C        | `pd->system->isCrankDocked()`                   |
    pub fn is_crank_docked(&self) -> bool {
        unsafe { (self.isCrankDocked)() != 0_i32 }
    }

    /// Playdate has built-in sound effects for various system events, such as the menu opening or
    /// closing, USB cable plugged or unplugged, and the crank docked or undocked. Since games can
    /// receive notification of the crank docking and undocking, and may incorporate this into the
    /// game, we’ve provided a function for muting the default sounds for these events.
    ///
    /// The function returns the previous value for this setting.
    ///
    /// | Language | Equivalent function                                          |
    /// | :------- | :----------------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.set_crank_sounds_disabled(...)` |
    /// | C        | `pd->system->setCrankSoundsDisabled(...)`                    |
    pub fn set_crank_sounds_disabled(&self, disable: bool) -> bool {
        unsafe { (self.setCrankSoundsDisabled)(disable as i32) != 0_i32 }
    }

    /// Returns whether the global "flipped" system setting is set.
    ///
    /// | Language | Equivalent function                         |
    /// | :------- | :------------------------------------------ |
    /// | Rust     | `CrankstartAPI::get().system.get_flipped()` |
    /// | C        | `pd->system->getFlipped()`                  |
    pub fn get_flipped(&self) -> bool {
        unsafe { (self.getFlipped)() != 0_i32 }
    }

    /// Disables or enables the 3 minute auto lock feature. When called, the timer is reset to 3
    /// minutes.
    ///
    /// | Language | Equivalent function                                    |
    /// | :------- | :----------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.set_auto_lock_disabled()` |
    /// | C        | `pd->system->setAutoLockDisabled()`                    |
    pub fn set_auto_lock_disabled(&self, disable: bool) {
        unsafe {
            (self.setAutoLockDisabled)(disable as i32);
        }
    }

    /// Adds a new menu item to the system menu.
    ///
    /// * Parameter `title` will be the title displayed by the menu item.
    /// * When invoked by the user, this menu item will:
    ///     1. Invoke parameter `callback`.
    ///     2. Hide the system menu.
    ///     3. Unpause your game and call [`Game::handle_event`] with the
    ///        [`PDSystemEvent::kEventResume`] `event`.
    ///
    /// Your game can then present an options interface to the player, or take other action, in
    /// whatever manner you choose.
    ///
    /// | Language | Equivalent function                                      |
    /// | :------- | :------------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.add_default_menu_item(...)` |
    /// | C        | `pd->system->addMenuItem(...)`                           |
    ///
    /// [`PDSystemEvent::kEventResume`]: crate::pd_api::PDSystemEvent#variant.kEventResume
    pub fn add_default_menu_item(
        &self,
        title: &str,
        callback: DefaultMenuItemCallback,
    ) -> Result<MenuItemPtr> {
        ensure!(!self.menu_items.borrow().0.is_full());
        ensure!(title.is_ascii());

        let title: TempString = TempString::clone_null_terminated_truncating(title);
        let title: *const c_char = title.as_ptr() as *const c_char;
        let pd_menu_item: *mut PDMenuItem =
            unsafe { (self.addMenuItem)(title, Some(Self::menu_item_callback), null_mut()) };
        let pd_menu_item: NonNull<PDMenuItem> = q!(NonNull::new(pd_menu_item).ok_or(()));
        let menu_item: MenuItemPtr =
            MenuItemPtr::new(pd_menu_item, MenuItemState::new_default(callback));

        // Now the `MenuItemState` is living on the heap and can safely be set as the userdata.
        self.set_menu_item_user_data(&menu_item);
        self.menu_items.borrow_mut().0.push(menu_item.clone());

        Ok(menu_item)
    }

    /// Adds a new menu item that can be checked or unchecked by the player.
    ///
    /// * Parameter `title` will be the title displayed by the menu item.
    /// * Parameter `is_checked` is whether the menu item is checked initially.
    /// * If this menu item is interacted with while the system menu is open, parameter `callback`
    ///   will be called when the menu is closed.
    ///
    /// | Language | Equivalent function                                        |
    /// | :------- | :--------------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.add_checkmark_menu_item(...)` |
    /// | C        | `pd->system->addCheckmarkMenuItem(...)`                    |
    pub fn add_checkmark_menu_item(
        &self,
        title: &str,
        is_checked: bool,
        callback: CheckmarkMenuItemCallback,
    ) -> Result<MenuItemPtr> {
        ensure!(!self.menu_items.borrow().0.is_full());
        ensure!(title.is_ascii());

        let title: TempString = TempString::clone_null_terminated_truncating(title);
        let title: *const c_char = title.as_ptr() as *const c_char;
        let pd_menu_item: *mut PDMenuItem = unsafe {
            (self.addCheckmarkMenuItem)(
                title,
                is_checked as i32,
                Some(Self::menu_item_callback),
                null_mut(),
            )
        };
        let pd_menu_item: NonNull<PDMenuItem> = q!(NonNull::new(pd_menu_item).ok_or(()));
        let menu_item: MenuItemPtr =
            MenuItemPtr::new(pd_menu_item, MenuItemState::new_checkmark(callback));

        // Now the `MenuItemState` is living on the heap and can safely be set as the userdata.
        self.set_menu_item_user_data(&menu_item);
        self.menu_items.borrow_mut().0.push(menu_item.clone());

        Ok(menu_item)
    }

    /// Adds a new menu item that allows the player to cycle through a set of options.
    ///
    /// * Parameter `title` will be the title displayed by the menu item.
    /// * Parameter `options` should be an array of strings representing the states this menu item
    ///   can cycle through. Due to limited horizontal space, the option strings and title should be
    ///   kept short for this type of menu item.
    /// * If this menu item is interacted with while the system menu is open, parameter `callback`
    ///   will be called when the menu is closed.
    ///
    /// | Language | Equivalent function                                      |
    /// | :------- | :------------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.add_options_menu_item(...)` |
    /// | C        | `pd->system->addOptionsMenuItem(...)`                    |
    pub fn add_options_menu_item(
        &self,
        title: &str,
        options: &[&str],
        callback: OptionsMenuItemCallback,
    ) -> Result<MenuItemPtr, Error> {
        ensure!(!self.menu_items.borrow().0.is_full());
        ensure!(title.is_ascii());
        ensure!(options.len() <= MAX_OPTION_COUNT);

        let title: TempString = TempString::clone_null_terminated_truncating(title);
        let title: *const c_char = title.as_ptr() as *const c_char;

        type OptionArrayVec = ArrayVec<TempString, MAX_OPTION_COUNT>;

        let options: OptionArrayVec = options
            .iter()
            .copied()
            .map(|option| {
                ensure!(option.is_ascii());

                Ok(TempString::clone_null_terminated_truncating(option))
            })
            .collect::<Result<OptionArrayVec, Error>>()?;

        type COptionArrayVec = ArrayVec<*const c_char, MAX_OPTION_COUNT>;

        let options: COptionArrayVec = options
            .iter()
            .map(|option| option.as_ptr() as *const c_char)
            .collect();
        let options_titles: *mut *const c_char = options.as_ptr() as *mut *const c_char;
        let pd_menu_item: *mut PDMenuItem = unsafe {
            (self.addOptionsMenuItem)(
                title,
                options_titles,
                options.len() as i32,
                Some(Self::menu_item_callback),
                null_mut(),
            )
        };
        let pd_menu_item: NonNull<PDMenuItem> = q!(NonNull::new(pd_menu_item).ok_or(()));
        let menu_item: MenuItemPtr = MenuItemPtr::new(
            pd_menu_item,
            q!(MenuItemState::try_new_options(options.len(), callback)),
        );

        // Now the `MenuItemState` is living on the heap and can safely be set as the userdata.
        self.set_menu_item_user_data(&menu_item);
        self.menu_items.borrow_mut().0.push(menu_item.clone());

        Ok(menu_item)
    }

    /// Removes all custom menu items from the system menu.
    ///
    /// | Language | Equivalent function                                   |
    /// | :------- | :---------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.remove_all_menu_items()` |
    /// | C        | `pd->system->removeAllMenuItems()`                    |
    pub fn remove_all_menu_items(&self) {
        unsafe {
            ((self.removeAllMenuItems)());
        }

        for menu_item in &mut self.menu_items.borrow_mut().0.drain(..) {
            if let Some(mut menu_item_state) = menu_item.try_borrow_state_mut() {
                menu_item_state.mark_as_removed();
            }
        }
    }

    /// Removes the menu item from the system menu.
    ///
    /// | Language | Equivalent function                              |
    /// | :------- | :----------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.remove_menu_item()` |
    /// | C        | `pd->system->removeMenuItem()`                   |
    pub fn remove_menu_item(&self, menu_item: MenuItemPtr) -> Result<()> {
        // One reference for parameter `menu_item`, one reference for what's stored in
        // self.menu_items, and one reference for what's stored in `CrankstartAPI::ptr_manager`.
        ensure!(menu_item.get_strong_count() == 3_usize);

        Self::try_get_borrowable_and_not_removed_menu_item(&menu_item)?;

        self.menu_items
            .borrow_mut()
            .0
            .retain(|stored_menu_item| stored_menu_item != &menu_item);

        // Explicitly drops item. The actual calling of the `removeMenuItem` (via
        // `remove_menu_item_internal`) is done in the drop impl to avoid calling it multiple times,
        // even though that's been experimentally shown to be safe.
        drop(menu_item);

        Ok(())
    }

    /// Gets the integer value of the menu item.
    ///
    /// * For default menu items, the value is 0.
    /// * For checkmark menu items, 1 means checked, 0 unchecked.
    /// * For option menu items, the value indicates the array index of the currently selected
    ///   option.
    ///
    /// Returns an error if the menu item is currently borrowed mutably.
    ///
    /// | Language | Equivalent function                                    |
    /// | :------- | :----------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.get_menu_item_value(...)` |
    /// | C        | `pd->system->getMenuItemValue(...)`                    |
    pub fn get_menu_item_value(&self, menu_item: &MenuItemPtr) -> Result<usize> {
        let menu_item_state: Ref<MenuItemState> =
            Self::try_get_borrowable_and_not_removed_menu_item(menu_item)?;

        const_assert_eq!(MenuItemKind::COUNT, 3_usize);

        Ok(match menu_item_state.get_kind() {
            // getMenuItemValue apparently can return garbage values for default menu items? This
            // needs to be verified.
            MenuItemKind::Default => 0_usize,
            _ => unsafe { (self.getMenuItemValue)(menu_item.get_pd_ptr().as_ptr()) as usize },
        })
    }

    /// Sets the integer value of the menu item.
    ///
    /// * For checkmark menu items, 1 means checked, 0 unchecked.
    /// * For option menu items, the value indicates the array index of the currently selected
    ///   option.
    ///
    /// Returns an error if:
    /// * The menu item is currently borrowed mutably.
    /// * The menu item is a default menu item.
    /// * An invalid value was supplied.
    ///
    /// | Language | Equivalent function                                    |
    /// | :------- | :----------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.set_menu_item_value(...)` |
    /// | C        | `pd->system->setMenuItemValue(...)`                    |
    pub fn set_menu_item_value(&self, menu_item: &MenuItemPtr, value: usize) -> Result<()> {
        let menu_item_state: Ref<MenuItemState> =
            Self::try_get_borrowable_and_not_removed_menu_item(menu_item)?;

        match menu_item_state.get_kind() {
            MenuItemKind::Default => {
                Err(anyhow!(str_lit!("Default menu items can't have a value")))?
            }
            MenuItemKind::Checkmark => ensure!(value <= 1_usize),
            MenuItemKind::Options => {
                ensure!(value < menu_item_state.try_get_option_count().unwrap())
            }
        }

        unsafe {
            (self.setMenuItemValue)(menu_item.get_pd_ptr().as_ptr(), value as i32);
        }

        Ok(())
    }

    /// Sets the display title of the menu item.
    ///
    /// Returns an error if the
    ///
    /// | Language | Equivalent function                                    |
    /// | :------- | :----------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.set_menu_item_title(...)` |
    /// | C        | `pd->system->setMenuItemTitle(...)`                    |
    pub fn get_menu_item_title(&self, menu_item: &MenuItemPtr) -> Result<ShortTempString> {
        Self::try_get_borrowable_and_not_removed_menu_item(menu_item)?;

        let title: *const c_char =
            unsafe { (self.getMenuItemTitle)(menu_item.get_pd_ptr().as_ptr()) };

        // Menu item titles are guaranteed to be valid ASCII, which is a subset of UTF-8.
        Ok(ShortTempString::try_clone_c_str_truncating(title).unwrap())
    }

    /// Sets the display title of the menu item.
    ///
    /// | Language | Equivalent function                                    |
    /// | :------- | :----------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.set_menu_item_title(...)` |
    /// | C        | `pd->system->setMenuItemTitle(...)`                    |
    pub fn set_menu_item_title(&self, menu_item: &MenuItemPtr, title: &str) -> Result<(), Error> {
        Self::try_get_borrowable_and_not_removed_menu_item(menu_item)?;
        ensure!(title.is_ascii());

        let title: TempString = TempString::clone_null_terminated_truncating(title);
        let title: *const c_char = title.as_ptr() as *const c_char;

        unsafe {
            (self.setMenuItemTitle)(menu_item.get_pd_ptr().as_ptr(), title);
        }

        Ok(())
    }

    /// Returns whether the global "reduce flashing" system setting is set.
    ///
    /// | Language | Equivalent function                                 |
    /// | :------- | :-------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.get_reduce_flashing()` |
    /// | C        | `pd->system->getReduceFlashing()`                   |
    pub fn get_reduce_flashing(&self) -> bool {
        unsafe { (self.getReduceFlashing)() != 0_i32 }
    }

    /// Returns the number of seconds since [`SysAPI::reset_elapsed_time`] was called. The value is
    /// a floating-point number with microsecond accuracy.
    ///
    /// | Language | Equivalent function                              |
    /// | :------- | :----------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.get_elapsed_time()` |
    /// | C        | `pd->system->getElapsedTime()`                   |
    pub fn get_elapsed_time(&self) -> f32 {
        unsafe { (self.getElapsedTime)() }
    }

    /// Resets the high-resolution timer.
    ///
    /// | Language | Equivalent function                                |
    /// | :------- | :------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.reset_elapsed_time()` |
    /// | C        | `pd->system->resetElapsedTime()`                   |
    pub fn reset_elapsed_time(&self) {
        unsafe { (self.resetElapsedTime)() }
    }

    /// Returns a value from 0-100 denoting the current level of battery charge. 0 = empty; 100 =
    /// full.
    ///
    /// | Language | Equivalent function                                    |
    /// | :------- | :----------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.get_battery_percentage()` |
    /// | C        | `pd->system->getBatteryPercentage()`                   |
    pub fn get_battery_percentage(&self) -> f32 {
        unsafe { (self.getBatteryPercentage)() }
    }

    /// Returns the battery’s current voltage level.
    ///
    /// | Language | Equivalent function                                 |
    /// | :------- | :-------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.get_battery_voltage()` |
    /// | C        | `pd->system->getBatteryVoltage()`                   |
    pub fn get_battery_voltage(&self) -> f32 {
        unsafe { (self.getBatteryVoltage)() }
    }

    /// Returns the system timezone offset from GMT, in seconds.
    ///
    /// | Language | Equivalent function                                 |
    /// | :------- | :-------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.get_timezone_offset()` |
    /// | C        | `pd->system->getTimezoneOffset()`                   |
    pub fn get_timezone_offset(&self) -> i32 {
        unsafe { (self.getTimezoneOffset)() }
    }

    /// Returns whether the user has set the 24-Hour Time preference in the Settings program.
    ///
    /// | Language | Equivalent function                                         |
    /// | :------- | :---------------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.should_display_24_hour_time()` |
    /// | C        | `pd->system->shouldDisplay24HourTime()`                     |
    pub fn should_display_24_hour_time(&self) -> bool {
        unsafe { (self.shouldDisplay24HourTime)() != 0_i32 }
    }

    /// Converts the given [`Duration`] elapsed since the Playdate epoch--midnight (hour 0), January
    /// 1, 2000-- to a [`PDDateTime`].
    ///
    /// | Language | Equivalent function                                           |
    /// | :------- | :------------------------------------------------------------ |
    /// | Rust     | `CrankstartAPI::get().system.convert_duration_to_date_time()` |
    /// | C        | `pd->system->convertEpochToDateTime()`                        |
    pub fn convert_duration_to_date_time(&self, duration: Duration) -> PDDateTime {
        let mut date_time: PDDateTime = PDDateTime::default();

        unsafe {
            (self.convertEpochToDateTime)(duration.seconds, &mut date_time);
        }

        date_time
    }

    /// Converts the given [`PDDateTime`] to the [`Duration`] elapsed since the Playdate epoch:
    /// midnight (hour 0), January 1, 2000.
    ///
    /// | Language | Equivalent function                                           |
    /// | :------- | :------------------------------------------------------------ |
    /// | Rust     | `CrankstartAPI::get().system.convert_duration_to_date_time()` |
    /// | C        | `pd->system->convertEpochToDateTime()`                        |
    pub fn convert_date_time_to_duration(&self, date_time: PDDateTime) -> Duration {
        let mut date_time: PDDateTime = date_time;

        let seconds: u32 = unsafe { (self.convertDateTimeToEpoch)(&mut date_time) };
        let milliseconds: u32 = 0_u32;

        Duration {
            seconds,
            milliseconds,
        }
    }

    /// Replaces the default Lua run loop function with a custom update function. The update
    /// function should return whether the system should update the display.
    ///
    /// This isn't fully public to keep the public API Rusty.
    ///
    /// | Language | Equivalent function                                 |
    /// | :------- | :-------------------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.set_update_callback()` |
    /// | C        | `pd->system->setUpdateCallback()`                   |
    pub(super) fn set_update_callback<G: Game>(&self, callback_function: PDCallbackFunction) {
        let user_data: *mut c_void = G::try_get_mut().map_or(null_mut(), |mut game: RefMut<G>| {
            (&mut (*game)) as *mut G as *mut c_void
        });

        // This is admittedly not guaranteed to be memory safe, but this is how the C API is set up.
        unsafe {
            (self.setUpdateCallback)(callback_function, user_data);
        }
    }

    /// Attempts to log a `str` through one of the C API's functions.
    ///
    /// Returns `Err` if the `string` isn't ASCII.
    ///
    /// This isn't public to keep the public API Rusty.
    fn try_log_internal(
        &self,
        string: &str,
        log_internal: unsafe extern "C" fn(*const c_char, ...),
    ) -> Result<()> {
        ensure!(string.is_ascii());

        let string: LongTempString = LongTempString::clone_null_terminated_truncating(string);
        let string: *const c_char = string.as_ptr() as *const c_char;

        // SAFETY: This was sourced from the Playdate API, and we're providing it a valid ASCII,
        // null-terminated string.
        unsafe {
            log_internal(string);
        }

        Ok(())
    }

    /// Sets the user data value associated with this menu item.
    ///
    /// This isn't public since we require the user data to fill a particular role between for
    /// invoking the callback (see [`SysAPI::menu_item_callback`]/
    /// [`SysAPI::menu_item_callback_internal`]).
    ///
    /// | Language | Equivalent function                                     |
    /// | :------- | :------------------------------------------------------ |
    /// | Rust     | `CrankstartAPI::get().system.set_menu_item_user_data()` |
    /// | C        | `pd->system->setMenuItemUserdata()`                     |
    fn set_menu_item_user_data(&self, menu_item: &MenuItemPtr) {
        let pd_menu_item: *mut PDMenuItem = menu_item.get_pd_ptr().as_ptr();
        let ptr_inner: &PtrInner<MenuItemPtr> = menu_item.get_ptr_inner();
        let user_data: *mut c_void = ptr_inner as *const PtrInner<MenuItemPtr> as *mut c_void;

        unsafe {
            (self.setMenuItemUserdata)(pd_menu_item, user_data);
        }
    }

    /// Removes the menu item from the system menu.
    ///
    /// This is only called when the last reference to a menu item is dropped.
    ///
    /// See public function [`SysAPI::remove_menu_item`].
    fn remove_menu_item_internal(&self, pd_menu_item: NonNull<PDMenuItem>) {
        unsafe {
            (self.removeMenuItem)(pd_menu_item.as_ptr());
        }
    }

    fn try_get_borrowable_and_not_removed_menu_item<'m>(
        menu_item: &'m MenuItemPtr,
    ) -> Result<Ref<'m, MenuItemState>> {
        let menu_item_state: Ref<MenuItemState> = q!(menu_item.try_borrow_state().ok_or(()));

        ensure!(!menu_item_state.was_removed());

        Ok(menu_item_state)
    }

    /// Reinterprets the user data provided to the callback as the same type that was provided to
    /// [`SysAPI::setMenuItemUserdata`] in [`SysAPI::set_menu_item_user_data`], and invokes the
    /// callback on the menu item's state.
    extern "C" fn menu_item_callback(user_data: *mut c_void) {
        let menu_item_callback = || -> Result<()> {
            let ptr_inner: *const PtrInner<MenuItemPtr> = user_data as *const PtrInner<MenuItemPtr>;

            ensure!(!ptr_inner.is_null());
            ensure!(ptr_inner.is_aligned());

            // We've just explicitly verified that it's not null.
            let ptr_inner: &PtrInner<MenuItemPtr> = unsafe { ptr_inner.as_ref_unchecked() };
            let menu_item: MenuItemPtr = q!(CrankstartAPI::get()
                .ptr_manager
                .borrow()
                .try_get_ptr(ptr_inner)
                .ok_or(()));
            let menu_item_state: Ref<MenuItemState> = q!(menu_item.try_borrow_state().ok_or(()));

            menu_item_state.invoke_callback(&menu_item);

            Ok(())
        };

        match menu_item_callback() {
            Err(e) => {
                eprintln!("{e}");
            }
            _ => (),
        }
    }

    /// * If `ptr` is not null and `size` is positive, reallocates the memory, possibly returning a
    ///   different pointer or null if enough memory cannot be allocated.
    /// * If `ptr` is not null and `size` equals zero, frees the memory and returns null.
    /// * If `ptr` is null and `size` is positive, returns a pointer to allocated memory large
    ///   enough to hold `size` bytes or null if it cannot be allocated.
    /// * If `ptr` is null and `size` is zero, returns null.
    ///
    /// | Language | Equivalent function                     |
    /// | :------- | :-------------------------------------- |
    /// | Rust     | `CrankstartAPI::get().system.realloc()` |
    /// | C        | `pd->system->realloc()`                 |
    #[cfg_attr(any(test, doctest), allow(dead_code))]
    pub(crate) fn realloc(&self, ptr: *mut u8, size: usize) -> *mut u8 {
        unsafe { (self.realloc)(ptr as *mut c_void, size) as *mut u8 }
    }
}

use {
    crate::{
        alloc::{boxed::Box, rc::Rc, string::String, vec::Vec},
        breakpoint_nop, define_crankstart_api, ensure,
        pd_api::{
            __va_list_tag,
            ctypes::{c_char, c_void},
            playdate_sys, LCDBitmap, PDButtonCallbackFunction, PDButtons, PDCallbackFunction,
            PDDateTime, PDLanguage, PDMenuItem, PDMenuItemCallbackFunction, PDPeripherals,
        },
        q,
        util::{
            euclid::IPxPoint2D,
            prelude::*,
            singleton::Singleton,
            string::{ArrayStringTrait, TempString},
        },
        CrankstartAPI, Game,
    },
    anyhow::{anyhow, Error, Result},
    arrayvec::ArrayVec,
    core::{
        cell::RefCell, convert::TryFrom, ffi::CStr, fmt::Write, num::TryFromIntError,
        ptr::null_mut, result::Result as CoreResult, time::Duration,
    },
    cstr_core::CString,
    euclid::default::Vector3D,
};

pub const MAX_OPTIONS: usize = 32_usize;

#[derive(Default, Clone, Copy)]
pub struct ButtonState {
    pub current: PDButtons,
    pub pushed: PDButtons,
    pub released: PDButtons,
}

#[derive(Default, Clone, Copy)]
pub struct PDDuration {
    pub seconds: u32,
    pub milliseconds: u32,
}

impl From<PDDuration> for Duration {
    fn from(value: PDDuration) -> Self {
        Self::from_secs(value.seconds as u64) + Self::from_millis(value.milliseconds as u64)
    }
}

impl TryFrom<Duration> for PDDuration {
    type Error = TryFromIntError;

    fn try_from(value: Duration) -> CoreResult<Self, Self::Error> {
        let seconds: u32 = u32::try_from(value.as_secs())?;
        let milliseconds: u32 = value.subsec_millis();

        Ok(Self {
            seconds,
            milliseconds,
        })
    }
}

define_crankstart_api! {
    pub struct SysAPI => playdate_sys {
        ; // No sub-API fields
        pub(crate) realloc: unsafe extern "C" fn(ptr: *mut c_void, size: usize) -> *mut c_void,
        pub(crate) formatString: unsafe extern "C" fn(
            ret: *mut *mut c_char,
            fmt: *const c_char,
            ...
        ) -> i32,
        pub(crate) logToConsole: unsafe extern "C" fn(fmt: *const c_char, ...),
        pub(crate) error: unsafe extern "C" fn(fmt: *const c_char, ...),
        pub(crate) getLanguage: unsafe extern "C" fn() -> PDLanguage,
        pub(crate) getCurrentTimeMilliseconds: unsafe extern "C" fn() -> u32,
        pub(crate) getSecondsSinceEpoch: unsafe extern "C" fn(milliseconds: *mut u32) -> u32,
        pub(crate) drawFPS: unsafe extern "C" fn(x: i32, y: i32),
        pub(crate) setUpdateCallback:
            unsafe extern "C" fn(update: PDCallbackFunction, userdata: *mut c_void),
        pub(crate) getButtonState: unsafe extern "C" fn(
            current: *mut PDButtons,
            pushed: *mut PDButtons,
            released: *mut PDButtons,
        ),
        pub(crate) setPeripheralsEnabled: unsafe extern "C" fn(mask: PDPeripherals),
        pub(crate) getAccelerometer:
            unsafe extern "C" fn(outx: *mut f32, outy: *mut f32, outz: *mut f32),
        pub(crate) getCrankChange: unsafe extern "C" fn() -> f32,
        pub(crate) getCrankAngle: unsafe extern "C" fn() -> f32,
        pub(crate) isCrankDocked: unsafe extern "C" fn() -> i32,
        pub(crate) setCrankSoundsDisabled: unsafe extern "C" fn(flag: i32) -> i32,
        pub(crate) getFlipped: unsafe extern "C" fn() -> i32,
        pub(crate) setAutoLockDisabled: unsafe extern "C" fn(disable: i32),
        pub(crate) setMenuImage: unsafe extern "C" fn(bitmap: *mut LCDBitmap, xOffset: i32),
        pub(crate) addMenuItem: unsafe extern "C" fn(
            title: *const c_char,
            callback: PDMenuItemCallbackFunction,
            userdata: *mut c_void,
        ) -> *mut PDMenuItem,
        pub(crate) addCheckmarkMenuItem: unsafe extern "C" fn(
            title: *const c_char,
            value: i32,
            callback: PDMenuItemCallbackFunction,
            userdata: *mut c_void,
        ) -> *mut PDMenuItem,
        pub(crate) addOptionsMenuItem: unsafe extern "C" fn(
            title: *const c_char,
            optionTitles: *mut *const c_char,
            optionsCount: i32,
            f: PDMenuItemCallbackFunction,
            userdata: *mut c_void,
        ) -> *mut PDMenuItem,
        pub(crate) removeAllMenuItems: unsafe extern "C" fn(),
        pub(crate) removeMenuItem: unsafe extern "C" fn(menuItem: *mut PDMenuItem),
        pub(crate) getMenuItemValue: unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> i32,
        pub(crate) setMenuItemValue:  unsafe extern "C" fn(menuItem: *mut PDMenuItem, value: i32),
        pub(crate) getMenuItemTitle:
            unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *const c_char,
        pub(crate) setMenuItemTitle:
            unsafe extern "C" fn(menuItem: *mut PDMenuItem, title: *const c_char),
        pub(crate) getMenuItemUserdata:
            unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *mut c_void,
        pub(crate) setMenuItemUserdata:
            unsafe extern "C" fn(menuItem: *mut PDMenuItem, ud: *mut c_void),
        pub(crate) getReduceFlashing: unsafe extern "C" fn() -> i32,
        pub(crate) getElapsedTime: unsafe extern "C" fn() -> f32,
        pub(crate) resetElapsedTime: unsafe extern "C" fn(),
        pub(crate) getBatteryPercentage: unsafe extern "C" fn() -> f32,
        pub(crate) getBatteryVoltage: unsafe extern "C" fn() -> f32,
        pub(crate) getTimezoneOffset: unsafe extern "C" fn() -> i32,
        pub(crate) shouldDisplay24HourTime: unsafe extern "C" fn() -> i32,
        pub(crate) convertEpochToDateTime:
            unsafe extern "C" fn(epoch: u32, datetime: *mut PDDateTime),
        pub(crate) convertDateTimeToEpoch: unsafe extern "C" fn(datetime: *mut PDDateTime) -> u32,
        pub(crate) clearICache: unsafe extern "C" fn(),
        pub(crate) setButtonCallback: unsafe extern "C" fn(
            cb: PDButtonCallbackFunction,
            buttonud: *mut c_void,
            queuesize: i32,
        ),
        pub(crate) setSerialMessageCallback: unsafe extern "C" fn(
            callback: Option<unsafe extern "C" fn(data: *const c_char)>,
        ),
        pub(crate) vaFormatString: unsafe extern "C" fn(
            outstr: *mut *mut c_char,
            fmt: *const c_char,
            args: *mut __va_list_tag,
        ) -> i32,
        pub(crate) parseString: unsafe extern "C" fn(
            str_: *const c_char,
            format: *const c_char,
            ...
        ) -> i32,
    }
}

impl SysAPI {
    /// A helper function used by macros `crankstart::println` and `crankstart::eprintln`.
    pub fn print_internal<F: Fn(&mut TempString), G: Fn(&SysAPI, &str)>(
        write_closure: F,
        log_fn: G,
    ) {
        let mut temp_string: TempString = TempString::new();

        write_closure(&mut temp_string);

        log_fn(&CrankstartAPI::get().system, &temp_string.as_str());
    }

    pub fn log_to_console(&self, string: &str) {
        self.try_log_to_console(string).ok();
    }

    pub fn try_log_to_console(&self, string: &str) -> Result<()> {
        self.try_log_internal(string, CrankstartAPI::get().system.logToConsole)
    }

    pub fn error(&self, string: &str) {
        self.try_error(string).ok();
    }

    pub fn try_error(&self, string: &str) -> Result<()> {
        self.try_log_internal(string, CrankstartAPI::get().system.error)
    }

    pub fn get_language(&self) -> PDLanguage {
        unsafe { (self.getLanguage)() }
    }

    pub fn get_current_time_milliseconds(&self) -> u32 {
        unsafe { (self.getCurrentTimeMilliseconds)() }
    }

    pub fn get_duration_since_epoch(&self) -> PDDuration {
        let mut milliseconds: u32 = 0_u32;

        let seconds: u32 = unsafe { (self.getSecondsSinceEpoch)(&mut milliseconds) };

        PDDuration {
            seconds,
            milliseconds,
        }
    }

    pub fn draw_fps(&self, point: IPxPoint2D) {
        unsafe {
            (self.drawFPS)(point.x, point.y);
        }
    }

    pub(crate) fn set_update_callback<G: Game>(&self, callback_function: PDCallbackFunction) {
        unsafe {
            (self.setUpdateCallback)(callback_function, null_mut());
        }
    }

    pub fn get_button_state(&self) -> ButtonState {
        let mut button_state: ButtonState = Default::default();

        unsafe {
            (self.getButtonState)(
                &mut button_state.current,
                &mut button_state.pushed,
                &mut button_state.released,
            );
        }

        button_state
    }

    pub fn set_peripherals_enabled(&self, mask: PDPeripherals) {
        unsafe {
            (self.setPeripheralsEnabled)(mask);
        }
    }

    pub fn get_accelerometer(&self) -> Vector3D<f32> {
        let mut vector_3d: Vector3D<f32> = Vector3D::zero();

        unsafe {
            (self.getAccelerometer)(&mut vector_3d.x, &mut vector_3d.y, &mut vector_3d.z);
        }

        vector_3d
    }

    pub fn get_crank_change(&self) -> f32 {
        unsafe { (self.getCrankChange)() }
    }

    pub fn get_crank_angle(&self) -> f32 {
        unsafe { (self.getCrankAngle)() }
    }

    pub fn is_crank_docked(&self) -> bool {
        unsafe { (self.isCrankDocked)() != 0_i32 }
    }

    pub fn set_crank_sounds_disabled(&self, disable: bool) -> bool {
        unsafe { (self.setCrankSoundsDisabled)(disable as i32) != 0_i32 }
    }

    pub fn get_flipped(&self) -> bool {
        unsafe { (self.getFlipped)() != 0_i32 }
    }

    pub fn set_auto_lock_disabled(&self, disable: bool) {
        unsafe {
            (self.setAutoLockDisabled)(disable as i32);
        }
    }

    /// Adds a option to the menu. The callback is called when the option is selected.
    pub fn add_menu_item(&self, title: &str, callback: Box<dyn Fn()>) -> Result<MenuItem> {
        let mut local_title: TempString = TempString::new();

        ensure!(title.is_ascii());
        write!(&mut local_title, "{title}")?;

        let c_str: &CStr = q!(CStr::from_bytes_with_nul(local_title.as_bytes()));
        let wrapped_callback: Box<Box<dyn Fn()>> = Box::new(callback);
        let raw_callback_ptr: *mut Box<dyn Fn()> = Box::into_raw(wrapped_callback);
        let item: *mut PDMenuItem = unsafe {
            (self.addMenuItem)(
                c_str.as_ptr(),
                Some(Self::menu_item_callback),
                raw_callback_ptr as *mut c_void,
            )
        };

        Ok(MenuItem {
            inner: Rc::new(RefCell::new(MenuItemInner {
                item,
                raw_callback_ptr,
            })),
            kind: MenuItemKind::Normal,
        })
    }

    /// Adds a option to the menu that has a checkbox. The initial_checked_state is the initial
    /// state of the checkbox. Callback will only be called when the menu is closed, not when the
    /// option is toggled. Use `System::get_menu_item_value` to get the state of the checkbox when
    /// the callback is called.
    pub fn add_checkmark_menu_item(
        &self,
        title: &str,
        initial_checked_state: bool,
        callback: Box<dyn Fn()>,
    ) -> Result<MenuItem, Error> {
        let mut local_title: TempString = TempString::new();

        ensure!(title.is_ascii());
        write!(&mut local_title, "{title}")?;

        let c_str: &CStr = q!(CStr::from_bytes_with_nul(local_title.as_bytes()));
        let wrapped_callback: Box<Box<dyn Fn()>> = Box::new(callback);
        let raw_callback_ptr: *mut Box<dyn Fn()> = Box::into_raw(wrapped_callback);
        let item: *mut PDMenuItem = unsafe {
            (self.addCheckmarkMenuItem)(
                c_str.as_ptr(),
                initial_checked_state as i32,
                Some(Self::menu_item_callback),
                raw_callback_ptr as *mut c_void,
            )
        };

        Ok(MenuItem {
            inner: Rc::new(RefCell::new(MenuItemInner {
                item,
                raw_callback_ptr,
            })),
            kind: MenuItemKind::Checkmark,
        })
    }

    /// Adds a option to the menu that has multiple values that can be cycled through. The initial
    /// value is the first element in `options`. Callback will only be called when the menu is
    /// closed, not when the option is toggled. Use `System::get_menu_item_value` to get the index
    /// of the options list when the callback is called, which can be used to lookup the value.
    pub fn add_options_menu_item(
        &self,
        title: &str,
        options: &[&str],
        callback: Box<dyn Fn()>,
    ) -> Result<MenuItem, Error> {
        ensure!(title.is_ascii());
        ensure!(options.len() <= MAX_OPTIONS);

        type OptionArrayVec = ArrayVec<TempString, MAX_OPTIONS>;

        let title: TempString = TempString::clone_null_terminated(title);
        let title: *const c_char = title.as_ptr() as *const c_char;
        let options: OptionArrayVec = options
            .iter()
            .copied()
            .map(|option| {
                ensure!(option.is_ascii());

                Ok(TempString::clone_null_terminated(option))
            })
            .collect::<Result<OptionArrayVec, Error>>()?;

        type COptionArrayVec = ArrayVec<*const c_char, MAX_OPTIONS>;

        let options: COptionArrayVec = options
            .iter()
            .map(|option| option.as_ptr() as *const c_char)
            .collect();
        let options_titles: *mut *const c_char = options.as_ptr() as *mut *const c_char;
        let options_count: i32 = options.len() as i32;
        let f: PDMenuItemCallbackFunction = Some(Self::menu_item_callback);
        let wrapped_callback: Box<Box<dyn Fn()>> = Box::new(callback);
        let raw_callback_ptr = Box::into_raw(wrapped_callback);
        let userdata: *mut c_void = raw_callback_ptr as *mut c_void;
        let item: *mut PDMenuItem =
            unsafe { (self.addOptionsMenuItem)(title, options_titles, options_count, f, userdata) };
        let inner: Rc<RefCell<MenuItemInner>> = Rc::new(RefCell::new(MenuItemInner {
            item,
            raw_callback_ptr,
        }));
        let kind: MenuItemKind = MenuItemKind::Options { options_count };

        Ok(MenuItem { inner, kind })
    }

    pub fn remove_menu_item(&self, item: MenuItem) {
        // Explicitly drops item. The actual calling of the removeMenuItem (via
        // `remove_menu_item_internal`) is done in the drop impl to avoid calling it multiple times,
        // even though that's been experimentally shown to be safe.
        drop(item);
    }

    /// Returns the state of a given menu item. The meaning depends on the type of menu item. If it
    /// is the checkbox, the int represents the boolean checked state. If it's a option the int
    /// represents the index of the option array.
    pub fn get_menu_item_value(&self, menu_item: &MenuItem) -> usize {
        unsafe { (self.getMenuItemValue)(menu_item.inner.borrow().item) as usize }
    }

    /// Set the value of a given menu item. The meaning depends on the type of menu item. Picking
    /// the right value is left up to the caller, but is protected by the `MenuItemKind` of the
    /// `menu_item` passed
    pub fn set_menu_item_value(&self, menu_item: &MenuItem, value: usize) -> Result<(), Error> {
        match &menu_item.kind {
            MenuItemKind::Normal => {}
            MenuItemKind::Checkmark => {
                ensure!(value <= 1_usize);
            }
            MenuItemKind::Options { options_count } => {
                ensure!(value < *options_count as usize);
            }
        };

        unsafe {
            (self.setMenuItemValue)(menu_item.inner.borrow().item, value as i32);
        }

        Ok(())
    }

    /// Set the title of a given menu item
    pub fn set_menu_item_title(&self, menu_item: &MenuItem, title: &str) -> Result<(), Error> {
        ensure!(title.is_ascii());

        let title: TempString = TempString::clone_null_terminated(title);
        let title: *const c_char = title.as_ptr() as *const c_char;

        unsafe {
            (self.setMenuItemTitle)(menu_item.inner.borrow().item, title);
        }

        Ok(())
    }

    pub fn get_reduced_flashing(&self) -> bool {
        unsafe { (self.getReduceFlashing)() != 0_i32 }
    }

    pub fn get_elapsed_time(&self) -> f32 {
        unsafe { (self.getElapsedTime)() }
    }

    pub fn reset_elapsed_time(&self) {
        unsafe { (self.resetElapsedTime)() }
    }

    pub fn get_battery_percentage(&self) -> f32 {
        unsafe { (self.getBatteryPercentage)() }
    }

    pub fn get_battery_voltage(&self) -> f32 {
        unsafe { (self.getBatteryVoltage)() }
    }

    pub fn get_timezone_offset(&self) -> i32 {
        unsafe { (self.getTimezoneOffset)() }
    }

    pub fn should_display_24_hour_time(&self) -> bool {
        unsafe { (self.shouldDisplay24HourTime)() != 0_i32 }
    }

    pub fn convert_duration_to_date_time(&self, duration: PDDuration) -> PDDateTime {
        let mut date_time: PDDateTime = PDDateTime::default();

        unsafe {
            (self.convertEpochToDateTime)(duration.seconds, &mut date_time);
        }

        date_time
    }

    pub fn convert_date_time_to_duration(&self, date_time: PDDateTime) -> PDDuration {
        let mut date_time: PDDateTime = date_time;

        let seconds: u32 = unsafe { (self.convertDateTimeToEpoch)(&mut date_time) };
        let milliseconds: u32 = 0_u32;

        PDDuration {
            seconds,
            milliseconds,
        }
    }

    fn try_log_internal(
        &self,
        string: &str,
        log_internal: unsafe extern "C" fn(*const c_char, ...),
    ) -> Result<()> {
        ensure!(string.is_ascii());

        let c_str: &CStr = q!(CStr::from_bytes_with_nul(string.as_bytes()));

        // SAFETY: This was sourced from the Playdate API, and we're providing it a valid ASCII,
        // null-terminated string.
        unsafe {
            log_internal(c_str.as_ptr());
        }

        Ok(())
    }

    fn remove_menu_item_internal(&self, item_inner: &MenuItemInner) {
        unsafe {
            (self.removeMenuItem)(item_inner.item);
        }
    }

    extern "C" fn menu_item_callback(user_data: *mut c_void) {
        let callback: *mut Box<dyn Fn()> = user_data as *mut Box<dyn Fn()>;

        if let Some(callback) = callback
            .is_aligned()
            .then_some(())
            .and_then(|_| unsafe { callback.as_ref() })
        {
            callback();
        }
    }
}

/// The kind of menu item. See `System::add_{,checkmark_,options_}menu_item` for more details.
pub enum MenuItemKind {
    Normal,
    Checkmark,
    Options { options_count: i32 },
}

struct MenuItemInner {
    item: *mut PDMenuItem,
    raw_callback_ptr: *mut Box<dyn Fn()>,
}

impl Drop for MenuItemInner {
    fn drop(&mut self) {
        // We must remove the menu item on drop to avoid a memory or having the firmware read
        // unmanaged memory.
        CrankstartAPI::get().system.remove_menu_item_internal(self);

        unsafe {
            // Recast into box to let Box deal with freeing the right memory
            let _: Box<Box<dyn Fn()>> = Box::from_raw(self.raw_callback_ptr);
        }
    }
}

pub struct MenuItem {
    inner: Rc<RefCell<MenuItemInner>>,
    pub kind: MenuItemKind,
}

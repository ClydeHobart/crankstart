use {
    crate::{
        CrankstartAPI, Game, System, define_crankstart_api, ensure, eprintln,
        pd_api::{
            __va_list_tag, LCDBitmap, PDButtonCallbackFunction, PDButtons, PDCallbackFunction,
            PDDateTime, PDLanguage, PDMenuItem, PDMenuItemCallbackFunction, PDPeripherals,
            ctypes::{c_char, c_void},
            playdate_sys,
        },
        q,
        util::{
            callback::Callback,
            euclid::IPxPoint2D,
            ptr::{PtrInner, PtrTrait, UntypedPtr},
            singleton::Singleton,
            string::{ArrayStringTrait, TempString},
        },
    },
    anyhow::{Error, Result},
    arrayvec::ArrayVec,
    core::{
        cell::{Ref, RefCell, RefMut},
        convert::TryFrom,
        ffi::CStr,
        num::TryFromIntError,
        ptr::{NonNull, null_mut},
        result::Result as CoreResult,
        time::Duration as CoreDuration,
    },
    euclid::default::Vector3D,
};

pub const MAX_OPTION_COUNT: usize = 32_usize;
pub const MAX_MENU_ITEM_COUNT: usize = 32_usize;

pub type DefaultMenuItemCallback = Callback;
pub type CheckboxMenuItemCallback = Callback<bool>;
pub type OptionsMenuItemCallback = Callback<usize>;

#[derive(Default, Clone, Copy)]
pub struct ButtonState {
    pub current: PDButtons,
    pub pushed: PDButtons,
    pub released: PDButtons,
}

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
    pub fn print_internal<F: Fn(&mut TempString), G: Fn(&SysAPI, &str)>(
        write_closure: F,
        log_fn: G,
    ) {
        // Don't use `CrankstartAPI::get` in here. If the singleton hasn't been setup yet, we'll get
        // a double-tap on startup that's difficult to debug.
        if let Some(crankstart_api) = CrankstartAPI::try_get() {
            let mut temp_string: TempString = TempString::new();

            write_closure(&mut temp_string);

            log_fn(&crankstart_api.system, &temp_string.as_str());
        }
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

    pub fn get_duration_since_epoch(&self) -> Duration {
        let mut milliseconds: u32 = 0_u32;

        let seconds: u32 = unsafe { (self.getSecondsSinceEpoch)(&mut milliseconds) };

        Duration {
            seconds,
            milliseconds,
        }
    }

    pub fn draw_fps(&self, point: IPxPoint2D) {
        unsafe {
            (self.drawFPS)(point.x, point.y);
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
    pub fn add_default_menu_item(
        &self,
        title: &str,
        callback: DefaultMenuItemCallback,
    ) -> Result<MenuItemPtr> {
        ensure!(!self.menu_items.borrow().0.is_full());
        ensure!(title.is_ascii());

        let title: TempString = TempString::clone_null_terminated(title);
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

    /// Adds a option to the menu that has a checkbox. The initial_checked_state is the initial
    /// state of the checkbox. Callback will only be called when the menu is closed, not when the
    /// option is toggled. Use `System::get_menu_item_value` to get the state of the checkbox when
    /// the callback is called.
    pub fn add_checkmark_menu_item(
        &self,
        title: &str,
        is_checked: bool,
        callback: CheckboxMenuItemCallback,
    ) -> Result<MenuItemPtr> {
        ensure!(!self.menu_items.borrow().0.is_full());
        ensure!(title.is_ascii());

        let title: TempString = TempString::clone_null_terminated(title);
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

    /// Adds a option to the menu that has multiple values that can be cycled through. The initial
    /// value is the first element in `options`. Callback will only be called when the menu is
    /// closed, not when the option is toggled. Use `System::get_menu_item_value` to get the index
    /// of the options list when the callback is called, which can be used to lookup the value.
    pub fn add_options_menu_item(
        &self,
        title: &str,
        options: &[&str],
        callback: OptionsMenuItemCallback,
    ) -> Result<MenuItemPtr, Error> {
        ensure!(!self.menu_items.borrow().0.is_full());
        ensure!(title.is_ascii());
        ensure!(options.len() <= MAX_OPTION_COUNT);

        let title: TempString = TempString::clone_null_terminated(title);
        let title: *const c_char = title.as_ptr() as *const c_char;

        type OptionArrayVec = ArrayVec<TempString, MAX_OPTION_COUNT>;

        let options: OptionArrayVec = options
            .iter()
            .copied()
            .map(|option| {
                ensure!(option.is_ascii());

                Ok(TempString::clone_null_terminated(option))
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

    pub fn remove_menu_item(&self, menu_item: MenuItemPtr) {
        self.menu_items
            .borrow_mut()
            .0
            .retain(|stored_menu_item| stored_menu_item != &menu_item);

        // Explicitly drops item. The actual calling of the removeMenuItem (via
        // `remove_menu_item_internal`) is done in the drop impl to avoid calling it multiple times,
        // even though that's been experimentally shown to be safe.
        drop(menu_item);
    }

    /// Returns the state of a given menu item. The meaning depends on the type of menu item. If it
    /// is the checkbox, the int represents the boolean checked state. If it's a option the int
    /// represents the index of the option array.
    pub fn get_menu_item_value(&self, menu_item: &MenuItemPtr) -> usize {
        unsafe { (self.getMenuItemValue)(menu_item.get_pd_ptr().as_ptr()) as usize }
    }

    /// Set the value of a given menu item. The meaning depends on the type of menu item. Picking
    /// the right value is left up to the caller, but is protected by the `MenuItemKind` of the
    /// `menu_item` passed
    pub fn set_menu_item_value(&self, menu_item: &MenuItemPtr, value: usize) -> Result<()> {
        {
            let menu_item_state: Ref<MenuItemState> = q!(menu_item.try_borrow_state().ok_or(()));
            let is_value_valid_for_checkmark: bool =
                !menu_item_state.is_checkmark() || value <= 1_usize;

            let is_value_valid_for_options: bool = menu_item_state
                .try_get_option_count()
                .map_or(true, |option_count| value < option_count);
            ensure!(is_value_valid_for_checkmark);
            ensure!(is_value_valid_for_options);
        }

        unsafe {
            (self.setMenuItemValue)(menu_item.get_pd_ptr().as_ptr(), value as i32);
        }

        Ok(())
    }

    /// Set the title of a given menu item
    pub fn set_menu_item_title(&self, menu_item: &MenuItemPtr, title: &str) -> Result<(), Error> {
        ensure!(title.is_ascii());

        let title: TempString = TempString::clone_null_terminated(title);
        let title: *const c_char = title.as_ptr() as *const c_char;

        unsafe {
            (self.setMenuItemTitle)(menu_item.get_pd_ptr().as_ptr(), title);
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

    pub fn convert_duration_to_date_time(&self, duration: Duration) -> PDDateTime {
        let mut date_time: PDDateTime = PDDateTime::default();

        unsafe {
            (self.convertEpochToDateTime)(duration.seconds, &mut date_time);
        }

        date_time
    }

    pub fn convert_date_time_to_duration(&self, date_time: PDDateTime) -> Duration {
        let mut date_time: PDDateTime = date_time;

        let seconds: u32 = unsafe { (self.convertDateTimeToEpoch)(&mut date_time) };
        let milliseconds: u32 = 0_u32;

        Duration {
            seconds,
            milliseconds,
        }
    }

    pub(crate) fn set_update_callback<G: Game>(&self, callback_function: PDCallbackFunction) {
        let user_data: *mut c_void = G::try_get_mut().map_or(null_mut(), |mut game: RefMut<G>| {
            (&mut (*game)) as *mut G as *mut c_void
        });

        // This is admittedly not guaranteed to be memory safe, but this is how the C API is set up.
        unsafe {
            (self.setUpdateCallback)(callback_function, user_data);
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

    fn set_menu_item_user_data(&self, menu_item: &MenuItemPtr) {
        let pd_menu_item: *mut PDMenuItem = menu_item.get_pd_ptr().as_ptr();
        let ptr_inner: &PtrInner<MenuItemPtr> = menu_item.get_ptr_inner();
        let user_data: *mut c_void = ptr_inner as *const PtrInner<MenuItemPtr> as *mut c_void;

        unsafe {
            (self.setMenuItemUserdata)(pd_menu_item, user_data);
        }
    }

    fn remove_menu_item_internal(&self, pd_menu_item: NonNull<PDMenuItem>) {
        unsafe {
            (self.removeMenuItem)(pd_menu_item.as_ptr());
        }
    }

    extern "C" fn menu_item_callback(user_data: *mut c_void) {
        match Self::menu_item_callback_internal(user_data) {
            Err(e) => {
                eprintln!("{e}");
            }
            _ => (),
        }
    }

    fn menu_item_callback_internal(user_data: *mut c_void) -> Result<()> {
        let ptr_inner: *const PtrInner<MenuItemPtr> = user_data as *const PtrInner<MenuItemPtr>;

        ensure!(!ptr_inner.is_null());
        ensure!(ptr_inner.is_aligned());

        let ptr_inner: &PtrInner<MenuItemPtr> = q!(unsafe { ptr_inner.as_ref() }.ok_or(()));
        let menu_item: MenuItemPtr = q!(CrankstartAPI::get()
            .ptr_manager
            .borrow()
            .try_get_ptr(ptr_inner)
            .ok_or(()));
        let menu_item_state: Ref<MenuItemState> = q!(menu_item.try_borrow_state().ok_or(()));

        menu_item_state.invoke_callback(&menu_item);

        Ok(())
    }

    #[cfg(not(any(test, doctest)))]
    pub(crate) fn realloc(&self, ptr: *mut u8, size: usize) -> *mut u8 {
        unsafe { (self.realloc)(ptr as *mut c_void, size) as *mut u8 }
    }
}

enum MenuItemKind {
    Default {
        was_removed: bool,
        callback: DefaultMenuItemCallback,
    },
    Checkmark {
        was_removed: bool,
        callback: CheckboxMenuItemCallback,
    },
    Options {
        was_removed: bool,
        option_count: u8,
        callback: OptionsMenuItemCallback,
    },
}

pub struct MenuItemState(MenuItemKind);

impl MenuItemState {
    pub fn is_default(&self) -> bool {
        matches!(self.0, MenuItemKind::Default { .. })
    }

    pub fn is_checkmark(&self) -> bool {
        matches!(self.0, MenuItemKind::Checkmark { .. })
    }

    pub fn is_options(&self) -> bool {
        matches!(self.0, MenuItemKind::Options { .. })
    }

    pub fn try_get_option_count(&self) -> Option<usize> {
        match self.0 {
            MenuItemKind::Options { option_count, .. } => Some(option_count as usize),
            _ => None,
        }
    }

    fn new_default(callback: DefaultMenuItemCallback) -> Self {
        Self(MenuItemKind::Default {
            was_removed: false,
            callback,
        })
    }

    fn new_checkmark(callback: CheckboxMenuItemCallback) -> Self {
        Self(MenuItemKind::Checkmark {
            was_removed: false,
            callback,
        })
    }

    fn try_new_options(option_count: usize, callback: OptionsMenuItemCallback) -> Result<Self> {
        ensure!(option_count < MAX_OPTION_COUNT);

        Ok(Self(MenuItemKind::Options {
            was_removed: false,
            option_count: option_count as u8,
            callback,
        }))
    }

    fn invoke_callback(&self, menu_item: &MenuItemPtr) {
        let crankstart_api: Ref<CrankstartAPI> = CrankstartAPI::get();

        match &self.0 {
            MenuItemKind::Default { callback, .. } => {
                callback.invoke(());
            }
            MenuItemKind::Checkmark { callback, .. } => {
                callback.invoke(crankstart_api.system.get_menu_item_value(menu_item) != 0_usize);
            }
            MenuItemKind::Options { callback, .. } => {
                callback.invoke(crankstart_api.system.get_menu_item_value(menu_item))
            }
        }
    }

    fn was_removed(&self) -> bool {
        *match &self.0 {
            MenuItemKind::Default { was_removed, .. } => was_removed,
            MenuItemKind::Checkmark { was_removed, .. } => was_removed,
            MenuItemKind::Options { was_removed, .. } => was_removed,
        }
    }

    fn mark_as_removed(&mut self) {
        *match &mut self.0 {
            MenuItemKind::Default { was_removed, .. } => was_removed,
            MenuItemKind::Checkmark { was_removed, .. } => was_removed,
            MenuItemKind::Options { was_removed, .. } => was_removed,
        } = true;
    }
}

#[derive(Clone, PartialEq)]
pub struct MenuItemPtr(UntypedPtr);

impl From<UntypedPtr> for MenuItemPtr {
    fn from(value: UntypedPtr) -> Self {
        Self(value)
    }
}

impl PtrTrait for MenuItemPtr {
    type PDType = PDMenuItem;

    type State = MenuItemState;

    fn get_untyped_ptr(&self) -> &UntypedPtr {
        &self.0
    }

    fn remove_pd_ptr(pd_ptr: NonNull<Self::PDType>, state: &Self::State) {
        if !state.was_removed() {
            CrankstartAPI::get()
                .system
                .remove_menu_item_internal(pd_ptr);
        }
    }
}

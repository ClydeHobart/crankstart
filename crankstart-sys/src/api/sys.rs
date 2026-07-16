use crate::{
    __va_list_tag,
    ctypes::{c_char, c_void},
    define_crankstart_api, playdate_sys, LCDBitmap, PDButtonCallbackFunction, PDButtons,
    PDCallbackFunction, PDDateTime, PDLanguage, PDMenuItem, PDMenuItemCallbackFunction,
    PDPeripherals,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct SysAPI => playdate_sys {
        ; // No sub-API fields
        pub realloc: unsafe extern "C" fn(ptr: *mut c_void, size: usize) -> *mut c_void,
        pub formatString: unsafe extern "C" fn(
            ret: *mut *mut c_char,
            fmt: *const c_char,
            ...
        ) -> i32,
        pub logToConsole: unsafe extern "C" fn(fmt: *const c_char, ...),
        pub error: unsafe extern "C" fn(fmt: *const c_char, ...),
        pub getLanguage: unsafe extern "C" fn() -> PDLanguage,
        pub getCurrentTimeMilliseconds: unsafe extern "C" fn() -> u32,
        pub getSecondsSinceEpoch: unsafe extern "C" fn(milliseconds: *mut u32) -> u32,
        pub drawFPS: unsafe extern "C" fn(x: i32, y: i32),
        pub setUpdateCallback:
            unsafe extern "C" fn(update: PDCallbackFunction, userdata: *mut c_void),
        pub getButtonState: unsafe extern "C" fn(
            current: *mut PDButtons,
            pushed: *mut PDButtons,
            released: *mut PDButtons,
        ),
        pub setPeripheralsEnabled: unsafe extern "C" fn(mask: PDPeripherals),
        pub getAccelerometer: unsafe extern "C" fn(outx: *mut f32, outy: *mut f32, outz: *mut f32),
        pub getCrankChange: unsafe extern "C" fn() -> f32,
        pub getCrankAngle: unsafe extern "C" fn() -> f32,
        pub isCrankDocked: unsafe extern "C" fn() -> i32,
        pub setCrankSoundsDisabled: unsafe extern "C" fn(flag: i32) -> i32,
        pub getFlipped: unsafe extern "C" fn() -> i32,
        pub setAutoLockDisabled: unsafe extern "C" fn(disable: i32),
        pub setMenuImage: unsafe extern "C" fn(bitmap: *mut LCDBitmap, xOffset: i32),
        pub addMenuItem: unsafe extern "C" fn(
            title: *const c_char,
            callback: PDMenuItemCallbackFunction,
            userdata: *mut c_void,
        ) -> *mut PDMenuItem,
        pub addCheckmarkMenuItem: unsafe extern "C" fn(
            title: *const c_char,
            value: i32,
            callback: PDMenuItemCallbackFunction,
            userdata: *mut c_void,
        ) -> *mut PDMenuItem,
        pub addOptionsMenuItem: unsafe extern "C" fn(
            title: *const c_char,
            optionTitles: *mut *const c_char,
            optionsCount: i32,
            f: PDMenuItemCallbackFunction,
            userdata: *mut c_void,
        ) -> *mut PDMenuItem,
        pub removeAllMenuItems: unsafe extern "C" fn(),
        pub removeMenuItem: unsafe extern "C" fn(menuItem: *mut PDMenuItem),
        pub getMenuItemValue: unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> i32,
        pub setMenuItemValue:  unsafe extern "C" fn(menuItem: *mut PDMenuItem, value: i32),
        pub getMenuItemTitle: unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *const c_char,
        pub setMenuItemTitle: unsafe extern "C" fn(menuItem: *mut PDMenuItem, title: *const c_char),
        pub getMenuItemUserdata: unsafe extern "C" fn(menuItem: *mut PDMenuItem) -> *mut c_void,
        pub setMenuItemUserdata: unsafe extern "C" fn(menuItem: *mut PDMenuItem, ud: *mut c_void),
        pub getReduceFlashing: unsafe extern "C" fn() -> i32,
        pub getElapsedTime: unsafe extern "C" fn() -> f32,
        pub resetElapsedTime: unsafe extern "C" fn(),
        pub getBatteryPercentage: unsafe extern "C" fn() -> f32,
        pub getBatteryVoltage: unsafe extern "C" fn() -> f32,
        pub getTimezoneOffset: unsafe extern "C" fn() -> i32,
        pub shouldDisplay24HourTime: unsafe extern "C" fn() -> i32,
        pub convertEpochToDateTime: unsafe extern "C" fn(epoch: u32, datetime: *mut PDDateTime),
        pub convertDateTimeToEpoch: unsafe extern "C" fn(datetime: *mut PDDateTime) -> u32,
        pub clearICache: unsafe extern "C" fn(),
        pub setButtonCallback: unsafe extern "C" fn(
            cb: PDButtonCallbackFunction,
            buttonud: *mut c_void,
            queuesize: i32,
        ),
        pub setSerialMessageCallback: unsafe extern "C" fn(
            callback: Option<unsafe extern "C" fn(data: *const c_char)>,
        ),
        pub vaFormatString: unsafe extern "C" fn(
            outstr: *mut *mut c_char,
            fmt: *const c_char,
            args: *mut __va_list_tag,
        ) -> i32,
        pub parseString: unsafe extern "C" fn(
            str_: *const c_char,
            format: *const c_char,
            ...
        ) -> i32,
    }
}

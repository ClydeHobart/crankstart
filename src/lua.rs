use crate::{
    define_crankstart_api,
    pd_api::{
        ctypes::{c_char, c_void},
        lua_CFunction, lua_reg, lua_val, playdate_lua, LCDBitmap, LCDSprite, LuaType, LuaUDObject,
    },
};

define_crankstart_api! {
    pub struct LuaAPI => playdate_lua {
        ; // No sub-API fields
        pub(crate) addFunction: unsafe extern "C" fn(
            f: lua_CFunction,
            name: *const c_char,
            outErr: *mut *const c_char,
        ) -> i32,
        pub(crate) registerClass: unsafe extern "C" fn(
            name: *const c_char,
            reg: *const lua_reg,
            vals: *const lua_val,
            isstatic: i32,
            outErr: *mut *const c_char,
        ) -> i32,
        pub(crate) pushFunction: unsafe extern "C" fn(f: lua_CFunction),
        pub(crate) indexMetatable: unsafe extern "C" fn() -> i32,
        pub(crate) stop: unsafe extern "C" fn(),
        pub(crate) start: unsafe extern "C" fn(),
        pub(crate) getArgCount: unsafe extern "C" fn() -> i32,
        pub(crate) getArgType:
            unsafe extern "C" fn(pos: i32, outClass: *mut *const c_char) -> LuaType,
        pub(crate) argIsNil: unsafe extern "C" fn(pos: i32) -> i32,
        pub(crate) getArgBool: unsafe extern "C" fn(pos: i32) -> i32,
        pub(crate) getArgInt: unsafe extern "C" fn(pos: i32) -> i32,
        pub(crate) getArgFloat: unsafe extern "C" fn(pos: i32) -> f32,
        pub(crate) getArgString: unsafe extern "C" fn(pos: i32) -> *const c_char,
        pub(crate) getArgBytes: unsafe extern "C" fn(pos: i32, outlen: *mut usize) -> *const c_char,
        pub(crate) getArgObject: unsafe extern "C" fn(
            pos: i32,
            type_: *mut c_char,
            outud: *mut *mut LuaUDObject,
        ) -> *mut c_void,
        pub(crate) getBitmap: unsafe extern "C" fn(pos: i32) -> *mut LCDBitmap,
        pub(crate) getSprite: unsafe extern "C" fn(pos: i32) -> *mut LCDSprite,
        pub(crate) pushNil: unsafe extern "C" fn(),
        pub(crate) pushBool: unsafe extern "C" fn(val: i32),
        pub(crate) pushInt: unsafe extern "C" fn(val: i32),
        pub(crate) pushFloat: unsafe extern "C" fn(val: f32),
        pub(crate) pushString: unsafe extern "C" fn(str_: *const c_char),
        pub(crate) pushBytes: unsafe extern "C" fn(str_: *const c_char, len: usize),
        pub(crate) pushBitmap: unsafe extern "C" fn(bitmap: *mut LCDBitmap),
        pub(crate) pushSprite: unsafe extern "C" fn(sprite: *mut LCDSprite),
        pub(crate) pushObject: unsafe extern "C" fn(
            obj: *mut c_void,
            type_: *mut c_char,
            nValues: i32,
        ) -> *mut LuaUDObject,
        pub(crate) retainObject: unsafe extern "C" fn(obj: *mut LuaUDObject) -> *mut LuaUDObject,
        pub(crate) releaseObject: unsafe extern "C" fn(obj: *mut LuaUDObject),
        pub(crate) setUserValue: unsafe extern "C" fn(obj: *mut LuaUDObject, slot: u32),
        pub(crate) getUserValue: unsafe extern "C" fn(obj: *mut LuaUDObject, slot: u32) -> i32,
        pub(crate) callFunction_deprecated: unsafe extern "C" fn(name: *const c_char, nargs: i32),
        pub(crate) callFunction: unsafe extern "C" fn(
            name: *const c_char,
            nargs: i32,
            outerr: *mut *const c_char,
        ) -> i32,
    }
}

use crate::{
    ctypes::{c_char, c_void},
    define_crankstart_api, lua_CFunction, lua_reg, lua_val, playdate_lua, LCDBitmap, LCDSprite,
    LuaType, LuaUDObject,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct LuaAPI => playdate_lua {
        ; // No sub-API fields
        pub addFunction: unsafe extern "C" fn(
            f: lua_CFunction,
            name: *const c_char,
            outErr: *mut *const c_char,
        ) -> i32,
        pub registerClass: unsafe extern "C" fn(
            name: *const c_char,
            reg: *const lua_reg,
            vals: *const lua_val,
            isstatic: i32,
            outErr: *mut *const c_char,
        ) -> i32,
        pub pushFunction: unsafe extern "C" fn(f: lua_CFunction),
        pub indexMetatable: unsafe extern "C" fn() -> i32,
        pub stop: unsafe extern "C" fn(),
        pub start: unsafe extern "C" fn(),
        pub getArgCount: unsafe extern "C" fn() -> i32,
        pub getArgType: unsafe extern "C" fn(pos: i32, outClass: *mut *const c_char) -> LuaType,
        pub argIsNil: unsafe extern "C" fn(pos: i32) -> i32,
        pub getArgBool: unsafe extern "C" fn(pos: i32) -> i32,
        pub getArgInt: unsafe extern "C" fn(pos: i32) -> i32,
        pub getArgFloat: unsafe extern "C" fn(pos: i32) -> f32,
        pub getArgString: unsafe extern "C" fn(pos: i32) -> *const c_char,
        pub getArgBytes: unsafe extern "C" fn(pos: i32, outlen: *mut usize) -> *const c_char,
        pub getArgObject: unsafe extern "C" fn(
            pos: i32,
            type_: *mut c_char,
            outud: *mut *mut LuaUDObject,
        ) -> *mut c_void,
        pub getBitmap: unsafe extern "C" fn(pos: i32) -> *mut LCDBitmap,
        pub getSprite: unsafe extern "C" fn(pos: i32) -> *mut LCDSprite,
        pub pushNil: unsafe extern "C" fn(),
        pub pushBool: unsafe extern "C" fn(val: i32),
        pub pushInt: unsafe extern "C" fn(val: i32),
        pub pushFloat: unsafe extern "C" fn(val: f32),
        pub pushString: unsafe extern "C" fn(str_: *const c_char),
        pub pushBytes: unsafe extern "C" fn(str_: *const c_char, len: usize),
        pub pushBitmap: unsafe extern "C" fn(bitmap: *mut LCDBitmap),
        pub pushSprite: unsafe extern "C" fn(sprite: *mut LCDSprite),
        pub pushObject: unsafe extern "C" fn(
            obj: *mut c_void,
            type_: *mut c_char,
            nValues: i32,
        ) -> *mut LuaUDObject,
        pub retainObject: unsafe extern "C" fn(obj: *mut LuaUDObject) -> *mut LuaUDObject,
        pub releaseObject: unsafe extern "C" fn(obj: *mut LuaUDObject),
        pub setUserValue: unsafe extern "C" fn(obj: *mut LuaUDObject, slot: u32),
        pub getUserValue: unsafe extern "C" fn(obj: *mut LuaUDObject, slot: u32) -> i32,
        pub callFunction_deprecated: unsafe extern "C" fn(name: *const c_char, nargs: i32),
        pub callFunction: unsafe extern "C" fn(
            name: *const c_char,
            nargs: i32,
            outerr: *mut *const c_char,
        ) -> i32,
    }
}

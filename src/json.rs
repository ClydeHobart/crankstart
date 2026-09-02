use crate::{
    define_crankstart_api,
    pd_api::{
        ctypes::{c_char, c_void},
        json_decoder, json_encoder, json_reader, json_value, json_writeFunc, playdate_json,
    },
};

define_crankstart_api! {
    #[allow(dead_code, non_snake_case)]
    pub struct JSONAPI => playdate_json {
        ; // No sub-API fields
        pub(crate) initEncoder: unsafe extern "C" fn(
            encoder: *mut json_encoder,
            write: json_writeFunc,
            userdata: *mut c_void,
            pretty: i32,
        ),
        pub(crate) decode: unsafe extern "C" fn(
            functions: *mut json_decoder,
            reader: json_reader,
            outval: *mut json_value,
        ) -> i32,
        pub(crate) decodeString: unsafe extern "C" fn(
            functions: *mut json_decoder,
            jsonString: *const c_char,
            outval: *mut json_value,
        ) -> i32,
    }
}

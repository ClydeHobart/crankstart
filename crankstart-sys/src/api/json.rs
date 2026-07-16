use crate::{
    ctypes::{c_char, c_void},
    define_crankstart_api, json_decoder, json_encoder, json_reader, json_value, json_writeFunc,
    playdate_json,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct JSONAPI => playdate_json {
        ; // No sub-API fields
        pub initEncoder: unsafe extern "C" fn(
            encoder: *mut json_encoder,
            write: json_writeFunc,
            userdata: *mut c_void,
            pretty: i32,
        ),
        pub decode: unsafe extern "C" fn(
            functions: *mut json_decoder,
            reader: json_reader,
            outval: *mut json_value,
        ) -> i32,
        pub decodeString: unsafe extern "C" fn(
            functions: *mut json_decoder,
            jsonString: *const c_char,
            outval: *mut json_value,
        ) -> i32,
    }
}

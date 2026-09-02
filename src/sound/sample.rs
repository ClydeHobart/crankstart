use crate::{
    define_crankstart_api,
    pd_api::{ctypes::c_char, playdate_sound_sample, AudioSample, SoundFormat},
};

define_crankstart_api! {
    #[allow(dead_code, non_snake_case)]
    pub struct SampleAPI => playdate_sound_sample {
        ; // No sub-API fields
        pub(crate) newSampleBuffer: unsafe extern "C" fn(byteCount: i32) -> *mut AudioSample,
        pub(crate) loadIntoSample: unsafe extern "C" fn(
            sample: *mut AudioSample,
            path: *const c_char,
        ) -> i32,
        pub(crate) load: unsafe extern "C" fn(path: *const c_char) -> *mut AudioSample,
        pub(crate) newSampleFromData: unsafe extern "C" fn(
            data: *mut u8,
            format: SoundFormat,
            sampleRate: u32,
            byteCount: i32,
            shouldFreeData: i32,
        ) -> *mut AudioSample,
        pub(crate) getData: unsafe extern "C" fn(
            sample: *mut AudioSample,
            data: *mut *mut u8,
            format: *mut SoundFormat,
            sampleRate: *mut u32,
            bytelength: *mut u32,
        ),
        pub(crate) freeSample: unsafe extern "C" fn(sample: *mut AudioSample),
        pub(crate) getLength: unsafe extern "C" fn(sample: *mut AudioSample) -> f32,
        pub(crate) decompress: unsafe extern "C" fn(sample: *mut AudioSample) -> i32,
    }
}

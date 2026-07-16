use crate::{
    ctypes::c_char, define_crankstart_api, playdate_sound_sample, AudioSample, SoundFormat,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct SampleAPI => playdate_sound_sample {
        ; // No sub-API fields
        pub newSampleBuffer: unsafe extern "C" fn(byteCount: i32) -> *mut AudioSample,
        pub loadIntoSample: unsafe extern "C" fn(
            sample: *mut AudioSample,
            path: *const c_char,
        ) -> i32,
        pub load: unsafe extern "C" fn(path: *const c_char) -> *mut AudioSample,
        pub newSampleFromData: unsafe extern "C" fn(
            data: *mut u8,
            format: SoundFormat,
            sampleRate: u32,
            byteCount: i32,
            shouldFreeData: i32,
        ) -> *mut AudioSample,
        pub getData: unsafe extern "C" fn(
            sample: *mut AudioSample,
            data: *mut *mut u8,
            format: *mut SoundFormat,
            sampleRate: *mut u32,
            bytelength: *mut u32,
        ),
        pub freeSample: unsafe extern "C" fn(sample: *mut AudioSample),
        pub getLength: unsafe extern "C" fn(sample: *mut AudioSample) -> f32,
        pub decompress: unsafe extern "C" fn(sample: *mut AudioSample) -> i32,
    }
}

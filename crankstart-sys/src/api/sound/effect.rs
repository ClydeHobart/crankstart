use {
    self::{
        bit_crusher::BitCrusherAPI, delay_line::DelayLineAPI, one_pole_filter::OnePoleFilterAPI,
        overdrive::OverdriveAPI, ring_modulator::RingModulatorAPI,
        two_pole_filter::TwoPoleFilterAPI,
    },
    crate::{
        ctypes::c_void, define_crankstart_api, effectProc, playdate_sound_effect,
        PDSynthSignalValue, SoundEffect,
    },
};

pub mod bit_crusher;
pub mod delay_line;
pub mod one_pole_filter;
pub mod overdrive;
pub mod ring_modulator;
pub mod two_pole_filter;

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct EffectAPI => playdate_sound_effect {
        pub twopolefilter: TwoPoleFilterAPI,
        pub onepolefilter: OnePoleFilterAPI,
        pub bitcrusher: BitCrusherAPI,
        pub ringmodulator: RingModulatorAPI,
        pub delayline: DelayLineAPI,
        pub overdrive: OverdriveAPI;
        pub newEffect:
            unsafe extern "C" fn(proc_: effectProc, userdata: *mut c_void) -> *mut SoundEffect,
        pub freeEffect: unsafe extern "C" fn(effect: *mut SoundEffect),
        pub setMix: unsafe extern "C" fn(effect: *mut SoundEffect, level: f32),
        pub setMixModulator:
            unsafe extern "C" fn(effect: *mut SoundEffect, signal: *mut PDSynthSignalValue),
        pub getMixModulator:
            unsafe extern "C" fn(effect: *mut SoundEffect) -> *mut PDSynthSignalValue,
        pub setUserdata: unsafe extern "C" fn(effect: *mut SoundEffect, userdata: *mut c_void),
        pub getUserdata: unsafe extern "C" fn(effect: *mut SoundEffect) -> *mut c_void,
    }
}

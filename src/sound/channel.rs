use crate::{
    define_crankstart_api,
    pd_api::{
        ctypes::c_void, playdate_sound_channel, AudioSourceFunction, PDSynthSignalValue,
        SoundChannel, SoundEffect, SoundSource,
    },
};

define_crankstart_api! {
    pub struct ChannelAPI => playdate_sound_channel{
        ; // No sub-API fields
        pub(crate) newChannel: unsafe extern "C" fn() -> *mut SoundChannel,
        pub(crate) freeChannel: unsafe extern "C" fn(channel: *mut SoundChannel),
        pub(crate) addSource:
            unsafe extern "C" fn(channel: *mut SoundChannel, source: *mut SoundSource) -> i32,
        pub(crate) removeSource:
            unsafe extern "C" fn(channel: *mut SoundChannel, source: *mut SoundSource) -> i32,
        pub(crate) addCallbackSource: unsafe extern "C" fn(
            channel: *mut SoundChannel,
            callback: AudioSourceFunction,
            context: *mut c_void,
            stereo: i32,
        ) -> *mut SoundSource,
        pub(crate) addEffect:
            unsafe extern "C" fn(channel: *mut SoundChannel, effect: *mut SoundEffect),
        pub(crate) removeEffect:
            unsafe extern "C" fn(channel: *mut SoundChannel, effect: *mut SoundEffect),
        pub(crate) setVolume: unsafe extern "C" fn(channel: *mut SoundChannel, volume: f32),
        pub(crate) getVolume: unsafe extern "C" fn(channel: *mut SoundChannel) -> f32,
        pub(crate) setVolumeModulator:
            unsafe extern "C" fn(channel: *mut SoundChannel, mod_: *mut PDSynthSignalValue),
        pub(crate) getVolumeModulator:
            unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut PDSynthSignalValue,
        pub(crate) setPan: unsafe extern "C" fn(channel: *mut SoundChannel, pan: f32),
        pub(crate) setPanModulator:
            unsafe extern "C" fn(channel: *mut SoundChannel, mod_: *mut PDSynthSignalValue),
        pub(crate) getPanModulator:
            unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut PDSynthSignalValue,
        pub(crate) getDryLevelSignal:
            unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut PDSynthSignalValue,
        pub(crate) getWetLevelSignal:
            unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut PDSynthSignalValue,
    }
}

use crate::{
    ctypes::c_void, define_crankstart_api, playdate_sound_channel, AudioSourceFunction,
    PDSynthSignalValue, SoundChannel, SoundEffect, SoundSource,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct ChannelAPI => playdate_sound_channel{
        ; // No sub-API fields
        pub newChannel: unsafe extern "C" fn() -> *mut SoundChannel,
        pub freeChannel: unsafe extern "C" fn(channel: *mut SoundChannel),
        pub addSource:
            unsafe extern "C" fn(channel: *mut SoundChannel, source: *mut SoundSource) -> i32,
        pub removeSource:
            unsafe extern "C" fn(channel: *mut SoundChannel, source: *mut SoundSource) -> i32,
        pub addCallbackSource: unsafe extern "C" fn(
            channel: *mut SoundChannel,
            callback: AudioSourceFunction,
            context: *mut c_void,
            stereo: i32,
        ) -> *mut SoundSource,
        pub addEffect: unsafe extern "C" fn(channel: *mut SoundChannel, effect: *mut SoundEffect),
        pub removeEffect:
            unsafe extern "C" fn(channel: *mut SoundChannel, effect: *mut SoundEffect),
        pub setVolume: unsafe extern "C" fn(channel: *mut SoundChannel, volume: f32),
        pub getVolume: unsafe extern "C" fn(channel: *mut SoundChannel) -> f32,
        pub setVolumeModulator:
            unsafe extern "C" fn(channel: *mut SoundChannel, mod_: *mut PDSynthSignalValue),
        pub getVolumeModulator:
            unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut PDSynthSignalValue,
        pub setPan: unsafe extern "C" fn(channel: *mut SoundChannel, pan: f32),
        pub setPanModulator:
            unsafe extern "C" fn(channel: *mut SoundChannel, mod_: *mut PDSynthSignalValue),
        pub getPanModulator:
            unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut PDSynthSignalValue,
        pub getDryLevelSignal:
            unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut PDSynthSignalValue,
        pub getWetLevelSignal:
            unsafe extern "C" fn(channel: *mut SoundChannel) -> *mut PDSynthSignalValue,
    }
}

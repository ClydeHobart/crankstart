use crate::{
    define_crankstart_api,
    pd_api::{
        ctypes::{c_char, c_void},
        playdate_sound_sequence, SequenceFinishedCallback, SequenceTrack, SoundSequence,
    },
};

define_crankstart_api! {
    #[allow(dead_code, non_snake_case)]
    pub struct SequenceAPI => playdate_sound_sequence {
        ; // No sub-API fields
        pub(crate) newSequence: unsafe extern "C" fn() -> *mut SoundSequence,
        pub(crate) freeSequence: unsafe extern "C" fn(sequence: *mut SoundSequence),
        pub(crate) loadMIDIFile:
            unsafe extern "C" fn(seq: *mut SoundSequence, path: *const c_char) -> i32,
        pub(crate) getTime: unsafe extern "C" fn(seq: *mut SoundSequence) -> u32,
        pub(crate) setTime: unsafe extern "C" fn(seq: *mut SoundSequence, time: u32),
        pub(crate) setLoops: unsafe extern "C" fn(
            seq: *mut SoundSequence,
            loopstart: i32,
            loopend: i32,
            loops: i32,
        ),
        pub(crate) getTempo_deprecated: unsafe extern "C" fn(seq: *mut SoundSequence) -> i32,
        pub(crate) setTempo: unsafe extern "C" fn(seq: *mut SoundSequence, stepsPerSecond: f32),
        pub(crate) getTrackCount: unsafe extern "C" fn(seq: *mut SoundSequence) -> i32,
        pub(crate) addTrack: unsafe extern "C" fn(seq: *mut SoundSequence) -> *mut SequenceTrack,
        pub(crate) getTrackAtIndex:
            unsafe extern "C" fn(seq: *mut SoundSequence, track: u32) -> *mut SequenceTrack,
        pub(crate) setTrackAtIndex: unsafe extern "C" fn(
            seq: *mut SoundSequence,
            track: *mut SequenceTrack,
            idx: u32,
        ),
        pub(crate) allNotesOff: unsafe extern "C" fn(seq: *mut SoundSequence),
        pub(crate) isPlaying: unsafe extern "C" fn(seq: *mut SoundSequence) -> i32,
        pub(crate) getLength: unsafe extern "C" fn(seq: *mut SoundSequence) -> u32,
        pub(crate) play: unsafe extern "C" fn(
            seq: *mut SoundSequence,
            finishCallback: SequenceFinishedCallback,
            userdata: *mut c_void,
        ),
        pub(crate) stop: unsafe extern "C" fn(seq: *mut SoundSequence),
        pub(crate) getCurrentStep: unsafe extern "C" fn(
            seq: *mut SoundSequence,
            timeOffset: *mut i32,
        ) -> i32,
        pub(crate) setCurrentStep: unsafe extern "C" fn(
            seq: *mut SoundSequence,
            step: i32,
            timeOffset: i32,
            playNotes: i32,
        ),
        pub(crate) getTempo: unsafe extern "C" fn(seq: *mut SoundSequence) -> f32,
    }
}

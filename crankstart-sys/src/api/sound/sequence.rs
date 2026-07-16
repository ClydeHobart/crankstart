use crate::{
    ctypes::{c_char, c_void},
    define_crankstart_api, playdate_sound_sequence, SequenceFinishedCallback, SequenceTrack,
    SoundSequence,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct SequenceAPI => playdate_sound_sequence {
        ; // No sub-API fields
        pub newSequence: unsafe extern "C" fn() -> *mut SoundSequence,
        pub freeSequence: unsafe extern "C" fn(sequence: *mut SoundSequence),
        pub loadMIDIFile:
            unsafe extern "C" fn(seq: *mut SoundSequence, path: *const c_char) -> i32,
        pub getTime: unsafe extern "C" fn(seq: *mut SoundSequence) -> u32,
        pub setTime: unsafe extern "C" fn(seq: *mut SoundSequence, time: u32),
        pub setLoops: unsafe extern "C" fn(
            seq: *mut SoundSequence,
            loopstart: i32,
            loopend: i32,
            loops: i32,
        ),
        pub getTempo_deprecated: unsafe extern "C" fn(seq: *mut SoundSequence) -> i32,
        pub setTempo: unsafe extern "C" fn(seq: *mut SoundSequence, stepsPerSecond: f32),
        pub getTrackCount: unsafe extern "C" fn(seq: *mut SoundSequence) -> i32,
        pub addTrack: unsafe extern "C" fn(seq: *mut SoundSequence) -> *mut SequenceTrack,
        pub getTrackAtIndex:
            unsafe extern "C" fn(seq: *mut SoundSequence, track: u32) -> *mut SequenceTrack,
        pub setTrackAtIndex: unsafe extern "C" fn(
            seq: *mut SoundSequence,
            track: *mut SequenceTrack,
            idx: u32,
        ),
        pub allNotesOff: unsafe extern "C" fn(seq: *mut SoundSequence),
        pub isPlaying: unsafe extern "C" fn(seq: *mut SoundSequence) -> i32,
        pub getLength: unsafe extern "C" fn(seq: *mut SoundSequence) -> u32,
        pub play: unsafe extern "C" fn(
            seq: *mut SoundSequence,
            finishCallback: SequenceFinishedCallback,
            userdata: *mut c_void,
        ),
        pub stop: unsafe extern "C" fn(seq: *mut SoundSequence),
        pub getCurrentStep: unsafe extern "C" fn(
            seq: *mut SoundSequence,
            timeOffset: *mut i32,
        ) -> i32,
        pub setCurrentStep: unsafe extern "C" fn(
            seq: *mut SoundSequence,
            step: i32,
            timeOffset: i32,
            playNotes: i32,
        ),
        pub getTempo: unsafe extern "C" fn(seq: *mut SoundSequence) -> f32,
    }
}

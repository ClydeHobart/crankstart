use crate::{
    define_crankstart_api, playdate_sound_track, ControlSignal, MIDINote, PDSynthInstrument,
    SequenceTrack,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct TrackAPI => playdate_sound_track {
        ; // No sub-API fields
        pub newTrack: unsafe extern "C" fn() -> *mut SequenceTrack,
        pub freeTrack: unsafe extern "C" fn(track: *mut SequenceTrack),
        pub setInstrument:
            unsafe extern "C" fn(track: *mut SequenceTrack, inst: *mut PDSynthInstrument),
        pub getInstrument:
            unsafe extern "C" fn(track: *mut SequenceTrack) -> *mut PDSynthInstrument,
        pub addNoteEvent: unsafe extern "C" fn(
            track: *mut SequenceTrack,
            step: u32,
            len: u32,
            note: MIDINote,
            velocity: f32,
        ),
        pub removeNoteEvent:
            unsafe extern "C" fn(track: *mut SequenceTrack, step: u32, note: MIDINote),
        pub clearNotes: unsafe extern "C" fn(track: *mut SequenceTrack),
        pub getControlSignalCount: unsafe extern "C" fn(track: *mut SequenceTrack) -> i32,
        pub getControlSignal:
            unsafe extern "C" fn(track: *mut SequenceTrack, idx: i32) -> *mut ControlSignal,
        pub clearControlEvents: unsafe extern "C" fn(track: *mut SequenceTrack),
        pub getPolyphony: unsafe extern "C" fn(track: *mut SequenceTrack) -> i32,
        pub activeVoiceCount: unsafe extern "C" fn(track: *mut SequenceTrack) -> i32,
        pub setMuted: unsafe extern "C" fn(track: *mut SequenceTrack, mute: i32),
        pub getLength: unsafe extern "C" fn(track: *mut SequenceTrack) -> u32,
        pub getIndexForStep: unsafe extern "C" fn(track: *mut SequenceTrack, step: u32) -> i32,
        pub getNoteAtIndex: unsafe extern "C" fn(
            track: *mut SequenceTrack,
            index: i32,
            outStep: *mut u32,
            outLen: *mut u32,
            outNote: *mut MIDINote,
            outVelocity: *mut f32,
        ) -> i32,
        pub getSignalForController: unsafe extern "C" fn(
            track: *mut SequenceTrack,
            controller: i32,
            create: i32,
        ) -> *mut ControlSignal,
    }
}

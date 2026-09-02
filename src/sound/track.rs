use crate::{
    define_crankstart_api,
    pd_api::{playdate_sound_track, ControlSignal, MIDINote, PDSynthInstrument, SequenceTrack},
};

define_crankstart_api! {
    #[allow(dead_code, non_snake_case)]
    pub struct TrackAPI => playdate_sound_track {
        ; // No sub-API fields
        pub(crate) newTrack: unsafe extern "C" fn() -> *mut SequenceTrack,
        pub(crate) freeTrack: unsafe extern "C" fn(track: *mut SequenceTrack),
        pub(crate) setInstrument:
            unsafe extern "C" fn(track: *mut SequenceTrack, inst: *mut PDSynthInstrument),
        pub(crate) getInstrument:
            unsafe extern "C" fn(track: *mut SequenceTrack) -> *mut PDSynthInstrument,
        pub(crate) addNoteEvent: unsafe extern "C" fn(
            track: *mut SequenceTrack,
            step: u32,
            len: u32,
            note: MIDINote,
            velocity: f32,
        ),
        pub(crate) removeNoteEvent:
            unsafe extern "C" fn(track: *mut SequenceTrack, step: u32, note: MIDINote),
        pub(crate) clearNotes: unsafe extern "C" fn(track: *mut SequenceTrack),
        pub(crate) getControlSignalCount: unsafe extern "C" fn(track: *mut SequenceTrack) -> i32,
        pub(crate) getControlSignal:
            unsafe extern "C" fn(track: *mut SequenceTrack, idx: i32) -> *mut ControlSignal,
        pub(crate) clearControlEvents: unsafe extern "C" fn(track: *mut SequenceTrack),
        pub(crate) getPolyphony: unsafe extern "C" fn(track: *mut SequenceTrack) -> i32,
        pub(crate) activeVoiceCount: unsafe extern "C" fn(track: *mut SequenceTrack) -> i32,
        pub(crate) setMuted: unsafe extern "C" fn(track: *mut SequenceTrack, mute: i32),
        pub(crate) getLength: unsafe extern "C" fn(track: *mut SequenceTrack) -> u32,
        pub(crate) getIndexForStep:
            unsafe extern "C" fn(track: *mut SequenceTrack, step: u32) -> i32,
        pub(crate) getNoteAtIndex: unsafe extern "C" fn(
            track: *mut SequenceTrack,
            index: i32,
            outStep: *mut u32,
            outLen: *mut u32,
            outNote: *mut MIDINote,
            outVelocity: *mut f32,
        ) -> i32,
        pub(crate) getSignalForController: unsafe extern "C" fn(
            track: *mut SequenceTrack,
            controller: i32,
            create: i32,
        ) -> *mut ControlSignal,
    }
}

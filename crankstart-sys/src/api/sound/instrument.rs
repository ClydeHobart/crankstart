use crate::{
    define_crankstart_api, playdate_sound_instrument, MIDINote, PDSynth, PDSynthInstrument,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct InstrumentAPI => playdate_sound_instrument {
        ; // No sub-API fields
        pub newInstrument: unsafe extern "C" fn() -> *mut PDSynthInstrument,
        pub freeInstrument: unsafe extern "C" fn(inst: *mut PDSynthInstrument),
        pub addVoice: unsafe extern "C" fn(
            inst: *mut PDSynthInstrument,
            synth: *mut PDSynth,
            rangeStart: MIDINote,
            rangeEnd: MIDINote,
            transpose: f32,
        ) -> i32,
        pub playNote: unsafe extern "C" fn(
            inst: *mut PDSynthInstrument,
            frequency: f32,
            vel: f32,
            len: f32,
            when: u32,
        ) -> *mut PDSynth,
        pub playMIDINote: unsafe extern "C" fn(
            inst: *mut PDSynthInstrument,
            note: MIDINote,
            vel: f32,
            len: f32,
            when: u32,
        ) -> *mut PDSynth,
        pub setPitchBend: unsafe extern "C" fn(inst: *mut PDSynthInstrument, bend: f32),
        pub setPitchBendRange: unsafe extern "C" fn(inst: *mut PDSynthInstrument, halfSteps: f32),
        pub setTranspose: unsafe extern "C" fn(inst: *mut PDSynthInstrument, halfSteps: f32),
        pub noteOff: unsafe extern "C" fn(inst: *mut PDSynthInstrument, note: MIDINote, when: u32),
        pub allNotesOff: unsafe extern "C" fn(inst: *mut PDSynthInstrument, when: u32),
        pub setVolume: unsafe extern "C" fn(inst: *mut PDSynthInstrument, left: f32, right: f32),
        pub getVolume:
            unsafe extern "C" fn(inst: *mut PDSynthInstrument, left: *mut f32, right: *mut f32),
        pub activeVoiceCount: unsafe extern "C" fn(inst: *mut PDSynthInstrument) -> i32,
    }
}

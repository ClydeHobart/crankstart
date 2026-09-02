use crate::{
    define_crankstart_api,
    pd_api::{playdate_sound_instrument, MIDINote, PDSynth, PDSynthInstrument},
};

define_crankstart_api! {
    #[allow(dead_code, non_snake_case)]
    pub struct InstrumentAPI => playdate_sound_instrument {
        ; // No sub-API fields
        pub(crate) newInstrument: unsafe extern "C" fn() -> *mut PDSynthInstrument,
        pub(crate) freeInstrument: unsafe extern "C" fn(inst: *mut PDSynthInstrument),
        pub(crate) addVoice: unsafe extern "C" fn(
            inst: *mut PDSynthInstrument,
            synth: *mut PDSynth,
            rangeStart: MIDINote,
            rangeEnd: MIDINote,
            transpose: f32,
        ) -> i32,
        pub(crate) playNote: unsafe extern "C" fn(
            inst: *mut PDSynthInstrument,
            frequency: f32,
            vel: f32,
            len: f32,
            when: u32,
        ) -> *mut PDSynth,
        pub(crate) playMIDINote: unsafe extern "C" fn(
            inst: *mut PDSynthInstrument,
            note: MIDINote,
            vel: f32,
            len: f32,
            when: u32,
        ) -> *mut PDSynth,
        pub(crate) setPitchBend: unsafe extern "C" fn(inst: *mut PDSynthInstrument, bend: f32),
        pub(crate) setPitchBendRange:
            unsafe extern "C" fn(inst: *mut PDSynthInstrument, halfSteps: f32),
        pub(crate) setTranspose: unsafe extern "C" fn(inst: *mut PDSynthInstrument, halfSteps: f32),
        pub(crate) noteOff:
            unsafe extern "C" fn(inst: *mut PDSynthInstrument, note: MIDINote, when: u32),
        pub(crate) allNotesOff: unsafe extern "C" fn(inst: *mut PDSynthInstrument, when: u32),
        pub(crate) setVolume:
            unsafe extern "C" fn(inst: *mut PDSynthInstrument, left: f32, right: f32),
        pub(crate) getVolume:
            unsafe extern "C" fn(inst: *mut PDSynthInstrument, left: *mut f32, right: *mut f32),
        pub(crate) activeVoiceCount: unsafe extern "C" fn(inst: *mut PDSynthInstrument) -> i32,
    }
}

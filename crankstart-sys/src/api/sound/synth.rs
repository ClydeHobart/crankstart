use crate::{
    ctypes::c_void, define_crankstart_api, playdate_sound_synth, synthCopyUserdata,
    synthDeallocFunc, synthNoteOnFunc, synthReleaseFunc, synthRenderFunc, synthSetParameterFunc,
    AudioSample, MIDINote, PDSynth, PDSynthEnvelope, PDSynthSignalValue, SoundWaveform,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct SynthAPI => playdate_sound_synth {
        ; // No sub-API fields
        pub newSynth: unsafe extern "C" fn() -> *mut PDSynth,
        pub freeSynth: unsafe extern "C" fn(synth: *mut PDSynth),
        pub setWaveform: unsafe extern "C" fn(synth: *mut PDSynth, wave: SoundWaveform),
        pub setGenerator_deprecated: unsafe extern "C" fn(
            synth: *mut PDSynth,
            stereo: i32,
            render: synthRenderFunc,
            noteOn: synthNoteOnFunc,
            release: synthReleaseFunc,
            setparam: synthSetParameterFunc,
            dealloc: synthDeallocFunc,
            userdata: *mut c_void,
        ),
        pub setSample: unsafe extern "C" fn(
            synth: *mut PDSynth,
            sample: *mut AudioSample,
            sustainStart: u32,
            sustainEnd: u32,
        ),
        pub setAttackTime: unsafe extern "C" fn(synth: *mut PDSynth, attack: f32),
        pub setDecayTime: unsafe extern "C" fn(synth: *mut PDSynth, decay: f32),
        pub setSustainLevel: unsafe extern "C" fn(synth: *mut PDSynth, sustain: f32),
        pub setReleaseTime: unsafe extern "C" fn(synth: *mut PDSynth, release: f32),
        pub setTranspose: unsafe extern "C" fn(synth: *mut PDSynth, halfSteps: f32),
        pub setFrequencyModulator:
            unsafe extern "C" fn(synth: *mut PDSynth, mod_: *mut PDSynthSignalValue),
        pub getFrequencyModulator:
            unsafe extern "C" fn(synth: *mut PDSynth) -> *mut PDSynthSignalValue,
        pub setAmplitudeModulator:
            unsafe extern "C" fn(synth: *mut PDSynth, mod_: *mut PDSynthSignalValue),
        pub getAmplitudeModulator:
            unsafe extern "C" fn(synth: *mut PDSynth) -> *mut PDSynthSignalValue,
        pub getParameterCount: unsafe extern "C" fn(synth: *mut PDSynth) -> i32,
        pub setParameter: unsafe extern "C" fn(
            synth: *mut PDSynth,
            parameter: i32,
            value: f32,
        ) -> i32,
        pub setParameterModulator: unsafe extern "C" fn(
            synth: *mut PDSynth,
            parameter: i32,
            mod_: *mut PDSynthSignalValue,
        ),
        pub getParameterModulator: unsafe extern "C" fn(
            synth: *mut PDSynth,
            parameter: i32,
        ) -> *mut PDSynthSignalValue,
        pub playNote:
            unsafe extern "C" fn(synth: *mut PDSynth, freq: f32, vel: f32, len: f32, when: u32),
        pub playMIDINote: unsafe extern "C" fn(
            synth: *mut PDSynth,
            note: MIDINote,
            vel: f32,
            len: f32,
            when: u32
        ),
        pub noteOff: unsafe extern "C" fn(synth: *mut PDSynth, when: u32),
        pub stop: unsafe extern "C" fn(synth: *mut PDSynth),
        pub setVolume: unsafe extern "C" fn(synth: *mut PDSynth, left: f32, right: f32),
        pub getVolume: unsafe extern "C" fn(synth: *mut PDSynth, left: *mut f32, right: *mut f32),
        pub isPlaying: unsafe extern "C" fn(synth: *mut PDSynth) -> i32,
        pub getEnvelope: unsafe extern "C" fn(synth: *mut PDSynth) -> *mut PDSynthEnvelope,
        pub setWavetable: unsafe extern "C" fn(
            synth: *mut PDSynth,
            sample: *mut AudioSample,
            log2size: i32,
            columns: i32,
            rows: i32,
        ) -> i32,
        pub setGenerator: unsafe extern "C" fn(
            synth: *mut PDSynth,
            stereo: i32,
            render: synthRenderFunc,
            noteOn: synthNoteOnFunc,
            release: synthReleaseFunc,
            setparam: synthSetParameterFunc,
            dealloc: synthDeallocFunc,
            copyUserdata: synthCopyUserdata,
            userdata: *mut c_void,
        ),
        pub copy: unsafe extern "C" fn(synth: *mut PDSynth) -> *mut PDSynth,
    }
}

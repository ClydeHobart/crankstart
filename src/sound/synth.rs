use crate::{
    define_crankstart_api,
    pd_api::{
        ctypes::c_void, playdate_sound_synth, synthCopyUserdata, synthDeallocFunc, synthNoteOnFunc,
        synthReleaseFunc, synthRenderFunc, synthSetParameterFunc, AudioSample, MIDINote, PDSynth,
        PDSynthEnvelope, PDSynthSignalValue, SoundWaveform,
    },
};

define_crankstart_api! {
    pub struct SynthAPI => playdate_sound_synth {
        ; // No sub-API fields
        pub(crate) newSynth: unsafe extern "C" fn() -> *mut PDSynth,
        pub(crate) freeSynth: unsafe extern "C" fn(synth: *mut PDSynth),
        pub(crate) setWaveform: unsafe extern "C" fn(synth: *mut PDSynth, wave: SoundWaveform),
        pub(crate) setGenerator_deprecated: unsafe extern "C" fn(
            synth: *mut PDSynth,
            stereo: i32,
            render: synthRenderFunc,
            noteOn: synthNoteOnFunc,
            release: synthReleaseFunc,
            setparam: synthSetParameterFunc,
            dealloc: synthDeallocFunc,
            userdata: *mut c_void,
        ),
        pub(crate) setSample: unsafe extern "C" fn(
            synth: *mut PDSynth,
            sample: *mut AudioSample,
            sustainStart: u32,
            sustainEnd: u32,
        ),
        pub(crate) setAttackTime: unsafe extern "C" fn(synth: *mut PDSynth, attack: f32),
        pub(crate) setDecayTime: unsafe extern "C" fn(synth: *mut PDSynth, decay: f32),
        pub(crate) setSustainLevel: unsafe extern "C" fn(synth: *mut PDSynth, sustain: f32),
        pub(crate) setReleaseTime: unsafe extern "C" fn(synth: *mut PDSynth, release: f32),
        pub(crate) setTranspose: unsafe extern "C" fn(synth: *mut PDSynth, halfSteps: f32),
        pub(crate) setFrequencyModulator:
            unsafe extern "C" fn(synth: *mut PDSynth, mod_: *mut PDSynthSignalValue),
        pub(crate) getFrequencyModulator:
            unsafe extern "C" fn(synth: *mut PDSynth) -> *mut PDSynthSignalValue,
        pub(crate) setAmplitudeModulator:
            unsafe extern "C" fn(synth: *mut PDSynth, mod_: *mut PDSynthSignalValue),
        pub(crate) getAmplitudeModulator:
            unsafe extern "C" fn(synth: *mut PDSynth) -> *mut PDSynthSignalValue,
        pub(crate) getParameterCount: unsafe extern "C" fn(synth: *mut PDSynth) -> i32,
        pub(crate) setParameter: unsafe extern "C" fn(
            synth: *mut PDSynth,
            parameter: i32,
            value: f32,
        ) -> i32,
        pub(crate) setParameterModulator: unsafe extern "C" fn(
            synth: *mut PDSynth,
            parameter: i32,
            mod_: *mut PDSynthSignalValue,
        ),
        pub(crate) getParameterModulator: unsafe extern "C" fn(
            synth: *mut PDSynth,
            parameter: i32,
        ) -> *mut PDSynthSignalValue,
        pub(crate) playNote:
            unsafe extern "C" fn(synth: *mut PDSynth, freq: f32, vel: f32, len: f32, when: u32),
        pub(crate) playMIDINote: unsafe extern "C" fn(
            synth: *mut PDSynth,
            note: MIDINote,
            vel: f32,
            len: f32,
            when: u32
        ),
        pub(crate) noteOff: unsafe extern "C" fn(synth: *mut PDSynth, when: u32),
        pub(crate) stop: unsafe extern "C" fn(synth: *mut PDSynth),
        pub(crate) setVolume: unsafe extern "C" fn(synth: *mut PDSynth, left: f32, right: f32),
        pub(crate) getVolume:
            unsafe extern "C" fn(synth: *mut PDSynth, left: *mut f32, right: *mut f32),
        pub(crate) isPlaying: unsafe extern "C" fn(synth: *mut PDSynth) -> i32,
        pub(crate) getEnvelope: unsafe extern "C" fn(synth: *mut PDSynth) -> *mut PDSynthEnvelope,
        pub(crate) setWavetable: unsafe extern "C" fn(
            synth: *mut PDSynth,
            sample: *mut AudioSample,
            log2size: i32,
            columns: i32,
            rows: i32,
        ) -> i32,
        pub(crate) setGenerator: unsafe extern "C" fn(
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
        pub(crate) copy: unsafe extern "C" fn(synth: *mut PDSynth) -> *mut PDSynth,
    }
}

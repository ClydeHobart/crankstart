use crate::{define_crankstart_api, playdate_sound_envelope, MIDINote, PDSynthEnvelope};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct EnvelopeAPI => playdate_sound_envelope {
        ; // No sub-API fields
        pub newEnvelope: unsafe extern "C" fn(
            attack: f32,
            decay: f32,
            sustain: f32,
            release: f32,
        ) -> *mut PDSynthEnvelope,
        pub freeEnvelope: unsafe extern "C" fn(env: *mut PDSynthEnvelope),
        pub setAttack: unsafe extern "C" fn(env: *mut PDSynthEnvelope, attack: f32),
        pub setDecay: unsafe extern "C" fn(env: *mut PDSynthEnvelope, decay: f32),
        pub setSustain: unsafe extern "C" fn(env: *mut PDSynthEnvelope, sustain: f32),
        pub setRelease: unsafe extern "C" fn(env: *mut PDSynthEnvelope, release: f32),
        pub setLegato: unsafe extern "C" fn(env: *mut PDSynthEnvelope, flag: i32),
        pub setRetrigger: unsafe extern "C" fn(lfo: *mut PDSynthEnvelope, flag: i32),
        pub getValue: unsafe extern "C" fn(env: *mut PDSynthEnvelope) -> f32,
        pub setCurvature: unsafe extern "C" fn(env: *mut PDSynthEnvelope, amount: f32),
        pub setVelocitySensitivity: unsafe extern "C" fn(env: *mut PDSynthEnvelope, velsens: f32),
        pub setRateScaling: unsafe extern "C" fn(
            env: *mut PDSynthEnvelope,
            scaling: f32,
            start: MIDINote,
            end: MIDINote,
        ),
    }
}

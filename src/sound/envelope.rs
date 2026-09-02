use crate::{
    define_crankstart_api,
    pd_api::{playdate_sound_envelope, MIDINote, PDSynthEnvelope},
};

define_crankstart_api! {
    #[allow(dead_code, non_snake_case)]
    pub struct EnvelopeAPI => playdate_sound_envelope {
        ; // No sub-API fields
        pub(crate) newEnvelope: unsafe extern "C" fn(
            attack: f32,
            decay: f32,
            sustain: f32,
            release: f32,
        ) -> *mut PDSynthEnvelope,
        pub(crate) freeEnvelope: unsafe extern "C" fn(env: *mut PDSynthEnvelope),
        pub(crate) setAttack: unsafe extern "C" fn(env: *mut PDSynthEnvelope, attack: f32),
        pub(crate) setDecay: unsafe extern "C" fn(env: *mut PDSynthEnvelope, decay: f32),
        pub(crate) setSustain: unsafe extern "C" fn(env: *mut PDSynthEnvelope, sustain: f32),
        pub(crate) setRelease: unsafe extern "C" fn(env: *mut PDSynthEnvelope, release: f32),
        pub(crate) setLegato: unsafe extern "C" fn(env: *mut PDSynthEnvelope, flag: i32),
        pub(crate) setRetrigger: unsafe extern "C" fn(lfo: *mut PDSynthEnvelope, flag: i32),
        pub(crate) getValue: unsafe extern "C" fn(env: *mut PDSynthEnvelope) -> f32,
        pub(crate) setCurvature: unsafe extern "C" fn(env: *mut PDSynthEnvelope, amount: f32),
        pub(crate) setVelocitySensitivity:
            unsafe extern "C" fn(env: *mut PDSynthEnvelope, velsens: f32),
        pub(crate) setRateScaling: unsafe extern "C" fn(
            env: *mut PDSynthEnvelope,
            scaling: f32,
            start: MIDINote,
            end: MIDINote,
        ),
    }
}

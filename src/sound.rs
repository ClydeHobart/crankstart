use {
    self::{
        channel::ChannelAPI, control_signal::ControlSignalAPI, effect::EffectAPI,
        envelope::EnvelopeAPI, file_player::FilePlayerAPI, instrument::InstrumentAPI, lfo::LFOAPI,
        sample::SampleAPI, sample_player::SamplePlayerAPI, sequence::SequenceAPI,
        signal::SignalAPI, source::SourceAPI, synth::SynthAPI, track::TrackAPI,
    },
    crate::{
        define_crankstart_api,
        pd_api::{
            ctypes::{c_char, c_void},
            playdate_sound, AudioSourceFunction, MicSource, RecordCallback, SoundChannel,
            SoundSource,
        },
    },
};

pub mod channel;
pub mod control_signal;
pub mod effect;
pub mod envelope;
pub mod file_player;
pub mod instrument;
pub mod lfo;
pub mod sample;
pub mod sample_player;
pub mod sequence;
pub mod signal;
pub mod source;
pub mod synth;
pub mod track;

define_crankstart_api! {
    pub struct SoundAPI => playdate_sound {
        pub channel: ChannelAPI,
        pub controlsignal: ControlSignalAPI,
        pub effect: EffectAPI,
        pub envelope: EnvelopeAPI,
        pub fileplayer: FilePlayerAPI,
        pub instrument: InstrumentAPI,
        pub lfo: LFOAPI,
        pub sample: SampleAPI,
        pub sampleplayer: SamplePlayerAPI,
        pub sequence: SequenceAPI,
        pub signal: SignalAPI,
        pub source: SourceAPI,
        pub synth: SynthAPI,
        pub track: TrackAPI;
        pub(crate) getCurrentTime: unsafe extern "C" fn() -> u32,
        pub(crate) addSource: unsafe extern "C" fn(
            callback: AudioSourceFunction,
            context: *mut c_void,
            stereo: i32,
        ) -> *mut SoundSource,
        pub(crate) getDefaultChannel: unsafe extern "C" fn() -> *mut SoundChannel,
        pub(crate) addChannel: unsafe extern "C" fn(channel: *mut SoundChannel) -> i32,
        pub(crate) removeChannel:unsafe extern "C" fn(channel: *mut SoundChannel) -> i32,
        pub(crate) setMicCallback: unsafe extern "C" fn(
            callback: RecordCallback,
            context: *mut c_void,
            source: MicSource,
        ) -> i32,
        pub(crate) getHeadphoneState: unsafe extern "C" fn(
            headphone: *mut i32,
            headsetmic: *mut i32,
            changeCallback: Option<unsafe extern "C" fn(headphone: i32, mic: i32)>,
        ),
        pub(crate) setOutputsActive: unsafe extern "C" fn(headphone: i32, speaker: i32),
        pub(crate) removeSource: unsafe extern "C" fn(source: *mut SoundSource) -> i32,
        pub(crate) getError: unsafe extern "C" fn() -> *const c_char,
    }
}

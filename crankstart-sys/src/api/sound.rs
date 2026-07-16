use {
    self::{
        channel::ChannelAPI, control_signal::ControlSignalAPI, effect::EffectAPI,
        envelope::EnvelopeAPI, file_player::FilePlayerAPI, instrument::InstrumentAPI, lfo::LFOAPI,
        sample::SampleAPI, sample_player::SamplePlayerAPI, sequence::SequenceAPI,
        signal::SignalAPI, source::SourceAPI, synth::SynthAPI, track::TrackAPI,
    },
    crate::{
        ctypes::{c_char, c_void},
        define_crankstart_api, playdate_sound, AudioSourceFunction, MicSource, RecordCallback,
        SoundChannel, SoundSource,
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
    #[allow(dead_code)]
    pub(crate) struct SoundAPI => playdate_sound {
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
        pub getCurrentTime: unsafe extern "C" fn() -> u32,
        pub addSource: unsafe extern "C" fn(
            callback: AudioSourceFunction,
            context: *mut c_void,
            stereo: i32,
        ) -> *mut SoundSource,
        pub getDefaultChannel: unsafe extern "C" fn() -> *mut SoundChannel,
        pub addChannel: unsafe extern "C" fn(channel: *mut SoundChannel) -> i32,
        pub removeChannel:unsafe extern "C" fn(channel: *mut SoundChannel) -> i32,
        pub setMicCallback: unsafe extern "C" fn(
            callback: RecordCallback,
            context: *mut c_void,
            source: MicSource,
        ) -> i32,
        pub getHeadphoneState: unsafe extern "C" fn(
            headphone: *mut i32,
            headsetmic: *mut i32,
            changeCallback: Option<unsafe extern "C" fn(headphone: i32, mic: i32)>,
        ),
        pub setOutputsActive: unsafe extern "C" fn(headphone: i32, speaker: i32),
        pub removeSource: unsafe extern "C" fn(source: *mut SoundSource) -> i32,
        pub getError: unsafe extern "C" fn() -> *const c_char,
    }
}

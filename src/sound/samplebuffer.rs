use super::AudioSample;
use crate::{alloc::rc::Rc, log_to_console, pd_func_caller, pd_func_caller_log};
use crankstart_sys::{ctypes, SoundFormat};

use anyhow::{anyhow, ensure, Error, Result};
use core::{mem::size_of, slice};

#[derive(Debug)]
pub struct SampleBuffer {
    raw_subsystem: *const crankstart_sys::playdate_sound_sample,
    raw_sample: *mut crankstart_sys::AudioSample,
    byte_count: usize,
    sample_rate: u32,
    format: SoundFormat,
}

impl SampleBuffer {
    pub(crate) fn new(
        raw_subsystem: *const crankstart_sys::playdate_sound_sample,
        raw_sample: *mut crankstart_sys::AudioSample,
        byte_count: usize,
        sample_rate: u32,
        format: SoundFormat,
    ) -> Result<Self> {
        ensure!(
            !raw_subsystem.is_null(),
            "Null pointer given as subsystem to SampleBuffer::new"
        );
        ensure!(
            !raw_sample.is_null(),
            "Null pointer given as player to SampleBuffer::new"
        );
        ensure!(
            Self::is_format_supported(format),
            "Unsupported format {format:?} given to SampleBuffer::new"
        );
        ensure!(
            byte_count % Self::frame_size(format) == 0_usize,
            "Byte count {byte_count} isn't divisible by frame size {0} for format {format:?} given \
                to SampleBuffer::new",
            Self::frame_size(format)
        );
        Ok(Self {
            raw_subsystem,
            raw_sample,
            byte_count,
            sample_rate,
            format,
        })
    }

    pub(crate) fn copy_from_data(&mut self, data: &[u8]) -> Result<()> {
        ensure!(
            data.len() == self.byte_count,
            "SampleBuffer::try_copy_from_data called with slice of incorrect length {0}",
            data.len()
        );

        let data_mut: &mut [u8] = unsafe {
            slice::from_raw_parts_mut(
                self.raw_sample as *mut u8,
                self.byte_count / size_of::<u8>(),
            )
        };

        data_mut.copy_from_slice(data);

        Ok(())
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Allocates an `AudioSample` with the contents of `self`.
    pub fn as_audio_sample(&self) -> Result<AudioSample> {
        let raw_audio_sample: *mut crankstart_sys::AudioSample = pd_func_caller!(
            (*self.raw_subsystem).newSampleFromData,
            self.raw_sample as *mut u8,
            self.format,
            self.sample_rate,
            self.byte_count as ctypes::c_int,
            false as ctypes::c_int
        )?;
        ensure!(
            !raw_audio_sample.is_null(),
            "Null returned from sample.newSampleFromData"
        );
        AudioSample::new(self.raw_subsystem, raw_audio_sample)
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
    }
}

macro_rules! define_format_specific_definitions {
    ( $( $frame:ident: {
        $bit_depth:ty,
        $channel_count:expr,
        $sound_format:ident,
        $try_get_frames:ident,
        $try_get_frames_mut:ident $(,)?
    } ),* $(,)? ) => {
        $(
            pub type $frame = Frame<$bit_depth, $channel_count>;
        )*

        impl SampleBuffer {
            pub(crate) fn frame_size(format: SoundFormat) -> usize {
                assert!(Self::is_format_supported(format));

                match format {
                    $( SoundFormat::$sound_format => size_of::<$frame>(), )*
                    _ => unreachable!()
                }
            }

            pub fn is_format_supported(format: SoundFormat) -> bool {
                match format {
                    $( SoundFormat::$sound_format => true, )*
                    _ => false
                }
            }

            $(
                pub fn $try_get_frames(&self) -> Option<&[$frame]> {
                    (self.format == SoundFormat::$sound_format).then(|| unsafe {
                        slice::from_raw_parts(
                            self.raw_sample as *const $frame,
                            self.byte_count / size_of::<$frame>()
                        )
                    })
                }

                pub fn $try_get_frames_mut(&mut self) -> Option<&mut [$frame]> {
                    (self.format == SoundFormat::$sound_format).then(|| unsafe {
                        slice::from_raw_parts_mut(
                            self.raw_sample as *mut $frame,
                            self.byte_count / size_of::<$frame>()
                        )
                    })
                }
            )*
        }
    }
}

pub trait BitDepthFormat {
    type RawInteger;
}

pub struct BitDepth8;

impl BitDepthFormat for BitDepth8 {
    type RawInteger = i8;
}

pub struct BitDepth16;

impl BitDepthFormat for BitDepth16 {
    type RawInteger = i16;
}

// Once `feature(generic_const_expr)` is finalized, it might be worth putting these behind types
// with a unifying `ChannelFormat` trait.
pub const MONO_CHANNEL_COUNT: usize = 1_usize;
pub const STEREO_CHANNEL_COUNT: usize = 2_usize;

pub type Frame<B, const CHANNEL_COUNT: usize> = [<B as BitDepthFormat>::RawInteger; CHANNEL_COUNT];

pub trait MonoFrame {
    type RawInteger;

    fn flatten(&self) -> &[Self::RawInteger];

    fn flatten_mut(&mut self) -> &mut [Self::RawInteger];
}

impl<I> MonoFrame for [[I; MONO_CHANNEL_COUNT]] {
    type RawInteger = I;

    fn flatten(&self) -> &[Self::RawInteger] {
        // SAFETY: This is a slice of arrays of capacity 1.
        unsafe { slice::from_raw_parts(self.as_ptr() as *const Self::RawInteger, self.len()) }
    }

    fn flatten_mut(&mut self) -> &mut [Self::RawInteger] {
        // SAFETY: This is a slice of arrays of capacity 1.
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr() as *mut Self::RawInteger, self.len()) }
    }
}

define_format_specific_definitions!(
    Frame8BitMono: {
        BitDepth8,
        MONO_CHANNEL_COUNT,
        kSound8bitMono,
        try_get_frames_8_bit_mono,
        try_get_frames_8_bit_mono_mut,
    },
    Frame8BitStereo: {
        BitDepth8,
        STEREO_CHANNEL_COUNT,
        kSound8bitStereo,
        try_get_frames_8_bit_stereo,
        try_get_frames_8_bit_stereo_mut,
    },
    Frame16BitMono: {
        BitDepth16,
        MONO_CHANNEL_COUNT,
        kSound16bitMono,
        try_get_frames_16_bit_mono,
        try_get_frames_16_bit_mono_mut,
    },
    Frame16BitStereo: {
        BitDepth16,
        STEREO_CHANNEL_COUNT,
        kSound16bitStereo,
        try_get_frames_16_bit_stereo,
        try_get_frames_16_bit_stereo_mut,
    },
);

impl Drop for SampleBuffer {
    fn drop(&mut self) {
        // Use _log to leak rather than fail
        pd_func_caller_log!((*self.raw_subsystem).freeSample, self.raw_sample);
    }
}

use std::{fs::File, io::BufReader};

use hound::WavReader;

use crate::{audio::audiofile::AudioFile, wavreader::WavSampleReader};

const VOLUME_STEPS: usize = 10000;

#[derive(Clone, Copy, PartialEq)]
pub enum SampleFormat {
    F32,
    I32,
    I24,
    I16,
    I8,
}

impl From<(hound::SampleFormat, u16)> for SampleFormat {
    fn from(x: (hound::SampleFormat, u16)) -> Self {
        match x {
            (hound::SampleFormat::Float, 32) => Self::F32,
            (hound::SampleFormat::Int, 32) => Self::I32,
            (hound::SampleFormat::Int, 24) => Self::I24,
            (hound::SampleFormat::Int, 16) => Self::I16,
            (hound::SampleFormat::Int, 8) => Self::I8,
            (x, y) => {
                let s = match x {
                    hound::SampleFormat::Float => "F",
                    hound::SampleFormat::Int => "I",
                };

                panic!("Sample format not supported: {}{}", s, y)
            }
        }
    }
}

pub struct AudioTrack {
    name: String,
    sample_format: SampleFormat,
    #[allow(dead_code)]
    sample_rate: u32,
    channels: u16,
    holder: WavReader<BufReader<File>>,
    ctx: TrackCtx,
}

impl AudioTrack {
    pub fn new(name: &str, file: AudioFile) -> Self {
        let holder = WavReader::new(file.clone().reader).unwrap();

        let spec = holder.spec();
        let sf = SampleFormat::from((spec.sample_format, spec.bits_per_sample));
        let sr = spec.sample_rate;
        let ch = spec.channels;

        Self {
            name: name.to_string(),
            sample_format: sf,
            sample_rate: sr,
            channels: ch,
            holder,
            ctx: TrackCtx {
                current_volume: 0.8,

                volume_counter: 0,
                volume_step: 1.0,
                target_volume: 0.8,

                panning: 0.5,
            },
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn advance(&mut self) -> f32 {
        let volume_calc = |ctx: &mut TrackCtx| {
            if ctx.volume_counter > 0 {
                ctx.volume_counter -= 1;
                ctx.current_volume += ctx.volume_step;
            }
            return ctx.current_volume;
        };

        match self.create_samples().read_next() {
            Some(x) => x * volume_calc(&mut self.ctx),
            None => 0.0,
        }
    }

    pub fn set_volume(&mut self, target_volume: f32) {
        self.ctx.target_volume = target_volume;
        self.ctx.volume_step = (target_volume - self.ctx.current_volume) / VOLUME_STEPS as f32;
        self.ctx.volume_counter = VOLUME_STEPS as u16;
    }

    fn create_samples(&mut self) -> WavSampleReader {
        match self.sample_format {
            SampleFormat::F32 => WavSampleReader::F32(self.holder.samples::<f32>()),
            SampleFormat::I32 => WavSampleReader::I32(self.holder.samples::<i32>()),
            SampleFormat::I24 => WavSampleReader::I24(self.holder.samples::<i32>()),
            SampleFormat::I16 => WavSampleReader::I16(self.holder.samples::<i16>()),
            SampleFormat::I8 => WavSampleReader::I8(self.holder.samples::<i8>()),
        }
    }
}

impl PartialEq for AudioTrack {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
        //&& self.sample_format == other.sample_format
        //&& self.sample_rate == other.sample_rate
        //&& self.holder == other.holder
        //&& self.volume == other.volume
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TrackCtx {
    // Volume
    current_volume: f32,

    // Linear Volume Smoothing
    target_volume: f32,
    volume_step: f32,
    volume_counter: u16,

    panning: f32,
}

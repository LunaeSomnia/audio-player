use std::{fs::File, io::BufReader};

use hound::WavReader;

use crate::{audio_file::AudioFile, wavreader::WavSampleReader};

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
    sample_format: SampleFormat,
    #[allow(dead_code)]
    sample_rate: u32,
    holder: WavReader<BufReader<File>>,
}

impl AudioTrack {
    pub fn new(file: AudioFile) -> Self {
        let holder = WavReader::new(file.clone().reader).unwrap();

        let spec = holder.spec();
        let sf = SampleFormat::from((spec.sample_format, spec.bits_per_sample));
        let sr = spec.sample_rate;

        Self {
            sample_format: sf,
            sample_rate: sr,
            holder,
        }
    }

    pub fn advance(&mut self) -> f32 {
        match self.create_samples().read_next() {
            Some(x) => x,
            None => 0.0,
        }
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

#[derive(Clone)]
pub struct AudioTrackHandle<'a> {
    #[allow(dead_code)]
    fileref: &'a AudioTrack,
    pub i: usize,
}

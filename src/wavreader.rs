use std::{fs::File, io::BufReader};

use hound::WavSamples;

pub enum WavSampleReader<'a> {
    F32(WavSamples<'a, BufReader<File>, f32>),
    I32(WavSamples<'a, BufReader<File>, i32>),
    I24(WavSamples<'a, BufReader<File>, i32>),
    I16(WavSamples<'a, BufReader<File>, i16>),
    I8(WavSamples<'a, BufReader<File>, i8>),
}

impl WavSampleReader<'_> {
    pub fn read_next(&mut self) -> Option<f32> {
        match self {
            WavSampleReader::F32(x) => match x.next() {
                Some(n) => Some(n.unwrap()),
                None => None,
            },
            WavSampleReader::I32(x) => match x.next() {
                Some(n) => Some(n.unwrap() as f32 / 0x7FFFFFFF as f32),
                None => None,
            },
            WavSampleReader::I24(x) => match x.next() {
                Some(n) => Some(n.unwrap() as f32 / 0x7FFFFF as f32),
                None => None,
            },
            WavSampleReader::I16(x) => match x.next() {
                Some(n) => Some(n.unwrap() as f32 / 0x7FFF as f32),
                None => None,
            },
            WavSampleReader::I8(x) => match x.next() {
                Some(n) => Some(n.unwrap() as f32 / 0x7F as f32),
                None => None,
            },
        }
    }
}

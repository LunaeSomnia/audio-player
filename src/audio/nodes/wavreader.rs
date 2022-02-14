use std::{collections::HashMap, fs::File, io::BufReader};

use crate::audio::{sample_format::SampleFormat, NodeId, PortId};

use super::Node;

pub struct WavReader<'a> {
    id: NodeId,

    inputs: HashMap<String, PortId>,
    outputs: HashMap<String, PortId>,

    path: String,

    reader: Option<hound::WavReader<BufReader<File>>>,
    samples: Option<WavSampleReader<'a>>,
}

impl Node for WavReader<'_> {
    fn new(id: NodeId) -> Self
    where
        Self: Sized,
    {
        Self {
            id,

            inputs: Default::default(),
            outputs: Default::default(),

            path: Default::default(),

            reader: None,
            samples: None,
        }
    }

    fn id(&self) -> NodeId {
        self.id
    }

    fn process(
        &self,
        _inputs: &HashMap<PortId, &[f32]>,
        outputs: &mut HashMap<PortId, &mut [f32]>,
    ) {
        let output_id = self.outputs.get("out").unwrap();

        let out = outputs.get_mut(&output_id).unwrap();

        if let Some(reader) = &self.reader {
            todo!()
        }
    }
}

impl WavReader<'_> {
    pub fn open(&mut self, path: &str) -> Result<(), String> {
        let file = File::open(path);
        if let Err(err) = file {
            return Err(err.to_string());
        }
        let buf_reader = BufReader::new(file.unwrap());

        let wav_reader_r = hound::WavReader::new(buf_reader);
        if let Err(err) = wav_reader_r {
            return Err(err.to_string());
        }

        let wav_reader = wav_reader_r.unwrap();

        let spec = wav_reader.spec();

        let sample_format: SampleFormat = (spec.sample_format, spec.bits_per_sample).into();

        self.reader = Some(wav_reader);
        self.samples = WavS
    }

    pub fn next(&mut self) -> &[f32] {
        todo!()
    }
}

pub enum WavSampleReader<'a> {
    F32(hound::WavSamples<'a, BufReader<File>, f32>),
    I32(hound::WavSamples<'a, BufReader<File>, i32>),
    I24(hound::WavSamples<'a, BufReader<File>, i32>),
    I16(hound::WavSamples<'a, BufReader<File>, i16>),
    I8(hound::WavSamples<'a, BufReader<File>, i8>),
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

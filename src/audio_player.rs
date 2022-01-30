use rb::{Producer, RbProducer};

use crate::audio_track::AudioTrack;

pub struct AudioPlayer {
    tracks: Vec<AudioTrack>,
    output: Producer<f32>,
}

impl AudioPlayer {
    pub fn new(output: Producer<f32>) -> Self {
        Self {
            tracks: Vec::new(),
            output,
        }
    }

    pub fn attach(&mut self, track: AudioTrack) {
        self.tracks.push(track);
    }

    pub fn advance(&mut self) {
        let mut sum: f32 = 0.0;

        for tr in self.tracks.iter_mut() {
            sum += tr.advance();
        }

        self.output.write_blocking(&[sum]);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioPlayerError {
    IOError(String),
    HoundError(String),
}

impl From<std::io::Error> for AudioPlayerError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err.to_string())
    }
}

impl From<hound::Error> for AudioPlayerError {
    fn from(err: hound::Error) -> Self {
        Self::HoundError(err.to_string())
    }
}

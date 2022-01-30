use rb::{Producer, RbProducer};

use crate::audio_track::{AudioTrack, AudioTrackHandler};

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

    pub fn attach(&mut self, track: AudioTrack) -> bool {
        if !self.tracks.contains(&track) {
            self.tracks.push(track);
            true
        } else {
            false
        }
    }

    pub fn get(&mut self, track_name: &str) -> Option<&mut AudioTrack> {
        self.tracks
            .iter_mut()
            .find(|t| t.name() == track_name.to_string())
    }

    pub fn advance(&mut self) -> bool {
        let mut sum: f32 = 0.0;

        for tr in self.tracks.iter_mut() {
            sum += tr.advance();
        }

        // If we could advance, return true
        if let None = self.output.write_blocking(&[sum]) {
            false
        } else {
            true
        }
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

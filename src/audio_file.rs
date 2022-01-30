use std::{fs::File, io::BufReader};

#[derive(Debug)]
pub struct AudioFile {
    pub reader: BufReader<File>,
    path: String,
}

impl AudioFile {
    pub fn open(path: &str) -> Result<Self, AudioFileError> {
        Ok(Self {
            reader: BufReader::new(File::open(path)?),
            path: path.to_string(),
        })
    }
}

impl Clone for AudioFile {
    fn clone(&self) -> Self {
        Self {
            reader: BufReader::new(File::open(&self.path).unwrap()),
            path: self.path.clone(),
        }
    }
}

impl PartialEq for AudioFile {
    fn eq(&self, other: &Self) -> bool {
        //self.reader == other.reader &&
        self.path == other.path
    }
}

#[derive(Debug)]
pub enum AudioFileError {
    IOError(String),
}

impl From<std::io::Error> for AudioFileError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err.to_string())
    }
}

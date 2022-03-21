use crate::client::vault::{Track};

use std::net::TcpStream;
use std::thread;
use std::fs;
use std::io::{
    BufReader,
    Error,
    ErrorKind,
};
use std::path::Path;
use std::collections::{hash_map::DefaultHasher, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::fmt;
use std::sync::{Arc, Mutex};


use lofty::{id3, mp3, Probe, Tag, TagItem, TaggedFile};
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

use reqwest::header::{HeaderValue, CONTENT_LENGTH, RANGE};
use reqwest::StatusCode;

#[derive(Debug)]
struct PlayerError {}

#[derive(Debug, Clone)]
struct PlaybackError;

impl fmt::Display for PlaybackError {
    fn fmt(&self, f: &mut fmt::Formatter) {
        write!(f, "invalid track file.");
    } 
}

enum PlaybackOrderSetting {
    Straight, // DEFAULT
    Shuffle, 
    RepeatTrack,
    RepeatPlaylist,
}

enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

struct Player {
    current_playback: PlaybackStatus,
    current_playmode: PlaybackOrderSetting,
    playback_queue: Arc<Mutex<Vec<Track>>>,
    output_stream: Option<Sink>,
}

impl Player {
    pub fn new(output: OutputStream) -> Result<Self, PlayerError> {
        let (_stream, stream_handle) = OutputStream::try_default()?;     
        let sink = Sink::new_idle(&stream_handle).unwrap();
        Ok(Player {
            current_playmode: PlaybackOrderSetting::Straight, 
            output_stream: sink
        })
    }

    // Return the contents of an audio file
    pub fn load_track(&self, filepath: Path) -> Result<(), PlaybackError> {
        let f = fs::File::open(filepath).unwrap();
        let samples_buffer = BufReader::new(f).unwrap();
        let metadata = &self.load_metadata(filepath).unwrap();
        Ok(samples_buffer, metadata)
    } 
    
    // Returns the tag, 
    // if there is none, it creates and returns a new one
    pub fn load_metadata(&self, filepath: Path) -> Result<(), PlaybackError> {
        let mut tag = match Tag::new().read_from_path(filepath).unwrap() {
            Ok(tag) => tag,
            Err(err) => return Err(Box::new(err)), 
        };
        Ok(tag)
    }

    pub fn play(&self) -> Result<(), PlaybackError> {
        &self.output_stream.play();
    }

    pub fn queue(&self, track: Track) -> Result<(), PlaybackError> {
        &self.output_stream.append(track);
    }
}

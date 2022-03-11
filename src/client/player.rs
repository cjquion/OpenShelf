use std::fs;
use std::io::BufReader;
use std::path::Path;

use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

pub struct Track {
    pub title: String,
    pub artist: String,
    pub artist_code: u64, 
    pub path: Path,
}

#[derive(Debug, Clone)]
struct TrackError;

impl fmt::Display for TrackError -> fmt::Result {
    fn fmt(&self, f: &mut fmt::Formatter) {
        write!(f, "invalid track file.");
    } 
}

impl Track {
    fn new(path: Path) -> Result<Track, TrackError> {
    
    }
}

#[derive(Debug, Clone)]
struct PlaybackError;

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

#[derive(Debug)]
struct PlayerError {}

impl Player {
    pub fn new(output: OutputStream) -> Result<Self, PlayerError> {
        let (_stream, stream_handle) = OutputStream::try_default()?;     
        let sink = Sink::new_idle(&stream_handle).unwrap();
        Ok(Player {
            current_playmode: PlaybackOrderSetting::Straight, 
            output_stream: sink
        })
    }

    pub fn load_track(&self, filepath: Path) -> Result<(), PlaybackError> {
        let f = fs::File::open(filepath).unwrap();
        let samples_buffer = BufReader::new(f).unwrap();
        let metadata = &self.load_metadata(filepath).unwrap();
        Ok(samples_buffer, metadata)
    } 
    
    pub fn load_metadata(&self, filepath: Path) -> Result<(), PlaybackError> {
        let mut tag = match id3::read_from_tag(filepath).unwrap() {
            Ok(tag) => tag,
            Err(Error{kind: ErrorKind::NoTag, ..}) => id3::Tag::new(),
            Err(err) => return Err(Box::new(err)), 
        }
        Ok(tag)
    }

    pub fn play(&self) -> Result<(), PlaybackError> {
        &self.output_stream.play();
    }

    pub fn queue(&self, track: Track) -> Result<(), PlaybackError> {
        &self.output_stream.append(track);
    }
}


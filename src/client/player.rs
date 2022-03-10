use std::path::Path;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};
pub struct Track {}

#[derive(Debug, Clone)]
struct PlaybackError;

enum PlaybackOrderSetting {
    Straight,
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
    currently_playing: Option<Track>,
    current_playmode: PlaybackOrderSetting,
    output_stream: Option<Sink>,
}

#[derive(Debug)]
struct PlayerError {}

impl Player {
    pub fn new(output: OutputStream) -> Result<Self, PlayerError> {
        let (_stream, stream_handle) = OutputStream::try_default()?;     
        let sink = Sink::new_idle(&stream_handle).unwrap();
        Ok(Player {
            current_playback: PlaybackStatus::Stopped, 
            currently_playing: None, 
            current_playmode: PlaybackOrderSetting::Straight, 
            output_stream: sink
        })
    }

    pub fn load_track(filepath: Path) -> Result<(), PlaybackError> {
        
    } 
    
    pub fn play(&self) -> Result<(), PlaybackError> {
        &self.output_stream.play();
    }

    pub fn queue(&self, track: Source) -> Result<(), PlaybackError> {
        &self.output_stream.append(track);
    }
}


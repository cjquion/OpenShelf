use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

pub struct Track {}

#[(Derive, Debug)]
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
    current_playback: PlaybackStatus
    currently_playing: Option<Track>,
    current_playmode: PlaybackOrderSetting,
    output: (OutputStream, OutputStreamHandle),
}

#[derive(Debug)]
struct PlayerError {}

impl Player {
    pub fn new(output: OutputStream) -> Result<Self, PlayerError> {
        let (_stream, stream_handle) = OutputStream::try_default()?;     
        Ok(Player {
            current_playback: playbackStatus::Stopped, 
            currently_playing: None, 
            current_playmode: PlaybackOrderSetting::Straight, 
            output: (_stream, stream_handle)
        )}
)
    }
    
    pub fn play() -> Result<(), PlaybackError> {
    
    }

}

pub fn play_track() {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let playing_track = stream_handle.play_once(BufReader::new(Track.contents));
}

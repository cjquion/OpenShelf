use std::{
    fmt, env, thread, fs, path::Path, time::Duration,
};

use std::io::{
    stdin, stdout, Write, Stdout,
    BufReader, Error, ErrorKind
};

use rodio::{
    Decoder, OutputStream, Sink
};

use rodio::source::{
    SineWave, Source
};

use lofty::{
    id3, mp3, Probe, Accessor, Tag, TagItem, TaggedFile, ItemKey
};

use termion::{
    async_stdin,
    raw::{IntoRawMode, RawTerminal},
    event::Key,
    input::TermRead,
};

#[derive(Debug)]
struct PlaybackError {}

// Work around for fmt::Display
pub struct TrackDuration(pub Duration);

#[derive(Debug)]
enum Commands {
    Play,
    Pause,
    Stop,
    SkipFoward,
    SkipBackward,
}


enum PlaybackOrderSetting {
    Straight,
    Shuffle
}

#[derive(PartialEq, Debug)]
enum PlaybackStatus {
    Playing,
    Paused, 
    Stopped
}

struct Track {
    artist: String,
    title: String,
}

struct Player {
    current_playback_status: PlaybackStatus,
    current_playback_order: PlaybackOrderSetting,
    playback_queue: Vec<Track>,
    output_sink: Option<Sink>
}

impl fmt::Display for TrackDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let seconds = &self.0.as_secs() % 60;
        let minutes = (&self.0.as_secs() / 60) % 60;
        let hours = (&self.0.as_secs() / 60) / 60;
        write!(f, "{}:{}:{}", hours, minutes, seconds)
    }
}

impl Player {
    pub fn new() -> Player { 
        let current_playback_status = PlaybackStatus::Stopped;
        let current_playback_order = PlaybackOrderSetting::Straight;
        let playback_queue = Vec::new();

        Player {
            current_playback_status,
            current_playback_order,
            playback_queue,
            output_sink: None,
        }
    }

    pub fn play(&mut self, track_path: &Path) -> Result<(), PlaybackError> {
        // Set up devices and the handlers
        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok((_stream, stream_handle)) => (_stream, stream_handle),
            Err(rodio::StreamError::NoDevice) => panic!("No audio device detected."),
            _ => panic!("Error setting up output stream"),
        };
        let sink = Sink::try_new(&stream_handle).unwrap();
        
        // Open the track and load it into an audio buffer
        let track = fs::File::open(track_path).unwrap();
        let source = rodio::Decoder::new(BufReader::new(track)).unwrap();

        // Open the track's metadata
        let track_metadata = Probe::open(track_path).unwrap()
            .read(true).unwrap(); // true means dont zero out the properties
        let metadata_tag = match track_metadata.primary_tag() { 
            Some(primary_tag) => primary_tag,
            None => track_metadata.first_tag().unwrap(),
        };
        let properties = track_metadata.properties();


        // Extract the basic data
        let artist = match metadata_tag.artist() {
            Some(artist) => artist,
            None => "Unknown" 
        };
        let title = match metadata_tag.album() { 
            Some(title) => title,
            None => "Unknown"
        };
        let duration = properties.duration();
       
        // Set up the terminal for io,
        // then display basic info about the track as well as usage.
        // async_stdin().keys() returns an iterator. 
        let mut stdin = async_stdin().keys();
        let mut stdout = stdout().into_raw_mode().unwrap();
        
        write!(stdout,
            "{}{}press p to toggle pause/play. {}press q to quit.",
            termion::clear::All,
            termion::cursor::Goto(1,1),
            termion::cursor::Goto(1,2),
        ).unwrap();
        stdout.flush().unwrap();
        write!(stdout,
            "{}currently: {:?}",
            termion::cursor::Goto(1,4),
            self.current_playback_status,
        ).unwrap();
        stdout.flush().unwrap();

        write!(stdout, 
            "{}playing {} by {}. {}.{}",
            termion::cursor::Goto(1,3),
            artist,    
            title,
            TrackDuration(duration),
            termion::cursor::Hide,
        ).unwrap();
        stdout.flush().unwrap();
        
        // Play the song!
        sink.append(source);
        self.current_playback_status = PlaybackStatus::Playing;
        write!(stdout,
            "{}currently: {:?}",
            termion::cursor::Goto(1,4),
            self.current_playback_status,
        ).unwrap();
        stdout.flush().unwrap();
        
        // Listen for keyboard events
        'playing: loop {
            let key = stdin.next();
            match key { 
                Some(Ok(Key::Char('q'))) => {
                    write!(stdout, "Quit command received. Stopping playback.");
                    break Ok(())
                },
                Some(Ok(Key::Char('p'))) => {
                    if self.current_playback_status == PlaybackStatus::Playing {
                        self.current_playback_status = PlaybackStatus::Paused;
                        sink.pause();
                        write!(stdout,
                            "{}{}currently: {:?}",
                            termion::clear::CurrentLine,
                            termion::cursor::Goto(1,4),
                            self.current_playback_status,
                        ).unwrap();
                        stdout.flush().unwrap();
                    } else if self.current_playback_status == PlaybackStatus::Paused {
                        self.current_playback_status = PlaybackStatus::Playing;
                        sink.play();
                        write!(stdout,
                            "{}{}currently: {:?}",
                            termion::clear::CurrentLine,
                            termion::cursor::Goto(1,4),
                            self.current_playback_status,
                        ).unwrap();
                        stdout.flush().unwrap();
                    }
                }
                _ => continue,
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let target_path = Path::new(&args[1]);
        let mut player = Player::new();

        player.play(target_path);
    }
}

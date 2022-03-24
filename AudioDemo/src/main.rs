use std::{
    fmt, env, thread, fs, path::Path
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

        // Extract the basic data
        let artist = metadata_tag.artist().map(str::to_string).unwrap();
        let title = metadata_tag.album().map(str::to_string).unwrap();
       
        // Set up the terminal for io,
        // then display basic info about the track as well as usage.
        // async_stdin().keys() returns an iterator. 
        let mut stdin = async_stdin().keys();
        let mut stdout = stdout().into_raw_mode().unwrap();
        
        write!(stdout,
            "{}{}press p to toggle pause/play.{}",
            termion::clear::All,
            termion::cursor::Goto(1,1),
            termion::cursor::Hide
        ).unwrap();

        write!(stdout,
            "{}{}press q to stop playback.{}",
            termion::clear::All,
            termion::cursor::Goto(1,1),
            termion::cursor::Hide,
        ).unwrap();
        stdout.flush().unwrap();

        write!(stdout, 
            "{}{}playing {} by {}{}",
            termion::clear::All,
            termion::cursor::Goto(1,1),
            artist,    
            title,
            termion::cursor::Hide,
        ).unwrap();
        stdout.flush().unwrap();
        
        // Play the song!
        sink.append(source);
        self.current_playback_status = PlaybackStatus::Playing;
        
        // Listen for keyboard events
        'playing: loop {
            let key = stdin.next();
            match key.unwrap().unwrap() { 
                Key::Char('q') => break Ok(()),
                Key::Char('p') => break Ok(()),
                _ => println!("Key not recognized."),
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

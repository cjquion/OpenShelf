use std::{
    time, fmt, env, thread, fs, path::Path, time::Duration,
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


fn main() {
    println!("Hello, world!");
    spin_and_listen().unwrap();
}

pub fn spin_and_listen() -> Result<(), Error> {
    let mut stdin = async_stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();
        
    'playing: loop {
        let key = stdin.next();
        
        match key {
            Some(Ok(Key::Char('q'))) => {
                write!(stdout, "Quitting...");
                stdout.flush().unwrap();
                break Ok(())
            },
            Some(Ok(Key::Char('e'))) => {
                write!(stdout, "What??");
                stdout.flush().unwrap();
            },
            _ => {
                thread::sleep(time::Duration::from_millis(15));
                continue
            },
        }
    }
}

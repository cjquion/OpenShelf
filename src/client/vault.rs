use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::thread;
use std::path::Path;
use std::fmt;

use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

#[derive(Debug, Clone)]
struct VaultError;

#[derive(Debug, Clone)]
struct TrackError

impl fmt::Display for VaultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

pub struct Track { 
    pub id: u64,
    pub title: String,
    pub artist: String,
    pub filepath: Path,
    pub album: String,
    pub rating: u32,
    pub tags: Vec<String>
}

pub struct Artist {
    pub id: u64,
    pub name: String,
    pub alias: String,
    pub bio: String,
}

fn main() {
    let default_track = "test.mp3";
    let default_port = "127.0.0.1:7878";
    let vault_path = "";
    let opened_track = File::open(default_track).unwrap();
    let listener = TcpListener::bind(default_port).unwrap();
    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut rx_buffer = [0; 1024];
    stream.read(&mut rx_buffer).unwrap();
    let (status, file) = if buffer.starts_with(get) {}
}

fn register_vault(filepath: Path, vault_name: String,) -> Result<(), Error> {
    for entry in fs::read_dir(filepath).unwrap()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
        
        } else {
            register_track(path);
        }
    }
}

fn register_track(path: Path) -> Result<(), TrackError>{
        
}

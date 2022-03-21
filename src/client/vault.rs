use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::thread;
use std::path::Path;
use std::fmt;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serde::{
    Serialize, 
    Deserialize,
};
use rustbreak::{
    deser::Ron,
    FileDatabase,
};

use rodio::{
    Decoder, 
    OutputStream, 
    Sink
};
use rodio::source::{
    SineWave, 
    Source
};

use lofty::{id3, mp3, Probe, Tag, TagItem, TaggedFile, AudioFile};


#[derive(Debug, Clone)]
struct VaultError {}

#[derive(Debug, Clone)]
struct TrackError {}

impl fmt::Display for VaultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

type VAULT = FileDatabase<Config, Ron>;
pub trait Vault {
    type Data; 
    fn initialize() -> Result<Box<Self>, VaultError>;
    fn refresh(&self, path: Path) -> Result<(), VaultError>;
    fn data(&self) -> Result<Self::Data, VaultError>;
}

lazy_static! { 
    static ref CONFIG: VAULT = {
        let vault = FileDatabase::load_from_path_or_default("")
            .expect();
        vault.load().expect();
        vault
    };
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Track { 
    pub id: u64,
    pub title: Option<Path>,
    pub artist: Option<Path>,
    pub artist_code: Option<Path>,
    pub filepath: Option<Path>,
    pub album: Option<u64>,
    pub rating: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub picture: Option<Picture>,
}

impl Track {
    fn new(path: Path, album: Option<u64>, rating: Option<u32>, tags: Option<Vec<String>>, picture: Option<Picture>) -> Result<Track, TrackError> {
        let metadata = Tag::read_from_path(&path).unwrap();
        let artist = metadata.title();
        let artist_code = calculate_hash(artist);

        Track {
            path,
            title: metadata.title(),
            artist: artist,
            artist_code,
            album,
            rating,
            tags,
            picture,
        }   
    }
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) {
        self.path == other.path || (self.title == other.title && self.artist_code == other.artist_code && self.album == other.album)
    }
}

pub struct Artist {
    pub code: u64,
    pub name: String,
    pub alias: String,
    pub bio: String,
    pub tags: Option<Vec<String>>,
}

fn main() {
    let default_track = "test.mp3";
    let default_port = "127.0.0.1:7878";
    let vault_path = "";
    let opened_track = fs::File::open(default_track).unwrap();
    let listener = TcpListener::bind(default_port).unwrap();
}

fn register_vault(filepath: Path, vault_name: String,) -> Result<(), VaultError> {
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
    let probe = Probe::open(path)?;
    let track = Track::new(path);
    
    let audio_file = probe.read();
    match audio_file {
        Ok(af) => {},
        Err(e) => {println!("track error")},
    }
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish();
}

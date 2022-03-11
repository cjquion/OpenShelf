use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::thread;
use std::path::Path;
use std::fmt;

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

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

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
    fn initialize() -> Result<Box<Self>, failure::Error> {};
    fn refresh(&self, path: Path) -> {};
    fn data(&self) -> {}
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
    pub tag: Vec<String>,
}

fn main() {
    let default_track = "test.mp3";
    let default_port = "127.0.0.1:7878";
    let vault_path = "";
    let opened_track = fs::File::open(default_track).unwrap();
    let listener = TcpListener::bind(default_port).unwrap();
}

/*
fn handle_connection(mut stream: TcpStream) {
    let mut rx_buffer = [0; 1024];
    stream.read(&mut rx_buffer).unwrap();
    let (status, file) = if rx_buffer.starts_with("") {

    };
}
*/

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

}

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


fn main() {
    println!("Hello, world!");
}

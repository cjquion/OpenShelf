use std::fs;
use std::net::TcpStream;
use std::thread;

use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

use reqwest::header::{HeaderValue, CONTENT_LENGTH, RANGE};
use reqwest::StatusCode;


fn main() {
    const SERVER_ADDRESS = "127.0.0.1:7878";
    const CHUNK_SIZE: u32 = 10240;


    let player = Player::new();
    let download_handle = thread::spawn(move || {
        download_track()
    });
    let res = download_handle.join();
    Ok(())
}

fn download_track() {
        let client_agent = reqwest::blocking::Client::new();

        let response = client_agent.head(url).send()?;
        let length_raw = response
            .headers()
            .get(CONTENT_LENGTH)
            .ok_or("response does not contain content length.")?;
        let length_fmt = u64::from_str(length.to_str()?)
            .map_err(|_| "invalid Content-Length header")?;

        let mut output_file = File::create("currently_playing.mp3");
        for range in AudioBuffer::new(0, length_fmt - 1, CHUNK_SIZE)? {
            let response = client_agent.get(url).header(RANGE, range).send()?;
            let status = response.status();
            if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
                error_chain::bail!("Unexepected server response: {}", status)
            }
            std::io::copy(&mut content.as_bytes(), &mut output_file)?;
        }
}

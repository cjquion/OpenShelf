#![(allow]
mod client;

#[macro_use]
extern crate lazy_static;

pub fn main() {
    client::player::spawn();
}

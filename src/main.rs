use std::io;
use crate::generate_key::generate_key_map;

mod generate_key;
mod encrypt;

fn main() -> io::Result<()> {
    generate_key_map()
}

use std::collections::{HashSet, LinkedList};
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::Write;

pub fn generate_key_map() -> io::Result<()> {
    let mut file = File::create(DICTIONARY_PATH)?;
    let mut used_keys: HashSet<char> = HashSet::new();
    let mut result = "";
    for i in 65u8..=90u8 {
        let letter = i as char;
        let mut rng = rand::thread_rng();
        let mut key: char = rng.gen_range('A'..='Z');

        while used_keys.contains(&key) {
            key = rng.gen_range('A'..='Z');
        }
        used_keys.insert(key);
        let result = format!("{}\t{}\n", letter, key);

        write!(file, "{}", result)?;
    }
    Ok(())
}

const DICTIONARY_PATH: &'static str = "src/resources/dictionary.txt";

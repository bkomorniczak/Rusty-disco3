use std::io;
use rand::Rng;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{Write};

const DICTIONARY_PATH: &str = "src/resources/dictionary.txt";
const PLAINTEXT_PATH: &str = "src/resources/plain.txt";
const CIPHERTEXT_PATH: &str = "src/resources/ciphertext.txt";

pub fn generate_key_map() -> io::Result<()> {
    let mut file = File::create(DICTIONARY_PATH)?;
    let mut used_keys: HashSet<char> = HashSet::new();
    let mut rng = thread_rng();

    for i in 65u8..=90u8 {
        let letter = i as char;
        let mut key = rng.gen_range('A'..='Z');

        while used_keys.contains(&key) {
            key = rng.gen_range('A'..='Z');
        }
        used_keys.insert(key);
        let result = format!("{}\t{}\n", letter, key);

        write!(file, "{}", result)?;
    }
    Ok(())
}

pub fn read_dictionary() -> io::Result<HashMap<char, char>> {
    let contents = fs::read_to_string(DICTIONARY_PATH)?;
    let mut map = HashMap::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            let original = parts[0].chars().next().unwrap();
            let substitute = parts[1].chars().next().unwrap();
            map.insert(original, substitute);
        }
    }

    Ok(map)
}

pub fn read_plaintext() -> io::Result<String> {
    fs::read_to_string(PLAINTEXT_PATH)
}

pub fn encrypt_text(text: &str, map: &HashMap<char, char>) -> String {
    text.to_uppercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| *map.get(&c).unwrap_or(&c))
        .collect()
}

pub fn write_ciphertext(ciphertext: &str) -> io::Result<()> {
    fs::write(CIPHERTEXT_PATH, ciphertext)
}


fn main() {
    let plain = read_plaintext().unwrap();
    let dictionary = read_dictionary().unwrap();
    let cipher_text= encrypt_text(&plain, &dictionary);
    write_ciphertext(&cipher_text).expect("Write to file didn't happen.");
}
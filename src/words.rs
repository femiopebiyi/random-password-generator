use std::{fs, path::Path};

use rand::seq::IndexedRandom;

pub fn genrate_passphrase(word_count: usize) -> Result<String, Box<dyn std::error::Error>> {
    if Path::new("words.json").try_exists()? {
        let content = fs::read_to_string("words.json")?;
        let words: Vec<String> = serde_json::from_str(&content)?;

        let mut rng = rand::rng();
        let passphrase: Vec<String> = (0..word_count)
            .map(|_| words.choose(&mut rng).unwrap().to_string())
            .collect();

        return Ok(passphrase.join("-"));
    }

    Err("words.json file not found".into())
}

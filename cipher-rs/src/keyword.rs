use crate::{decipher_tools, dictionary};

use itertools::enumerate;
use std::io;

pub fn decipher(orig_ciphertext: &str) -> Option<String> {
    let _lock = io::stdout().lock();
    for (i, word) in enumerate(&dictionary::L_ENGLISH_ARR) {
        let mut cipher = vec![];
        let mut last = '0';
        for char in word.chars() {
            if !cipher.contains(&char) {
                cipher.push(char);
                last = char;
            }
        }
        let start_index = dictionary::ALPHABET_ARRAY
            .into_iter()
            .position(|c| c == last)
            .unwrap();

        for char in &dictionary::ALPHABET_ARRAY[start_index..] {
            if !cipher.contains(char) {
                cipher.push(*char);
            }
        }
        for char in &dictionary::ALPHABET_ARRAY[..(start_index + 2)] {
            if !cipher.contains(char) {
                cipher.push(*char);
            }
        }

        if let Some(unciphered) = decipher_tools::vec(cipher, orig_ciphertext) {
            return Some(unciphered);
        }

        if i % 10_000 == 0 {
            println!("{}%", i / 4000);
        }
    }
    None
}

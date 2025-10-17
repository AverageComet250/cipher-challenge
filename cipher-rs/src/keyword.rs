use crate::{dictionary, tools};

use eta::{Eta, TimeAcc};
use itertools::enumerate;
use std::io;

pub fn decipher(ciphertext: &str, aligned: bool) -> Option<String> {
    let _lock = io::stdout().lock();
    let mut eta = Eta::new(dictionary::ENGLISH_ARR.len() / 10_000, TimeAcc::MILLI);

    for (i, word) in enumerate(&dictionary::ENGLISH_ARR) {
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
        for char in &dictionary::ALPHABET_ARRAY[..(start_index + 2).min(26)] {
            if !cipher.contains(char) {
                cipher.push(*char);
            }
        }

        let plaintext = if aligned {
            tools::uncipher_vec_aligned(&cipher, ciphertext)
        } else {
            tools::uncipher_vec_non_aligned(&cipher, ciphertext)
        };

        if let Some(plaintext) = plaintext {
            return Some(plaintext);
        }

        if i % 10_000 == 0 {
            eta.step();
            if eta.progress() < 1.0 {
                println!(
                    "{:>6.2}% completed, {:>8}s remaining",
                    eta.progress() * 100.0,
                    eta.time_remaining() / 1000
                );
            }
        }
    }
    None
}

use crate::{
    dictionary,
    tools::{self, substitute_vec, uncipher_vec_and_score_non_aligned},
};

pub fn decipher(ciphertext: &str, aligned: bool) -> Option<String> {
    let possible_ciphers = rotate_wheel(dictionary::ALPHABET_ARRAY.to_vec());

    for cipher in possible_ciphers {
        let plaintext = if aligned {
            tools::uncipher_vec_aligned(&cipher, ciphertext)
        } else {
            tools::uncipher_vec_non_aligned(&cipher, ciphertext)
        };

        if let Some(plaintext) = plaintext {
            return Some(plaintext);
        }
    }
    None
}

pub fn most_deciphered(ciphertext: &str) -> String {
    let mut max = (vec![], 0.0);
    for shift in rotate_wheel(dictionary::ALPHABET_ARRAY.to_vec()) {
        let score = uncipher_vec_and_score_non_aligned(&shift, ciphertext);
        if score > max.1 {
            max = (shift, score)
        }
    }

    substitute_vec(&max.0, ciphertext)
}

fn rotate_wheel(mut wheel: Vec<char>) -> Vec<Vec<char>> {
    let mut wheels = vec![];
    for _ in 0..wheel.len() {
        let buf = wheel.remove(0);
        wheel.push(buf);
        wheels.push(wheel.clone());
    }

    wheels
}

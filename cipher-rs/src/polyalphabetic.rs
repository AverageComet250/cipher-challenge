use crate::{dictionary, tools};

pub fn decipher(ciphertext: &str, aligned: bool) -> Option<String> {
    let words: String = ciphertext
        .chars()
        .filter(|c| c.is_alphabetic() || *c == ' ')
        .collect();
    let letters: String = words.chars().filter(|c| c.is_alphabetic()).collect();

    None
}

fn decipher_vignere(ciphertext: &str, aligned: bool, prob_key_len: i32) -> Option<String> {
    let letters: String = ciphertext.chars().filter(|c| c.is_alphabetic()).collect();

    for key in dictionary::ENGLISH_ARR.iter() {
        if key.chars().count() != prob_key_len as usize {
            continue;
        }

        let mut plaintext = String::new();
        for (i, char) in letters.chars().enumerate() {
            plaintext.push(
                dictionary::VIGNERE_TABLE[i % prob_key_len as usize][(char as u8 - b'a') as usize],
            );
        }
        if tools::is_unciphered(&plaintext, false) {
            return Some(plaintext);
        }
    }
    None
}

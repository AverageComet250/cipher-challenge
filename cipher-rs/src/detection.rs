use crate::{CipherType, dictionary};

pub fn autodetect(ciphertext: &str) -> CipherType {
    let ioc = index_of_coincidence(ciphertext);
    if ioc > 0.057 && ioc < 0.075 {
        CipherType::Monoalphabetic
    } else {
        CipherType::Polyalphabetic
    }
}

fn index_of_coincidence(text: &str) -> f64 {
    let text: String = text.chars().filter(|char| char.is_alphabetic()).collect();
    let total_letters = text.len();
    let mut top = 0;
    for letter in dictionary::ALPHABET_ARRAY {
        let n_i: i32 = text.chars().filter(|char| *char == letter).count() as i32;
        top += n_i * (n_i - 1)
    }

    top as f64 / (total_letters * (total_letters - 1)) as f64
}

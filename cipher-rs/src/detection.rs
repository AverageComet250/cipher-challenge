use crate::{CipherType, dictionary};

pub fn autodetect(ciphertext: &str) -> CipherType {
    let ciphertext: String = ciphertext
        .chars()
        .filter(|char| char.is_alphabetic())
        .collect();
    let ioc = index_of_coincidence(&ciphertext);
    let entropy = entropy(&ciphertext);

    if (0.055..0.075).contains(&ioc) && (3.95..4.25).contains(&entropy) {
        CipherType::Monoalphabetic
    } else if (0.28..0.48).contains(&entropy) && (4.25..4.55).contains(&entropy) {
        CipherType::Polyalphabetic
    } else {
        CipherType::Unknown
    }
}

pub fn get_scores(text: &str) -> (f64, f64) {
    let text: String = text.chars().filter(|char| char.is_alphabetic()).collect();
    (index_of_coincidence(&text), entropy(&text))
}

fn index_of_coincidence(text: &str) -> f64 {
    let total_letters = text.chars().count();
    let mut top = 0;
    for letter in dictionary::ALPHABET_ARRAY {
        let n_i: i32 = text.chars().filter(|char| *char == letter).count() as i32;
        top += n_i * (n_i - 1);
    }

    top as f64 / (total_letters * (total_letters - 1)) as f64
}

fn entropy(text: &str) -> f64 {
    let n = text.chars().count() as f64;
    let mut h = 0.0;
    for letter in dictionary::ALPHABET_ARRAY {
        let n_i: f64 = text.chars().filter(|char| *char == letter).count() as f64;
        if n_i != 0.0 {
            let p_i = n_i / n;
            h += -p_i * p_i.log2();
        }
    }

    h
}

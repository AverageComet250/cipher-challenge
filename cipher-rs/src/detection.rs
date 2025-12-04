use itertools::Itertools;

use crate::{CipherType, freq_analysis};

pub fn autodetect(ciphertext: &str) -> CipherType {
    let ciphertext: String = ciphertext
        .chars()
        .filter(|char| char.is_alphabetic())
        .collect();
    let ioc = freq_analysis::index_of_coincidence(&ciphertext);
    let entropy = freq_analysis::entropy(&ciphertext);
    let chi2 = freq_analysis::chi2(&ciphertext);

    if (0.053..0.075).contains(&ioc) && (3.95..4.25).contains(&entropy) && chi2 > 200.0 {
        CipherType::Monoalphabetic
    } else if (0.028..0.052).contains(&ioc) && (4.25..4.70).contains(&entropy) && chi2 > 200.0 {
        for (c1, c2) in ciphertext.chars().tuples() {
            if c1 == c2 {
                return CipherType::Polyalphabetic;
            }
        }
        CipherType::Playfair
    } else if (0.053..0.075).contains(&ioc) && (3.95..4.25).contains(&entropy) && chi2 <= 200.0 {
        CipherType::Transposition
    } else {
        CipherType::Unknown
    }
}

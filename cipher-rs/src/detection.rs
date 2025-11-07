use crate::{CipherType, freq_analysis};

pub fn autodetect(ciphertext: &str) -> CipherType {
    let ciphertext: String = ciphertext
        .chars()
        .filter(|char| char.is_alphabetic())
        .collect();
    let ioc = freq_analysis::index_of_coincidence(&ciphertext);
    let entropy = freq_analysis::entropy(&ciphertext);
    let chi2 = freq_analysis::chi2(&ciphertext);

    if (0.055..0.075).contains(&ioc) && (3.95..4.25).contains(&entropy) && chi2 > 200.0 {
        CipherType::Monoalphabetic
    } else if (0.28..0.48).contains(&entropy) && (4.25..4.55).contains(&entropy) && chi2 > 200.0 {
        CipherType::Polyalphabetic
    } else if (0.055..0.075).contains(&ioc) && (3.95..4.25).contains(&entropy) && chi2 <= 200.0 {
        CipherType::Transposition
    } else {
        CipherType::Unknown
    }
}

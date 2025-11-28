use itertools::Itertools;

use crate::{freq_analysis::kasiski_score, tools::is_unciphered};

pub fn decipher(ciphertext: &str, _aligned: bool) -> Option<String> {
    let words: String = ciphertext
        .chars()
        .filter(|c| c.is_alphabetic() || *c == ' ')
        .collect();
    let letters: String = words.chars().filter(|c| c.is_alphabetic()).collect();
    let mut keylens = Vec::with_capacity(29);
    for k in 2..30 {
        let score = kasiski_score(&letters, k);
        keylens.push((score, k));
    }
    for (_, keylen) in keylens
        .iter()
        .sorted_by_key(|(score, _)| 100_000 - (score * 100_000.0) as i32)
    {
        if let Some(plain) = decipher_vignere(&letters, *keylen as i32) {
            return Some(plain);
        }
    }
    None
}

fn decipher_vignere(ciphertext: &str, prob_key_len: i32) -> Option<String> {
    let mut part_plains: Vec<Vec<String>> = vec![Vec::new(); prob_key_len as usize];

    for shift in 0..prob_key_len {
        let current_letters: String = ciphertext
            .chars()
            .enumerate()
            .filter(|(i, _)| i % prob_key_len as usize == shift as usize)
            .map(|(_, l)| l)
            .collect();
        let current_plains = crate::caesar::most_deciphered(&current_letters);
        part_plains[shift as usize] = current_plains;
    }

    for current_plains in part_plains.iter().multi_cartesian_product() {
        let mut plain: Vec<char> = vec![' '; ciphertext.chars().count()];
        for (shift, current_plain) in current_plains.iter().enumerate() {
            for (i, c) in current_plain.chars().enumerate() {
                plain[i * prob_key_len as usize + shift] = c;
            }
        }

        let plain = plain.iter().collect::<String>();
        let unciphered = is_unciphered(&plain, false);
        if unciphered {
            return Some(plain);
        }
    }
    None
}

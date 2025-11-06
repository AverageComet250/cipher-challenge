use itertools::Itertools;
use std::collections::HashMap;

use crate::dictionary;

pub fn uncipher_vec_aligned(cipher: &[char], text: &str) -> Option<String> {
    uncipher_map_aligned(&cipher_vec_to_map(cipher), text)
}

pub fn uncipher_map_aligned(cipher: &HashMap<char, char>, text: &str) -> Option<String> {
    let unciphered = substitute(cipher, text);
    if is_unciphered_aligned(&unciphered) {
        return Some(unciphered);
    }
    None
}
pub fn uncipher_vec_non_aligned(cipher: &[char], text: &str) -> Option<String> {
    uncipher_map_non_aligned(&cipher_vec_to_map(cipher), text)
}

pub fn uncipher_map_non_aligned(cipher: &HashMap<char, char>, text: &str) -> Option<String> {
    let unciphered = substitute(cipher, text);
    if is_unciphered_non_aligned(&unciphered) {
        return Some(unciphered);
    }
    None
}

fn cipher_vec_to_map(cipher: &[char]) -> HashMap<char, char> {
    cipher
        .iter()
        .enumerate()
        .map(|(i, char)| (*char, dictionary::ALPHABET_ARRAY[i]))
        .collect()
}

fn chi2(text: &str) -> f64 {
    let letter_count = text.chars().count();
    let mut c: [f64; 26] = [0.0; 26];
    for letter in text.chars() {
        c[(letter.to_ascii_lowercase() as u8 - b'a') as usize] += 1.0;
    }

    let mut total = 0.0;
    for (i, c_i) in c.iter().enumerate() {
        let e_i = dictionary::LETTER_FREQ[i] * letter_count as f64;
        total += (c_i - e_i).powi(2) / e_i;
    }

    total
}

fn bigram_score(text: &str) -> f64 {
    let mut score = 0.0;
    let mut i = 0;

    for (c1, c2) in text.chars().tuple_windows() {
        if c1 == ' ' || c2 == ' ' {
            continue;
        }
        score += dictionary::BIGRAM_FREQ
            .get([c1, c2].iter().collect::<String>().as_str())
            .unwrap_or(&1e-8)
            .log2();
        i += 1;
    }
    score / i as f64
}

fn trigram_score(text: &str) -> f64 {
    let mut score = 0.0;
    let mut i = 0;

    for (c1, c2, c3) in text.chars().tuple_windows() {
        if c1 == ' ' || c2 == ' ' || c3 == ' ' {
            continue;
        }
        score += dictionary::TRIGRAM_FREQ
            .get([c1, c2, c3].iter().collect::<String>().as_str())
            .unwrap_or(&1e-8)
            .log2();
        i += 1;
    }
    score / i as f64
}

fn substitute(cipher: &HashMap<char, char>, text: &str) -> String {
    text.chars()
        .map(|char| {
            if char.is_alphabetic() {
                *cipher.get(&char).unwrap()
            } else {
                char
            }
        })
        .join("")
}

fn is_unciphered_aligned(text: &str) -> bool {
    let text_c: String = text.chars().filter(|char| char.is_alphabetic()).collect();
    let text_w: String = text
        .chars()
        .filter(|char| char.is_alphabetic() || char.is_whitespace())
        .collect();
    let chi_squared = chi2(&text_c);
    if chi_squared < 60.0 {
        return true;
    } else if chi_squared < 200.0 {
        let words = text_w.split_whitespace();
        let wordcount = words.clone().count();
        let mut valid = 0;
        for word in words {
            if dictionary::ENGLISH_SET.contains(word.trim()) {
                valid += 1;
            } else {
                println!(
                    "Word not found in dictionary, consider adding it if it's a real word: {word}"
                );
            }
        }
        if valid as f64 / wordcount as f64 >= 0.7 {
            return true;
        }
    }
    false
}

pub fn get_scores(text: &str) -> (f64, f64, f64, f64) {
    let only_words_text: String = text
        .chars()
        .filter(|char| char.is_alphabetic() || char == &' ')
        .collect();

    let no_spaces_text: String = only_words_text
        .chars()
        .filter(|char| char.is_alphabetic())
        .collect();
    (
        englishness_score(text),
        chi2(&no_spaces_text),
        bigram_score(&only_words_text),
        trigram_score(&only_words_text),
    )
}

fn is_unciphered_non_aligned(text: &str) -> bool {
    let score = englishness_score_non_aligned(text);
    score > 0.8
}

fn englishness_score(text: &str) -> f64 {
    let only_words_text: String = text
        .chars()
        .filter(|char| char.is_alphabetic() || char == &' ')
        .collect();

    let no_spaces_text: String = only_words_text
        .chars()
        .filter(|char| char.is_alphabetic())
        .collect();
    let chi2_norm = 0.0_f64.max(1.0 - chi2(&no_spaces_text) / 10_000.0);
    if chi2_norm <= 0.05 {
        return 0.0;
    }
    let bigram = bigram_score(&only_words_text);
    const BIGRAM_WORST: f64 = -14.0;
    const bigram_best: f64 = -7.5;
    let bigram_norm = ((bigram - BIGRAM_WORST) / (bigram_best - BIGRAM_WORST)).clamp(0.0, 1.0);
    const trigram_worst: f64 = -21.5;
    const trigram_best: f64 = -10.0;
    let trigram = trigram_score(&only_words_text);
    let trigram_norm = ((trigram - trigram_worst) / (trigram_best - trigram_worst)).clamp(0.0, 1.0);

    let avg = (chi2_norm
        * bigram_norm
        * bigram_norm
        * bigram_norm
        * trigram_norm
        * trigram_norm
        * trigram_norm
        * trigram_norm
        * trigram_norm
        * trigram_norm
        * trigram_norm)
        .powf(1.0 / (1.0 + 3.0 + 6.0));
    avg * avg * avg * avg * avg * avg
}

fn englishness_score_non_aligned(text: &str) -> f64 {
    let no_spaces_text: String = text.chars().filter(|char| char.is_alphabetic()).collect();
    let chi2_norm = 1.0 - (chi2(&no_spaces_text) / 10_000.0).powf(0.383_209_703_183_097_9);
    if chi2_norm <= 0.05 {
        return 0.0;
    }

    chi2_norm
}

pub fn uncipher_vec_and_score(cipher: &[char], text: &str) -> f64 {
    let cipher = cipher_vec_to_map(cipher);
    englishness_score(&substitute(&cipher, text))
}

pub fn uncipher_vec_and_score_non_aligned(cipher: &[char], text: &str) -> f64 {
    let cipher = cipher_vec_to_map(cipher);
    englishness_score_non_aligned(&substitute(&cipher, text))
}

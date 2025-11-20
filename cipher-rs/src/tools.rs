use itertools::Itertools;
use std::collections::HashMap;

use crate::{dictionary, freq_analysis};

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

pub fn is_unciphered(text: &str, aligned: bool) -> bool {
    if aligned {
        is_unciphered_aligned(text)
    } else {
        is_unciphered_non_aligned(text)
    }
}

fn is_unciphered_aligned(text: &str) -> bool {
    let text_c: String = text.chars().filter(|char| char.is_alphabetic()).collect();
    let text_w: String = text
        .chars()
        .filter(|char| char.is_alphabetic() || char.is_whitespace())
        .collect();
    let chi_squared = freq_analysis::chi2(&text_c);
    if chi_squared < 60.0 {
        return true;
    } else if chi_squared < 200.0 {
        let words = text_w.split_whitespace();
        let wordcount = words.clone().count();
        let mut valid = 0;
        if valid as f64 / wordcount as f64 >= 0.7 {
            return true;
        }
    }
    false
}

fn is_unciphered_non_aligned(text: &str) -> bool {
    let score = englishness_score_non_aligned(text);
    score > 0.8
}

fn englishness_score(text: &str) -> f64 {
    const BIGRAM_WORST: f64 = -14.0;
    const BIGRAM_BEST: f64 = -7.5;
    const TRIGRAM_WORST: f64 = -21.5;
    const TRIGRAM_BEST: f64 = -10.0;
    const AVG_EXP: f64 = 1.0 / (1.0 + 3.0 + 6.0);
    const CHI_2_CUTOFF: f64 = 10_000.0;
    const CHI_2_EFFICIENCY_RATING: f64 = 0.05;

    let only_words_text: String = text
        .chars()
        .filter(|char| char.is_alphabetic() || char == &' ')
        .collect();

    let no_spaces_text: String = only_words_text
        .chars()
        .filter(|char| char.is_alphabetic())
        .collect();
    let chi2_norm = (1.0 - freq_analysis::chi2(&no_spaces_text) / CHI_2_CUTOFF).clamp(0.0, 1.0);
    if chi2_norm <= CHI_2_EFFICIENCY_RATING {
        return 0.0;
    }
    let bigram = freq_analysis::bigram_log_score(&only_words_text);
    let bigram_norm = ((bigram - BIGRAM_WORST) / (BIGRAM_BEST - BIGRAM_WORST)).clamp(0.0, 1.0);
    let trigram = freq_analysis::trigram_log_score(&only_words_text);
    let trigram_norm = ((trigram - TRIGRAM_WORST) / (TRIGRAM_BEST - TRIGRAM_WORST)).clamp(0.0, 1.0);

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
        .powf(AVG_EXP);
    avg * avg * avg * avg * avg * avg
}

fn englishness_score_non_aligned(text: &str) -> f64 {
    let no_spaces_text: String = text.chars().filter(|char| char.is_alphabetic()).collect();
    let chi2_norm =
        1.0 - (freq_analysis::chi2(&no_spaces_text) / 10_000.0).powf(0.383_209_703_183_097_9);
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

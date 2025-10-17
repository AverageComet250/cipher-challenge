use itertools::Itertools;
use std::collections::HashMap;

use crate::dictionary;

pub fn uncipher_vec_aligned(cipher: &Vec<char>, text: &str) -> Option<String> {
    uncipher_map_aligned(cipher_vec_to_map(cipher), text)
}

pub fn uncipher_map_aligned(cipher: HashMap<char, char>, text: &str) -> Option<String> {
    let unciphered = substitute(cipher, text);
    dbg!(&unciphered);
    if is_unciphered_aligned(&unciphered) {
        return Some(unciphered);
    }
    None
}

pub fn uncipher_vec_non_aligned(cipher: &Vec<char>, text: &str) -> Option<String> {
    uncipher_map_non_aligned(cipher_vec_to_map(cipher), text)
}

pub fn uncipher_map_non_aligned(cipher: HashMap<char, char>, text: &str) -> Option<String> {
    let unciphered = substitute(cipher, text);
    if is_unciphered_non_aligned(&unciphered) {
        return Some(unciphered);
    }
    None
}

fn cipher_vec_to_map(cipher: &Vec<char>) -> HashMap<char, char> {
    cipher
        .iter()
        .enumerate()
        .map(|(i, char)| (*char, dictionary::ALPHABET_ARRAY[i]))
        .collect()
}

fn chi2(text: &str) -> f64 {
    let letter_count = text.len();
    let mut c: [f64; 26] = [0.0; 26];
    for letter in text.chars() {
        c[(letter.to_ascii_lowercase() as u8 - b'a') as usize] += 1.0;
    }

    let mut total = 0.0;
    for i in 0..26 {
        let e_i = dictionary::LETTER_FREQ[i] * letter_count as f64;
        total += (c[i] - e_i).powi(2) / e_i;
    }

    total
}

fn bigram_score(text: &str) -> f64 {
    let mut score = 0.0;

    for i in 1..(text.len() - 1) {
        score += dictionary::BIGRAM_FREQ.get(&text[i..i + 2]).unwrap().log2();
    }
    score / (text.len()) as f64
}

fn trigram_score(text: &str) -> f64 {
    let mut score = 0.0;

    for i in 1..(text.len() - 2) {
        score += dictionary::TRIGRAM_FREQ
            .get(&text[i..i + 3])
            .unwrap()
            .log2();
    }
    score / (text.len() - 2) as f64
}

fn substitute(cipher: HashMap<char, char>, text: &str) -> String {
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
    if chi_squared < 45.0 {
        return true;
    } else if chi_squared < 100.0 {
        let words = text_w.split_whitespace();
        let wordcount = words.clone().count();
        let mut valid = 0;
        for word in words {
            if dictionary::ENGLISH.contains(word.trim()) {
                valid += 1;
            } else {
                dbg!(word);
            }
        }
        if valid as f64 / wordcount as f64 >= 0.5 {
            return true;
        }
    }
    false
}

fn is_unciphered_non_aligned(text: &str) -> bool {
    let text: String = text.chars().filter(|char| char.is_alphabetic()).collect();
    let chi2_norm = 0.0_f64.max(1.0 - chi2(&text) / 130.0);
    if chi2_norm <= 0.07 {
        return false;
    }

    let bigram = bigram_score(&text);
    let bigram_norm = ((bigram + 13.0) / (-6.0 + 13.0)).clamp(0.0, 1.0);
    let trigram = trigram_score(&text);
    let trigram_norm = ((trigram + 21.5) / (-9.0 + 21.5)).clamp(0.0, 1.0);

    let mean = (chi2_norm.powi(1) * bigram_norm.powi(2) * trigram_norm.powi(2))
        .powf(1.0 / (1.0 + 2.0 + 2.0));

    mean > 0.7
}

use itertools::Itertools;
use std::collections::HashMap;

use crate::dictionary;

pub fn vec(cipher: &Vec<char>, text: &str) -> Option<String> {
    map(cipher_vec_to_map(cipher), text)
}

pub fn map(cipher: HashMap<char, char>, text: &str) -> Option<String> {
    let unciphered = substitute(cipher, text);
    if is_unciphered_words(&unciphered) {
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

pub fn chi2_vec(cipher: &Vec<char>, text: &str) -> f64 {
    chi2_map(cipher_vec_to_map(cipher), text)
}

pub fn chi2_map(cipher: HashMap<char, char>, text: &str) -> f64 {
    let text: String = text.chars().filter(|char| char.is_alphabetic()).collect();
    chi2(substitute(cipher, &text))
}

fn chi2(text: String) -> f64 {
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

pub fn uncipher_key_vec(cipher: &Vec<char>, text: &str) -> String {
    substitute(cipher_vec_to_map(cipher), text)
}

fn substitute(cipher: HashMap<char, char>, text: &str) -> String {
    text.chars()
        .map(|char| {
            if dictionary::ALPHABET_SET.contains(&char) {
                *cipher.get(&char).unwrap()
            } else {
                char
            }
        })
        .join("")
}

fn is_unciphered_words(text: &str) -> bool {
    let chi_squared = chi2(text.chars().filter(|char| char.is_alphabetic()).collect());
    if chi_squared < 37.7 {
        return true;
    } else if chi_squared < 100.0 {
        let words = text.split(" ");
        let wordcount = words.clone().count();
        let required = if wordcount <= 6 { 0.8 } else { 0.6 };
        let mut i = 0;
        for word in words {
            if dictionary::ENGLISH.contains(word.trim()) {
                i += 1;
                if i as f64 / wordcount as f64 > required {
                    return true;
                }
            }
        }
    }
    false
}

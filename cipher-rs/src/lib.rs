use itertools::{Itertools, enumerate};
use std::{collections::HashMap, io};

pub mod affine;
pub mod caesar;
mod decipher_tools;
pub mod dictionary;
pub mod keyword;

pub fn decpipher_caeser(orig_ciphertext: &str) -> Option<String> {
    let possible_ciphers = rotate_wheel(dictionary::ALPHABET_ARRAY.to_vec());

    for cipher in possible_ciphers {
        if let Some(unciphered) = decipher_vec(cipher, orig_ciphertext) {
            return Some(unciphered);
        }
    }
    None
}

pub fn decipher_affine(orig_ciphertext: &str) -> Option<String> {
    let mult: [usize; 12] = [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25];
    let add = 0usize..=25usize;

    for a in add {
        for m in mult {
            let cipher = dictionary::ALPHABET_ARRAY
                .into_iter()
                .enumerate()
                .map(|(i, char)| (char, dictionary::ALPHABET_ARRAY[(i * m + a) % 26usize]))
                .collect();
            if let Some(unciphered) = decipher_map(cipher, orig_ciphertext) {
                dbg!(a);
                dbg!(m);
                return Some(unciphered);
            }
        }
    }

    None
}

fn decipher_vec(cipher: Vec<char>, text: &str) -> Option<String> {
    decipher_map(
        cipher
            .into_iter()
            .enumerate()
            .map(|(i, char)| (char, dictionary::ALPHABET_ARRAY[i]))
            .collect(),
        text,
    )
}

fn decipher_map(cipher: HashMap<char, char>, text: &str) -> Option<String> {
    let unciphered = substitute(cipher, text);
    if is_unciphered(&unciphered) {
        return Some(unciphered);
    }
    None
}

fn rotate_wheel(mut wheel: Vec<char>) -> Vec<Vec<char>> {
    let mut wheels = vec![];
    for _ in 0..wheel.len() {
        let buf = wheel.remove(0);
        wheel.push(buf);
        wheels.push(wheel.clone());
    }

    wheels
}

pub fn decipher_keyword(orig_ciphertext: &str) -> Option<String> {
    let _lock = io::stdout().lock();
    for (i, word) in enumerate(&dictionary::L_ENGLISH_ARR) {
        let mut cipher = vec![];
        let mut last = '0';
        for char in word.chars() {
            if !cipher.contains(&char) {
                cipher.push(char);
                last = char;
            }
        }
        let start_index = dictionary::ALPHABET_ARRAY
            .into_iter()
            .position(|c| c == last)
            .unwrap();

        for char in &dictionary::ALPHABET_ARRAY[start_index..] {
            if !cipher.contains(char) {
                cipher.push(*char);
            }
        }
        for char in dictionary::ALPHABET_ARRAY {
            if !cipher.contains(&char) {
                cipher.push(char);
            }
        }

        if let Some(unciphered) = decipher_vec(cipher, orig_ciphertext) {
            return Some(unciphered);
        }

        if i % 10_000 == 0 {
            println!("{}%", i / 4000);
        }
    }
    None
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

fn is_unciphered(text: &str) -> bool {
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
    false
}

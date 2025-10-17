use std::collections::HashMap;

use crate::dictionary;

pub fn vec(cipher: Vec<char>, text: &str) -> Option<String> {
    map(
        cipher
            .into_iter()
            .enumerate()
            .map(|(i, char)| (char, dictionary::ALPHABET_ARRAY[i]))
            .collect(),
        text,
    )
}

pub fn map(cipher: HashMap<char, char>, text: &str) -> Option<String> {
    let unciphered = substitute(cipher, text);
    if is_unciphered(&unciphered) {
        return Some(unciphered);
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

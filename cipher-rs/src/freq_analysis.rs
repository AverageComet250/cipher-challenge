use itertools::Itertools;

use crate::dictionary;

pub fn get_scores(text: &str) -> (f64, f64) {
    let text: String = text.chars().filter(|char| char.is_alphabetic()).collect();
    (index_of_coincidence(&text), entropy(&text))
}

pub fn index_of_coincidence(text: &str) -> f64 {
    let total_letters = text.chars().count();
    let mut top = 0;
    for letter in dictionary::ALPHABET_ARRAY {
        let n_i: i32 = text.chars().filter(|char| *char == letter).count() as i32;
        top += n_i * (n_i - 1);
    }

    top as f64 / (total_letters * (total_letters - 1)) as f64
}

pub fn entropy(text: &str) -> f64 {
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

pub fn chi2(text: &str) -> f64 {
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

pub fn bigram_log_score(text: &str) -> f64 {
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

pub fn trigram_log_score(text: &str) -> f64 {
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

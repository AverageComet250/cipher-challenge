use std::collections::HashMap;

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

pub fn autocorrelation_score(text: &str, k: usize) -> f64 {
    let n = text.chars().count();
    if n <= k {
        return 0.0;
    }

    let sum = text
        .chars()
        .tuple_windows()
        .filter(|(c1, c2)| c1 == c2)
        .count();

    sum as f64 / (n - k) as f64
}

pub fn kasiski_score(text: &str, k: usize) -> f64 {
    let mut possible = 0;
    let mut max_possible = 0;

    let mut kasiski_3: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, (c1, c2, c3)) in text.chars().tuple_windows().enumerate() {
        let key = [c1, c2, c3].iter().collect();
        kasiski_3.entry(key).or_default().push(i);
    }

    let mut kasiski_4: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, (c1, c2, c3, c4)) in text.chars().tuple_windows().enumerate() {
        let key = [c1, c2, c3, c4].iter().collect();
        kasiski_4.entry(key).or_default().push(i);
    }

    let mut kasiski_5: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, (c1, c2, c3, c4, c5)) in text.chars().tuple_windows().enumerate() {
        let key = [c1, c2, c3, c4, c5].iter().collect();
        kasiski_5.entry(key).or_default().push(i);
    }

    for (_, positions) in kasiski_3 {
        let n = positions.len();
        if n <= 2 {
            if n == 2 {
                max_possible += 1;
                if positions[0].abs_diff(positions[1]) % k == 0 {
                    possible += 1;
                }
            }
            continue;
        }
        max_possible += 3 * n * (n - 1) / 2;
        possible += 3 * positions
            .iter()
            .tuple_combinations()
            .map(|(p1, p2)| (*p1).abs_diff(*p2))
            .filter(|diff| diff % k == 0)
            .count();
    }

    for (_, positions) in kasiski_4 {
        let n = positions.len();
        if n <= 2 {
            if n == 2 {
                max_possible += 2;
                if positions[0].abs_diff(positions[1]) % k == 0 {
                    possible += 2;
                }
            }
            continue;
        }
        max_possible += 4 * n * (n - 1) / 2;
        possible += 4 * positions
            .iter()
            .tuple_combinations()
            .map(|(p1, p2)| (*p1).abs_diff(*p2))
            .filter(|diff| diff % k == 0)
            .count();
    }

    for (_, positions) in kasiski_5.iter() {
        let n = positions.len();
        if n <= 2 {
            if n == 2 {
                max_possible += 3;
                if positions[0].abs_diff(positions[1]) % k == 0 {
                    possible += 3;
                }
            }
            continue;
        }
        max_possible += 5 * n * (n - 1) / 2;
        possible += 5 * positions
            .iter()
            .tuple_combinations()
            .map(|(p1, p2)| (*p1).abs_diff(*p2))
            .filter(|diff| diff % k == 0)
            .count();
    }

    let t = (possible as f64 / max_possible as f64).clamp(0.0, 1.0);
    t * t * t * (t * (6.0 * t - 15.0) + 10.0)
}

pub fn friedman_score(text: &str, k: usize) -> f64 {
    let n = text.chars().count() as f64;
    let ic = index_of_coincidence(text);
    let m = (0.027 * n) / ((n - 1.0) * ic - 0.038 * n + 0.065);
    const EPSILON: f64 = 1e-6;
    const ALPHA: f64 = 1.0;
    let d = (k as f64 / (m + EPSILON)).ln().abs();
    (-ALPHA * d).exp()
}

pub fn decomposition_score(text: &str, k: usize) -> f64 {
    let n = text.chars().count();
    let mut ic_sum = 0.0;
    for j in 1..k {
        let column: String = text
            .chars()
            .enumerate()
            .filter(|(i, _)| *i == (n / j) % k)
            .map(|(_, char)| char)
            .collect();

        let ic_c = index_of_coincidence(&column);
        ic_sum += ic_c;
    }

    const I_RAND: f64 = 0.03846;
    const I_ENG: f64 = 0.0667;
    ((ic_sum - I_RAND) / (I_ENG - I_RAND)).clamp(0.0, 1.0)
}

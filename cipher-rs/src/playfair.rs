use itertools::Itertools;

use crate::{dictionary, tools};

pub fn decipher(ciphertext: &str) -> Option<String> {
    let ciphertext: String = ciphertext
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let mut alphabet = dictionary::ALPHABET_ARRAY.to_vec();
    alphabet.swap_remove(9); // j
    fastrand::shuffle(&mut alphabet);
    let mut best_key: [char; 25] = alphabet.try_into().unwrap();

    let decrypted = decrypt_playfair(best_key, &ciphertext);
    let mut best_score = tools::playfair_score(&decrypted);

    let mut last_score = best_score;
    let mut repeats = 0;
    dbg!(&best_score);
    dbg!(&best_key);

    const N: u32 = 10_000;
    const TEMP_STEP: f64 = 0.002;

    let mut t = 0.2;
    while t > 0.0 {
        t -= TEMP_STEP;
        let mut current_key = best_key;
        let mut current_score = best_score;
        for _ in 0..N {
            let new_key = mutate(current_key, &ciphertext);
            let decrypted: String = decrypt_playfair(new_key, &ciphertext)
                .chars()
                .filter(|char| char.is_alphabetic())
                .collect();
            let new_score = tools::playfair_score(&decrypted);
            if new_score > current_score
                || fastrand::f64() < f64::exp((new_score - current_score) / t)
            {
                current_key = new_key;
                current_score = new_score;
            }

            if current_score > best_score {
                best_score = current_score;
                best_key = current_key;
            }
        }
        if last_score == best_score {
            repeats += 1;
        } else {
            repeats = 0;
        }
        last_score = best_score;
        if repeats > 25 {
            t = 20.0;
            repeats = 0;
        }
        dbg!(&t);
        dbg!(&best_score);
        dbg!(&best_key);
        dbg!(&decrypt_playfair(best_key, &ciphertext));
    }

    let plaintext = decrypt_playfair(best_key, &ciphertext);
    tools::playfair_score(&plaintext);
    if tools::is_unciphered(&plaintext, false) {
        Some(plaintext)
    } else {
        dbg!(&plaintext);
        None
    }
}

fn mutate(key: [char; 25], ciphertext: &str) -> [char; 25] {
    let mut key = key;
    let p = fastrand::u8(0..50);

    match p {
        0 => key.reverse(),
        1 => {
            for col in 0..5 {
                for i in 0..2 {
                    key.swap(col + i * 5, col + (4 - i) * 5);
                }
            }
        }
        2 => {
            let row = fastrand::usize(0..5);
            key[5 * row..5 * row + 5].reverse();
        }
        3 => {
            let row1 = fastrand::usize(0..5);
            let mut row2 = fastrand::usize(0..5);
            while row2 == row1 {
                row2 = fastrand::usize(0..5);
            }
            for i in 0..5 {
                key.swap(row1 * 5 + i, row2 * 5 + i);
            }
        }
        4 => {
            let col1 = fastrand::usize(0..5);
            let mut col2 = fastrand::usize(0..5);
            while col2 == col1 {
                col2 = fastrand::usize(0..5);
            }
            for i in 0..5 {
                key.swap(col1 + i * 5, col2 + i * 5);
            }
        }
        5 => {
            let col1 = fastrand::usize(0..5);
            let mut col2 = fastrand::usize(0..5);
            while col2 == col1 {
                col2 = fastrand::usize(0..5);
            }
            for i in 0..5 {
                key.swap(col1 + i * 5, col2 + i * 5);
            }
        }
        _ => loop {
            let ch1 = fastrand::usize(0..25);
            let ch2 = fastrand::usize(0..25);
            key.swap(ch1, ch2);
            if fastrand::u8(0..3) > 1 {
                break;
            }
        },
    }

    key
}

fn decrypt_playfair(key: [char; 25], ciphertext: &str) -> String {
    let mut plaintext = String::with_capacity(ciphertext.chars().count());
    let i_pos = key
        .iter()
        .position(|&ch| ch == 'i')
        .map(|i| (i / 5, i % 5))
        .unwrap();

    for (c1, c2) in ciphertext.chars().tuples() {
        let (x1, y1) = key
            .iter()
            .position(|&ch| ch == c1)
            .map(|i| (i / 5, i % 5))
            .unwrap_or(i_pos);
        let (x2, y2) = key
            .iter()
            .position(|&ch| ch == c2)
            .map(|i| (i / 5, i % 5))
            .unwrap_or(i_pos);

        if y1 == y2 {
            // same column
            let x1 = (x1 as i32 - 1 + 5) % 5;
            let x2 = (x2 as i32 - 1 + 5) % 5;

            plaintext.push(key[x1 as usize * 5 + y1]);
            plaintext.push(key[x2 as usize * 5 + y2]);
        } else if x1 == x2 {
            // same row
            let y1 = (y1 as i32 - 1 + 5) % 5;
            let y2 = (y2 as i32 - 1 + 5) % 5;

            plaintext.push(key[x1 * 5 + y1 as usize]);
            plaintext.push(key[x2 * 5 + y2 as usize]);
        } else {
            // rectangle
            plaintext.push(key[x1 * 5 + y2]);
            plaintext.push(key[x2 * 5 + y1]);
        }
    }

    plaintext
}

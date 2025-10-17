use crate::{dictionary, tools};

pub fn decipher(orig_ciphertext: &str, aligned: bool) -> Option<String> {
    let possible_ciphers = rotate_wheel(dictionary::ALPHABET_ARRAY.to_vec());

    if aligned {
        for cipher in possible_ciphers {
            if let Some(unciphered) = tools::vec(&cipher, orig_ciphertext) {
                return Some(unciphered);
            }
        }
    } else {
        let mut smallest_chi2 = 100000.0;
        let mut nearest_key = vec![];
        for cipher in possible_ciphers {
            let chi2 = tools::chi2_vec(&cipher, orig_ciphertext);
            if chi2 < smallest_chi2 {
                nearest_key = cipher;
                smallest_chi2 = chi2;
            }
        }
        return Some(tools::uncipher_key_vec(&nearest_key, orig_ciphertext));
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

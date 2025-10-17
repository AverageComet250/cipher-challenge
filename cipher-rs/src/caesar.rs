use crate::{decipher_tools, dictionary};

pub fn decipher(orig_ciphertext: &str) -> Option<String> {
    let possible_ciphers = rotate_wheel(dictionary::ALPHABET_ARRAY.to_vec());

    for cipher in possible_ciphers {
        if let Some(unciphered) = decipher_tools::vec(cipher, orig_ciphertext) {
            return Some(unciphered);
        }
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

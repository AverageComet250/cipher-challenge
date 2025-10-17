use crate::{dictionary, tools};

pub fn decipher(orig_ciphertext: &str) -> Option<String> {
    let mult: [usize; 12] = [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25];
    let add = 0usize..=25usize;

    for a in add {
        for m in mult {
            let cipher = dictionary::ALPHABET_ARRAY
                .into_iter()
                .enumerate()
                .map(|(i, char)| (char, dictionary::ALPHABET_ARRAY[(i * m + a) % 26usize]))
                .collect();
            if let Some(unciphered) = tools::map(cipher, orig_ciphertext) {
                dbg!(a);
                dbg!(m);
                return Some(unciphered);
            }
        }
    }

    None
}

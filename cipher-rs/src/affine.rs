use crate::{dictionary, tools};

pub fn decipher(ciphertext: &str, aligned: bool) -> Option<String> {
    let mult: [usize; 12] = [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25];
    let add = 0usize..=25usize;

    for a in add {
        for m in mult {
            let cipher = dictionary::ALPHABET_ARRAY
                .into_iter()
                .enumerate()
                .map(|(i, char)| (char, dictionary::ALPHABET_ARRAY[(i * m + a) % 26usize]))
                .collect();

            let plaintext = if aligned {
                tools::uncipher_map_aligned(cipher, ciphertext)
            } else {
                tools::uncipher_map_non_aligned(cipher, ciphertext)
            };

            if let Some(plaintext) = plaintext {
                return Some(plaintext);
            }
        }
    }

    None
}

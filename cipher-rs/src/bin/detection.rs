use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Enter ciphertext below:");
    io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.to_lowercase();

    match cipher_rs::detection::autodetect(&buffer) {
        cipher_rs::CipherType::Monoalphabetic => {
            println!("Likely a monoalphabetic cipher (eg. keyword, affine, caesar).")
        }
        cipher_rs::CipherType::Polyalphabetic => {
            println!("Likely a polyalphabetic cipher (eg. Vigenere).")
        }
        cipher_rs::CipherType::Transposition => {
            println!("Likely a transposition cipher (eg. column transposition).")
        }
        _ => panic!("Cannot decipher this type of cipher,"),
    }
}

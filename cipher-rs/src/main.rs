use std::io::{self, Write};

use cipher_rs::{playfair, polyalphabetic};

fn main() {
    let aligned = input_bool("Is the text split into words by whitespaces? [y/n] ");

    let mut buffer = String::new();
    println!("Enter ciphertext below:");
    io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.to_lowercase();

    match cipher_rs::detection::autodetect(&buffer) {
        cipher_rs::CipherType::Monoalphabetic => monoalphabetic(&buffer, aligned),
        cipher_rs::CipherType::Polyalphabetic => polyalphabetic(&buffer, aligned),
        cipher_rs::CipherType::Playfair => playfair(&buffer),
        _ => panic!("Cannot decipher this type of cipher,"),
    }
}

fn playfair(ciphertext: &str) {
    println!("Recognised playfair cipher, beginning decryption process... ");
    let unciphered = playfair::decipher(ciphertext);
    match unciphered {
        Some(plaintext) => {
            println!("{}", plaintext);
        }
        None => println!("ERRORERRORERROR"),
    }
}

fn monoalphabetic(ciphertext: &str, aligned: bool) {
    println!("Recognised mono-alphabetic cipher, beginning decryption process...");

    println!("\n\nTrying caesar cipher");
    let unciphered = cipher_rs::caesar::decipher(ciphertext, aligned);
    match unciphered {
        Some(plaintext) => {
            println!("Found possible unciphered text:\n{}", plaintext);
            if !input_bool("Continue deciphering? [y/n] ") {
                return;
            }
        }
        None => println!("Could not uncipher this input using caesar cipher"),
    }

    println!("\n\nTrying affine cipher");
    let unciphered = cipher_rs::affine::decipher(ciphertext, aligned);
    match unciphered {
        Some(plaintext) => {
            println!("{}", plaintext);
            return;
        }
        None => println!("Could not uncipher this input using affine cipher"),
    }

    println!("\n\nTrying keyword cipher");
    let unciphered = cipher_rs::keyword::decipher(ciphertext, aligned);
    match unciphered {
        Some(plaintext) => {
            println!("{}", plaintext);
            return;
        }
        None => println!("Could not uncipher this input using keyword cipher"),
    }
    println!("Could not uncipher this input.");

    if input_bool("Try a poly-alphabetic decoding (Unlikely to work)? [y/n] ") {
        polyalphabetic(ciphertext, aligned);
    }
}

fn polyalphabetic(ciphertext: &str, aligned: bool) {
    println!("Recognised poly-alphabetic cipher, beginning decryption process...");
    let unciphered = polyalphabetic::decipher(ciphertext, aligned);
    match unciphered {
        Some(plaintext) => {
            println!("{}", plaintext);
        }
        None => println!("ERRORERRORERROR"),
    }
}

fn input_bool(prompt: &str) -> bool {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut buffer)
        .expect("Couldn't read from stdin");

    match buffer.trim().to_lowercase().as_str() {
        "y" | "yes" | "true" => true,
        "n" | "no" | "false" => false,
        _ => input_bool(prompt),
    }
}

use std::io::{self, Write};

fn main() {
    let aligned = input_bool("Is the text whitespaced into words [y/n]: ");

    let mut buffer = String::new();
    println!("Enter ciphertext below:");
    io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.to_lowercase();

    monoalphabetic(&buffer, aligned);
}

fn monoalphabetic(ciphertext: &str, aligned: bool) {
    println!("Beginning decryption process by monoalphabetic substitution...");

    let unciphered = cipher_rs::monoalphabetic::decipher(ciphertext, aligned);
    match unciphered {
        Some(plaintext) => {
            println!("{}", plaintext);
            return;
        }
        None => println!("Could not uncipher this input using monoalphabetic substitution cipher"),
    }
    if input_bool("Retry genetic algorithm, may work this time? [y/n] ") {
        monoalphabetic(ciphertext, aligned);
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

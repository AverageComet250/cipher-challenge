use std::io::{self, Write};

fn main() {
    let aligned = input_bool("Is the text split into words by whitespaces? [y/n] ");

    let mut buffer = String::new();
    println!("Enter ciphertext below:");
    io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.to_lowercase();

    if aligned {
        println!("Trying caesar cipher");
        let unciphered = cipher_rs::caesar::decipher(buffer.as_str());
        match unciphered {
            Some(plaintext) => {
                println!("{}", plaintext);
                return;
            }
            None => println!("Could not uncipher this input using caesar cipher"),
        }

        println!("Trying affine cipher");
        let unciphered = cipher_rs::affine::decipher(buffer.as_str());
        match unciphered {
            Some(plaintext) => {
                println!("{}", plaintext);
                return;
            }
            None => println!("Could not uncipher this input using affine cipher"),
        }

        println!("Trying keyword cipher");
        let unciphered = cipher_rs::keyword::decipher(buffer.as_str());
        match unciphered {
            Some(plaintext) => {
                println!("{}", plaintext);
                return;
            }
            None => println!("Could not uncipher this input using keyword cipher"),
        }
        println!("Could not uncipher this input. Try some other cipher");
    } else {
        panic!("Unaligned text not supported yet.")
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

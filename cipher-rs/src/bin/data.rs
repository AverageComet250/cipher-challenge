use std::io::{self, Write};

use cipher_rs::freq_analysis::*;

fn main() {
    let mut buffer = String::new();
    println!("Enter text to analyse below:");
    io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.to_lowercase();

    let clean_text_spaced: String = buffer
        .chars()
        .filter(|c| c.is_alphabetic() || c == &' ')
        .collect();
    let spaceless: String = buffer.chars().filter(|c| c.is_alphabetic()).collect();

    let chi2 = chi2(&spaceless);
    let bi = bigram_log_score(&clean_text_spaced);
    let tri = trigram_log_score(&clean_text_spaced);
    let ioc = index_of_coincidence(&spaceless);
    let entropy = entropy(&spaceless);
    let keylength = input_usize("Enter keylength: ");
    let kasiski = kasiski_score(&spaceless, keylength);
    let ac = autocorrelation_score(&spaceless, keylength);
    let friedman = friedman_score(&spaceless, keylength);
    let decomp = decomposition_score(&spaceless, keylength);

    println!();
    println!();
    println!("Metrics:");
    println!();
    println!();
    println!("Letter and n-gram frequencies:");
    println!("Chi^2: {chi2}, bigram_score: {bi}, trigram_score: {tri}");
    println!();
    println!("Autorecognition data: ");
    println!("IoC: {ioc}, Shannon entropy: {entropy}");
    println!();
    println!("Keylength metrics for vigenere (keylength={keylength}):");
    println!(
        "kasiski_score: {kasiski}, autocorrelation_score: {ac}, friedman_score: {friedman}, decomposition_score: {decomp}"
    )
}

fn input_usize(prompt: &str) -> usize {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut buffer)
        .expect("Couldn't read from stdin");

    match buffer.trim().parse::<usize>() {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            input_usize(prompt)
        }
    }
}

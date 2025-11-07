use std::io;

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
    println!(
        "Chi^2: {chi2}, bigram_score: {bi}, trigram_score: {tri}, Index of Coincidence: {ioc}, Entropy: {entropy}"
    );
}

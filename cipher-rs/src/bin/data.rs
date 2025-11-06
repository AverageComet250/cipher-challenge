use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Enter text to analyse below:");
    io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.to_lowercase();
    let (score, chi2, bi, tri, ioc, entrop) = cipher_rs::analysis::scores(&buffer);
    println!(
        "Score: {score}, Chi^2: {chi2}, bigram_score: {bi}, trigram_score: {tri}, Index of Coincidence: {ioc}, Entropy: {entrop}"
    );
}

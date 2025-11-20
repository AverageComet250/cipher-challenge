pub mod affine;
pub mod caesar;
pub mod detection;
pub mod dictionary;
pub mod freq_analysis;
pub mod keyword;
pub mod monoalphabetic;
pub mod polyalphabetic;
mod tools;

pub enum CipherType {
    Monoalphabetic,
    Polyalphabetic,
    Transposition,
    Unknown,
}

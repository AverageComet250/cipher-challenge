pub mod affine;
pub mod analysis;
pub mod caesar;
pub mod detection;
pub mod dictionary;
pub mod keyword;
pub mod monoalphabetic;
mod tools;

pub enum CipherType {
    Monoalphabetic,
    Polyalphabetic,
    Transposition,
    Unknown,
}

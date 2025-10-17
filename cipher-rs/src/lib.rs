pub mod affine;
pub mod caesar;
pub mod detection;
pub mod dictionary;
pub mod keyword;
mod tools;

pub enum CipherType {
    Monoalphabetic,
    Polyalphabetic,
    Unknown,
}

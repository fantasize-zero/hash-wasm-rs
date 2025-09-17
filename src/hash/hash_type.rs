use std::fmt::{Display, Formatter, Result};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HashType {
    MD5 = 0,
    BLAKE3 = 1,
    SHA256 = 2,
    SHA512 = 3,
    SHA3_256 = 4,
    SHA3_512 = 5,
}

impl Display for HashType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            HashType::MD5 => write!(f, "MD5"),
            HashType::BLAKE3 => write!(f, "Blake3"),
            HashType::SHA256 => write!(f, "SHA256"),
            HashType::SHA512 => write!(f, "SHA512"),
            HashType::SHA3_256 => write!(f, "SHA3-256"),
            HashType::SHA3_512 => write!(f, "SHA3-512"),
        }
    }
}

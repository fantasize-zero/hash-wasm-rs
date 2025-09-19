use std::fmt::{Display, Formatter, Result};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HashType {
    MD5 = 0,
    BLAKE3 = 1,
    SHA224 = 2,
    SHA256 = 3,
    SHA384 = 4,
    SHA512 = 5,
    SHA3_224 = 6,
    SHA3_256 = 7,
    SHA3_384 = 8,
    SHA3_512 = 9,
}

impl Display for HashType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            HashType::MD5 => write!(f, "MD5"),
            HashType::BLAKE3 => write!(f, "Blake3"),
            HashType::SHA224 => write!(f, "SHA224"),
            HashType::SHA256 => write!(f, "SHA256"),
            HashType::SHA384 => write!(f, "SHA384"),
            HashType::SHA512 => write!(f, "SHA512"),
            HashType::SHA3_224 => write!(f, "SHA3-224"),
            HashType::SHA3_256 => write!(f, "SHA3-256"),
            HashType::SHA3_384 => write!(f, "SHA3-384"),
            HashType::SHA3_512 => write!(f, "SHA3-512"),
        }
    }
}

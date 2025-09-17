use blake3::Hasher;
use hex::ToHex;
use md5::{Digest, Md5};
use sha2::{Sha256, Sha512};
use sha3::{Sha3_256, Sha3_512};
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Uint8Array;

use crate::hash::{hash_result::HashResult, hash_type::HashType};

pub const CHUNK_SIZE: usize = 1024 * 1024;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HasherWrapperConfig {
    pub hash_type: HashType,
    pub chunk_size: usize,
}

#[wasm_bindgen]
impl HasherWrapperConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(hash_type: HashType, chunk_size: usize) -> HasherWrapperConfig {
        HasherWrapperConfig {
            hash_type,
            chunk_size,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct HasherWrapper {
    input: JsValue,
    hash_type: HashType,
    md5: Option<Md5>,
    blake3: Option<Hasher>,
    sha256: Option<Sha256>,
    sha512: Option<Sha512>,
    sha3_256: Option<Sha3_256>,
    sha3_512: Option<Sha3_512>,
}

#[wasm_bindgen]
impl HasherWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(hash_type: HashType, input: &JsValue) -> HasherWrapper {
        let mut wrapper = HasherWrapper {
            input: input.into(),
            hash_type,
            sha256: None,
            sha512: None,
            sha3_256: None,
            sha3_512: None,
            md5: None,
            blake3: None,
        };

        match hash_type {
            HashType::MD5 => wrapper.md5 = Some(Md5::new()),
            HashType::BLAKE3 => wrapper.blake3 = Some(Hasher::new()),
            HashType::SHA256 => wrapper.sha256 = Some(Sha256::new()),
            HashType::SHA512 => wrapper.sha512 = Some(Sha512::new()),
            HashType::SHA3_256 => wrapper.sha3_256 = Some(Sha3_256::new()),
            HashType::SHA3_512 => wrapper.sha3_512 = Some(Sha3_512::new()),
        }

        wrapper
    }

    pub fn update(&mut self) {
        let unit8_array = Uint8Array::new(&self.input).to_vec();

        match self.hash_type {
            HashType::MD5 => self
                .md5
                .as_mut()
                .expect("MD5 hasher not initialized")
                .update(&unit8_array),
            HashType::BLAKE3 => {
                self.blake3
                    .as_mut()
                    .expect("BLAKE3 hasher not initialized")
                    .update(&unit8_array);
            }
            HashType::SHA256 => self
                .sha256
                .as_mut()
                .expect("SHA256 hasher not initialized")
                .update(&unit8_array),
            HashType::SHA512 => self
                .sha512
                .as_mut()
                .expect("SHA512 hasher not initialized")
                .update(&unit8_array),
            HashType::SHA3_256 => self
                .sha3_256
                .as_mut()
                .expect("SHA3-256 hasher not initialized")
                .update(&unit8_array),
            HashType::SHA3_512 => self
                .sha3_512
                .as_mut()
                .expect("SHA3-512 hasher not initialized")
                .update(&unit8_array),
        }
    }

    pub fn finalize(&mut self) -> Vec<u8> {
        match self.hash_type {
            HashType::MD5 => self
                .md5
                .as_mut()
                .expect("MD5 hasher not initialized")
                .clone()
                .finalize()
                .to_vec(),
            HashType::BLAKE3 => self
                .blake3
                .as_mut()
                .expect("BLAKE3 hasher not initialized")
                .clone()
                .finalize()
                .as_bytes()
                .to_vec(),
            HashType::SHA256 => self
                .sha256
                .as_mut()
                .expect("SHA256 hasher not initialized")
                .clone()
                .finalize()
                .to_vec(),
            HashType::SHA512 => self
                .sha512
                .as_mut()
                .expect("SHA512 hasher not initialized")
                .clone()
                .finalize()
                .to_vec(),
            HashType::SHA3_256 => self
                .sha3_256
                .as_mut()
                .expect("SHA3-256 hasher not initialized")
                .clone()
                .finalize()
                .to_vec(),
            HashType::SHA3_512 => self
                .sha3_512
                .as_mut()
                .expect("SHA3-512 hasher not initialized")
                .clone()
                .finalize()
                .to_vec(),
        }
    }

    #[wasm_bindgen(js_name = result)]
    pub fn hash_result(&mut self) -> Result<HashResult, JsValue> {
        self.update();
        let result = self.finalize();
        Ok(HashResult {
            hex: result.encode_hex(),
            bytes: result.to_vec(),
        })
    }
}

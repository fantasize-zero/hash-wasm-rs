use crate::hash::{hash_result::HashResult, hash_type::HashType, hasher_wrapper::HasherWrapper};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sha256_hash(data: &JsValue) -> Result<HashResult, JsValue> {
    let mut hasher = HasherWrapper::new(HashType::SHA256, data);
    hasher.hash_result()
}

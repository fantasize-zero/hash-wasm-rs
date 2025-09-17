use crate::hash::{hash_result::HashResult, hash_type::HashType, hasher_wrapper::HasherWrapper};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sha512_hash(data: &JsValue) -> Result<HashResult, JsValue> {
    let mut hasher = HasherWrapper::new(HashType::SHA512, data);
    hasher.hash_result()
}

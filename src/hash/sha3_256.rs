use crate::hash::{hash_result::HashResult, hash_type::HashType, hasher_wrapper::HasherWrapper};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = sha3_256)]
pub fn sha3_256_hash(data: &JsValue) -> Result<HashResult, JsValue> {
    let mut hasher = HasherWrapper::new(HashType::SHA3_256, data);
    hasher.hash_result()
}

use crate::hash::{hash_result::HashResult, hash_type::HashType, hasher_wrapper::HasherWrapper};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = sha3_512)]
pub fn sha3_512_hash(data: &JsValue) -> Result<HashResult, JsValue> {
    let mut hasher = HasherWrapper::new(HashType::SHA3_512, data);
    hasher.hash_result()
}

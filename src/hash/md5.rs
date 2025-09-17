use crate::hash::{hash_result::HashResult, hash_type::HashType, hasher_wrapper::HasherWrapper};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = md5)]
pub fn md5_hash(data: &JsValue) -> Result<HashResult, JsValue> {
    let mut hasher = HasherWrapper::new(HashType::MD5, data);
    hasher.hash_result()
}

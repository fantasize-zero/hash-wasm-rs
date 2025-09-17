use crate::hash::{hash_result::HashResult, hash_type::HashType, hasher_wrapper::HasherWrapper};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = blake3)]
pub async fn blake3_hash(data: &JsValue) -> Result<HashResult, JsValue> {
    HasherWrapper::new(HashType::BLAKE3, data)
        .hash_result()
        .await
}

use crate::hash::{hash_result::HashResult, hash_type::HashType, hasher_wrapper::HasherWrapper};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn sha256_hash(data: &JsValue) -> Result<HashResult, JsValue> {
    HasherWrapper::new(HashType::SHA256, data)
        .hash_result()
        .await
}

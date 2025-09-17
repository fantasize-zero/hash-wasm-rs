use crate::hash::{hash_result::HashResult, hash_type::HashType, hasher_wrapper::HasherWrapper};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn sha512_hash(data: &JsValue) -> Result<HashResult, JsValue> {
    HasherWrapper::new(HashType::SHA512, data)
        .hash_result()
        .await
}

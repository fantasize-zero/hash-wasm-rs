use crate::hash::{hash_result::HashResult, hash_type::HashType, hasher_wrapper::HasherWrapper};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = sha_256)]
pub async fn sha_256_hash(data: &JsValue) -> Result<HashResult, JsValue> {
    HasherWrapper::new(HashType::SHA256, data)
        .hash_result()
        .await
}

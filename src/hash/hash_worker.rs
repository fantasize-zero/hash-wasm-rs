use crate::{hash::hash_type::HashType, prelude::HasherWrapper};
use hex::ToHex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct HashWorker {
    hash_type: HashType,
    hasher: HasherWrapper,
    chunk_data: JsValue,
}

#[wasm_bindgen]
impl HashWorker {
    #[wasm_bindgen(constructor)]
    pub fn new(hash_type: HashType, chunk_data: &JsValue) -> Self {
        HashWorker {
            hash_type,
            hasher: HasherWrapper::new(hash_type, &chunk_data),
            chunk_data: chunk_data.into(),
        }
    }

    // 处理单个分片
    #[wasm_bindgen]
    pub fn process_chunk(&mut self) -> Result<String, JsValue> {
        let result = HasherWrapper::new(self.hash_type, &self.chunk_data).hash_result()?;

        self.hasher.update();

        Ok(result.hex)
    }

    // 获取最终结果（整体文件的哈希）
    #[wasm_bindgen]
    pub fn finalize(&mut self) -> String {
        let result = self.hasher.clone().finalize();
        result.encode_hex()
    }
}

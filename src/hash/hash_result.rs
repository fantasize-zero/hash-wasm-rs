use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default, Clone, Debug, PartialEq, serde::Serialize)]
pub struct HashResult {
    pub(crate) hex: String,
    pub(crate) bytes: Vec<u8>,
}

#[wasm_bindgen]
impl HashResult {
    // #[wasm_bindgen(constructor)]
    // pub fn new(hex: String, bytes: Vec<u8>) -> HashResult {
    //     HashResult { hex, bytes }
    // }

    #[wasm_bindgen(getter)]
    pub fn hex(&self) -> String {
        self.hex.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}

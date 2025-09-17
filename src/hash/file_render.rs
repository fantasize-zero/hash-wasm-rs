use wasm_bindgen::prelude::*;
use web_sys::{
    Blob, File, FileReader,
    js_sys::{ArrayBuffer, JsString, Uint8Array},
};

#[derive(Debug, Clone, PartialEq)]
#[wasm_bindgen]
pub struct GetFileResult {
    #[wasm_bindgen(js_name = arrayBuffer)]
    pub(crate) array_buffer: ArrayBuffer,
    #[wasm_bindgen(js_name = uint8Array)]
    pub(crate) uint8_array: Vec<u8>,
    pub(crate) blob: Blob,
}

#[wasm_bindgen]
pub fn get_file_uint8array_and_blob(file: &File) -> Result<GetFileResult, JsValue> {
    let file_render = FileReader::new()?;
    file_render.read_as_array_buffer(&file)?;
    let js_array_buffer: JsValue = file_render.result()?;
    let array_buffer: ArrayBuffer = js_array_buffer.dyn_into()?;

    let uint8_array = Uint8Array::new(&array_buffer);
    let blob = Blob::new_with_u8_array_sequence(&uint8_array)?;

    Ok(GetFileResult {
        array_buffer: array_buffer,
        uint8_array: uint8_array.to_vec(),
        blob: blob,
    })
}

pub fn get_data_as_bytes(data: &JsValue) -> Result<Vec<u8>, JsValue> {
    if let Some(uint8_array) = data.dyn_ref::<Uint8Array>() {
        Ok(uint8_array.to_vec())
    } else if let Some(file) = data.dyn_ref::<File>() {
        let file_result = get_file_uint8array_and_blob(&file)?;
        Ok(file_result.uint8_array)
    } else if let Some(js_string) = data.dyn_ref::<JsString>() {
        let string: String = js_string.into();
        Ok(string.as_bytes().to_vec())
    } else {
        Err(JsValue::from_str(
            "Invalid input type to hash, should be Uint8Array, File or String",
        ))
    }
}

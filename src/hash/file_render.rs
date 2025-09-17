use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
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
pub async fn get_file_uint8array_and_blob(file: &File) -> Result<GetFileResult, JsValue> {
    let file_reader = FileReader::new()?;
    let promise = Promise::new(&mut |resolve, reject| {
        file_reader.set_onload(Some(&resolve));
        file_reader.set_onerror(Some(&reject));
        if let Err(e) = file_reader.read_as_array_buffer(file) {
            let _ = reject.call1(&JsValue::NULL, &e);
        }
    });
    let _ = JsFuture::from(promise).await?;

    let js_array_buffer: JsValue = file_reader.result()?;
    let array_buffer: ArrayBuffer = js_array_buffer.dyn_into()?;

    let uint8_array = Uint8Array::new(&array_buffer);
    let blob = Blob::new_with_u8_array_sequence(&uint8_array)?;

    Ok(GetFileResult {
        array_buffer,
        uint8_array: uint8_array.to_vec(),
        blob,
    })
}

// 新增：分块处理文件的函数
#[wasm_bindgen]
pub async fn process_file_in_chunks(
    file: &File,
    chunk_size: usize,
    process_chunk: js_sys::Function,
) -> Result<JsValue, JsValue> {
    let total_size = file.size() as usize;
    let mut offset = 0;

    while offset < total_size {
        let end = std::cmp::min(offset + chunk_size, total_size);
        let blob = file.slice_with_i32_and_i32(offset as i32, end as i32)?;

        // 读取当前块
        let file_reader = FileReader::new()?;
        let promise = js_sys::Promise::new(&mut |resolve, reject| {
            file_reader.set_onload(Some(&resolve));
            file_reader.set_onerror(Some(&reject));
            if let Err(e) = file_reader.read_as_array_buffer(&blob) {
                let _ = reject.call1(&JsValue::NULL, &e);
            }
        });

        let _ = JsFuture::from(promise).await?;
        let js_array_buffer: JsValue = file_reader.result()?;
        let array_buffer: ArrayBuffer = js_array_buffer.dyn_into()?;
        let chunk_data = Uint8Array::new(&array_buffer).to_vec();

        // 调用处理函数处理当前块
        let this = JsValue::NULL;
        let chunk_js = Uint8Array::from(chunk_data.as_slice()).into();
        let offset_js = JsValue::from(offset as f64);
        let total_js = JsValue::from(total_size as f64);

        process_chunk.call3(&this, &chunk_js, &offset_js, &total_js)?;

        offset = end;
    }

    Ok(JsValue::UNDEFINED)
}

pub async fn get_data_as_bytes(data: &JsValue) -> Result<Vec<u8>, JsValue> {
    if let Some(uint8_array) = data.dyn_ref::<Uint8Array>() {
        Ok(uint8_array.to_vec())
    } else if let Some(file) = data.dyn_ref::<File>() {
        let file_result = get_file_uint8array_and_blob(file).await?;
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

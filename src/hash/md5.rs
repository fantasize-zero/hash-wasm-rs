use crate::hash::hash_result::HashResult;
use crate::prelude::{HashType, HasherWrapper};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = md5)]
pub async fn md5_hash(data: &JsValue) -> Result<HashResult, JsValue> {
    // match HasherWrapper::new(HashType::MD5, data).hash_result().await {
    //     Ok(result) => Ok(result),
    //     Err(e) => {
    //         // 使用 as_string() 方法来转换 JsValue 为 String
    //         let error_message = e.as_string().unwrap_or_else(|| "Unknown error".to_string());
    //         Err(JsValue::from_str(&error_message))
    //     }
    // }
    HasherWrapper::new(HashType::MD5, data).hash_result().await
}

// #[wasm_bindgen(js_name = md5)]
// pub async fn md5_hash(data: &JsValue) -> Result<HashResult, JsValue> {
//     // 尝试获取完整数据
//     let data_array = get_data_as_bytes(data).await?;

//     // 如果数据不为空，说明是小文件，直接计算
//     if !data_array.is_empty() {
//         return compute_md5(&data_array);
//     }

//     // 如果是大文件，使用分块处理
//     if let Some(file) = data.dyn_ref::<web_sys::File>() {
//         // 使用Rc<RefCell>来在闭包中共享和修改hasher
//         let hasher = Rc::new(RefCell::new(Md5::new()));
//         let hasher_clone = Rc::clone(&hasher);

//         // 创建处理每个块的闭包
//         let closure = Closure::wrap(Box::new(
//             move |chunk: JsValue, _offset: JsValue, _total: JsValue| {
//                 if let Ok(uint8_array) = chunk.dyn_into::<Uint8Array>() {
//                     let chunk_data = uint8_array.to_vec();
//                     hasher_clone.borrow_mut().update(&chunk_data);
//                 }
//             },
//         ) as Box<dyn FnMut(JsValue, JsValue, JsValue)>);

//         let process_chunk = closure.as_ref().unchecked_ref::<js_sys::Function>();

//         // 设置分块大小（256KB）
//         let chunk_size = 256 * 1024;

//         // 处理文件分块
//         process_file_in_chunks(file, chunk_size, process_chunk.clone()).await?;

//         // 完成哈希计算
//         let result = hasher.borrow_mut().finalize_reset();

//         // 释放闭包
//         drop(closure);

//         Ok(HashResult {
//             hex: result.encode_hex::<String>(),
//             bytes: result.to_vec(),
//         })
//     } else {
//         // 如果不是文件，但数据为空，说明有错误
//         compute_md5(&data_array)
//     }
// }
// fn compute_md5(data: &[u8]) -> Result<HashResult, JsValue> {
//     let mut hasher = Md5::new();
//     hasher.update(data);
//     let result = hasher.finalize();
//     Ok(HashResult {
//         hex: result.encode_hex::<String>(),
//         bytes: result.to_vec(),
//     })
// }

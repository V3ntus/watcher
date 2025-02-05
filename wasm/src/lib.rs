use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn load_kismetdb(file_content: Vec<u8>) -> String {
    format!("[watcher-lib.wasm] File size is {} bytes.", file_content.len())
}

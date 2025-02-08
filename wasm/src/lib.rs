mod kismet;

use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;

#[wasm_bindgen(start)]
fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Initialized watcher-lib wasm module");
}

#[wasm_bindgen]
pub fn deserialize_kismetdb(file_content: Vec<u8>) -> JsValue {
    match kismet::db::load_kismetdb(file_content.as_slice()) {
        Ok(v) => serde_wasm_bindgen::to_value(&v).unwrap(),
        Err(e) => throw_str(e.to_string().as_str())
    }
}

mod kismet;

use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;

#[wasm_bindgen]
pub fn load_kismetdb(file_content: Vec<u8>) -> String {
    let result = match kismet::db::load_kismetdb(file_content.as_slice()) {
        Ok(v) => v,
        Err(e) => throw_str(e.to_string().as_str())
    };

    let mut fmt_data = Vec::new();

    for r in result {
        fmt_data.push(format!("{}", r));
    }

    format!("{}", fmt_data.join("\n"))
}

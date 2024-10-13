// wasm/main.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_threat_data(data: &str) -> f32 {
    // WebAssembly logic to handle real-time processing
    if data == "malware" {
        0.9
    } else {
        0.1
    }
}

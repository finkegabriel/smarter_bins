use wasm_bindgen::prelude::*;
use image::GrayImage;
use rqrr::PreparedImage;
use serde::Serialize;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
pub fn get_qr_results(decoded: Vec<String>) -> JsValue {
    to_value(&decoded).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, from wasm-qr-reader!");
}
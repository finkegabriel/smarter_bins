use wasm_bindgen::prelude::*;
use image::GrayImage;
use rqrr::PreparedImage;
use serde::Serialize;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
pub fn detect_qr_from_rgba(data: &[u8], width: u32, height: u32) -> String {
    // Convert RGBA to grayscale
    let gray_pixels: Vec<u8> = data.chunks(4)
        .map(|px| {
            // Use luminosity method
            (0.299 * px[0] as f32 + 0.587 * px[1] as f32 + 0.114 * px[2] as f32) as u8
        })
        .collect();

    if let Some(gray_img) = GrayImage::from_raw(width, height, gray_pixels) {
        let mut prepared = PreparedImage::prepare(gray_img);
        let grids = prepared.detect_grids();

        for grid in grids {
            if let Ok((_, content)) = grid.decode() {
                return content;
            }
        }
    }

    "".into()
}



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
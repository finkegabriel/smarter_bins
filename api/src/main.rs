use image::{GrayImage, ImageBuffer, Luma, Rgb, RgbImage};
use rqrr::PreparedImage;
use wasm_bindgen::prelude::*;

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


fn main() -> Result<(), Box<dyn std::error::Error>> {
	
    Ok(())
}

fn yuyv_to_gray(frame: &[u8], width: u32, height: u32) -> GrayImage {
    let mut buffer = Vec::with_capacity((width * height) as usize);
    for chunk in frame.chunks(4) {
        buffer.push(chunk[0]); // Y1
        buffer.push(chunk[2]); // Y2
    }
    GrayImage::from_raw(width, height, buffer).unwrap()
}

fn gray_to_rgb(gray: &GrayImage) -> RgbImage {
    let (w, h) = gray.dimensions();
    ImageBuffer::from_fn(w, h, |x, y| {
        let Luma([v]) = gray.get_pixel(x, y);
        Rgb([*v, *v, *v])
    })
}

fn draw_line(img: &mut RgbImage, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgb<u8>) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && y0 >= 0 && x0 < img.width() as i32 && y0 < img.height() as i32 {
            img.put_pixel(x0 as u32, y0 as u32, color);
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

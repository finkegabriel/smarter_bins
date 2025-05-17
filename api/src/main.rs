use opencv::{
    prelude::*,
    highgui,
    core,
    Result,
    imgproc::{self, ColorConversionCodes},
    imgcodecs,
};
use image::GrayImage;
use rqrr::PreparedImage;

fn run() -> Result<()> {
    let image_path = "/Users/sync/code/smarter_bins/api/src/bins_3.jpg";
    let frame = imgcodecs::imread(image_path, imgcodecs::IMREAD_COLOR)?;
    let mut gray = core::Mat::default();

	imgproc::cvt_color(
		&frame,
		&mut gray,
		ColorConversionCodes::COLOR_BGR2GRAY as i32,
		0, // this is the `dstCn` argument (number of channels)
		unsafe { std::mem::zeroed() }, // ⚠️ Use at your own risk
	)?;
	
    let mut gray_cont = core::Mat::default();
    gray.copy_to(&mut gray_cont)?;

    let width = gray_cont.cols() as u32;
    let height = gray_cont.rows() as u32;
    let bytes = gray_cont.data_bytes()?;

    if let Some(gray_image) = GrayImage::from_raw(width, height, bytes.to_vec()) {
        let mut prepared = PreparedImage::prepare(gray_image);
        let grids = prepared.detect_grids();
        for grid in grids {
            if let Ok((_, content)) = grid.decode() {
                println!("QR Code content: {}", content);
            }
        }
    }

    // Display the image with detected QR code
    let window = "QR Code Reader";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    highgui::imshow(window, &frame)?;
    highgui::wait_key(0)?;

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => println!("Program finished successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
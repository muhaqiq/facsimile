use base64::encode;
use image::ImageFormat::Jpeg;
use image::RgbImage;
use imageproc::drawing::draw_line_segment_mut;
use std::io::Cursor;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
trait ConvertToPixels {
    fn to_pixels(self) -> u32;
}

impl ConvertToPixels for u32 {
    fn to_pixels(self) -> u32 {
        self * 10
    }
}

pub trait GenerationPipeline {
    fn create_image(w: u32, h: u32) -> RgbImage;
    fn draw_grid(self) -> RgbImage;
    fn encode_as_base64(self) -> String;
}

impl GenerationPipeline for RgbImage {
    fn create_image(w: u32, h: u32) -> RgbImage {
        let mut im: RgbImage = image::ImageBuffer::new(w.to_pixels(), h.to_pixels());
        for pixel in im.pixels_mut() {
            *pixel = image::Rgb([227, 221, 202]);
        }
        im
    }
    fn draw_grid(mut self) -> RgbImage {
        let h = self.height();
        let w = self.width();
        for point in (0..w).step_by(50) {
            draw_line_segment_mut(
                &mut self,
                (point as f32, 0.0),
                (point as f32, h as f32),
                image::Rgb([0, 0, 0]),
            )
        }
        for point in (0..h).step_by(50) {
            draw_line_segment_mut(
                &mut self,
                (0.0, point as f32),
                (w as f32, point as f32),
                image::Rgb([0, 0, 0]),
            )
        }
        self
    }

    fn encode_as_base64(self) -> String {
        let mut cursor = Cursor::new(Vec::new());
        self.write_to(&mut cursor, Jpeg).unwrap();
        let encoded_img = encode(cursor.get_ref());
        format!("data:image/jpeg;base64,{}", encoded_img)
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn generate_facsimile(w: u32, h: u32) -> String {
    RgbImage::create_image(w, h).draw_grid().encode_as_base64()
}

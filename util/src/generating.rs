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

trait GenerationPipeline {
    fn create_image(w: u32, h: u32, color: [u8; 3]) -> RgbImage;
    fn draw_grid(self, color: [u8; 3]) -> RgbImage;
    fn encode_as_base64(self) -> String;
}

impl GenerationPipeline for RgbImage {
    fn create_image(w: u32, h: u32, color: [u8; 3]) -> RgbImage {
        let mut im: RgbImage = image::ImageBuffer::new(w.to_pixels(), h.to_pixels());
        for pixel in im.pixels_mut() {
            *pixel = image::Rgb(color);
        }
        im
    }
    fn draw_grid(mut self, color: [u8; 3]) -> RgbImage {
        let h = self.height();
        let w = self.width();
        for point in (0..w).step_by(50) {
            draw_line_segment_mut(
                &mut self,
                (point as f32, 0.0),
                (point as f32, h as f32),
                image::Rgb(color),
            )
        }
        for point in (0..h).step_by(50) {
            draw_line_segment_mut(
                &mut self,
                (0.0, point as f32),
                (w as f32, point as f32),
                image::Rgb(color),
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

const A4_WIDTH: u32 = 210;
const A4_HEIGHT: u32 = 297;
const PARCHAMENT_YELLOW: [u8; 3] = [227, 221, 202];
const BLACK: [u8; 3] = [0, 0, 0];

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn generate_facsimile(
    w: Option<u32>,
    h: Option<u32>,
    bg_color: Option<Box<[u8]>>,
    grid_color: Option<Box<[u8]>>,
) -> String {
    let w = w.unwrap_or(A4_WIDTH);
    let h = h.unwrap_or(A4_HEIGHT);
    let bg_color = if let Some(value) = bg_color {
        [value[0], value[1], value[2]]
    } else {
        PARCHAMENT_YELLOW
    };
    let grid_color = if let Some(value) = grid_color {
        [value[0], value[1], value[2]]
    } else {
        BLACK
    };
    RgbImage::create_image(w, h, bg_color)
        .draw_grid(grid_color)
        .encode_as_base64()
}

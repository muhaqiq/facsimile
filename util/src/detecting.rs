use base64::decode;
use image::imageops::{crop, replace};
use image::{load_from_memory_with_format, ImageFormat, RgbImage};

use crate::helpers::detect_lines;
use crate::rect::Rect;
use crate::transforms::{binarize, crop_region, rotate};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct LineDetector {
    pub(crate) data: RgbImage,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl LineDetector {
    pub fn new(encoded_file: &str) -> Self {
        let base64_to_vector = decode(encoded_file).unwrap();
        let data = load_from_memory_with_format(base64_to_vector.as_slice(), ImageFormat::Jpeg)
            .unwrap()
            .into_rgb8();

        LineDetector { data }
    }

    pub fn detect_lines_in_region(&self, region: &[u32], thresh: u8, density: u8) -> Box<[u32]> {
        let cropped = self.crop_region(&region[0..=7], region[8] as f32);
        let binarized = binarize(&cropped, thresh);
        detect_lines(binarized, density).into_boxed_slice()
    }

    fn get_padded_image(&self) -> RgbImage {
        let mut im: RgbImage =
            image::ImageBuffer::new(self.data.width() + 400, self.data.height() + 400);
        for pixel in im.pixels_mut() {
            *pixel = image::Rgb([255, 255, 255]);
        }
        replace(&mut im, &self.data, 200, 200);
        im
    }

    fn crop_bounding_area(&self, rect: &Rect, r: f32, padding: u32) -> RgbImage {
        let bounding_rect = rect.get_bounding_rect(r.round() as i32);
        let mut src = self.get_padded_image();
        crop(
            &mut src,
            if padding < bounding_rect.x + 200 {
                bounding_rect.x + 200 - padding
            } else {
                0
            },
            if padding < bounding_rect.y + 200 {
                bounding_rect.y + 200 - padding
            } else {
                0
            },
            bounding_rect.width + 2 * padding,
            bounding_rect.height + 2 * padding,
        )
        .to_image()
    }

    fn crop_region(&self, p: &[u32], r: f32) -> RgbImage {
        let rect = Rect::from_polygon(p);
        let bounding_area = self.crop_bounding_area(&rect, r, 0);
        let rotated = rotate(bounding_area, -r as f32);
        crop_region(rotated, rect.width, rect.height, 0)
    }
}

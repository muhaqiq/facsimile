use base64::{decode, encode};
use image::imageops::{crop, replace};
use image::ImageOutputFormat::Png;
use image::{load_from_memory_with_format, ImageFormat, RgbImage};
use std::io::Cursor;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::rect::Rect;
use crate::transforms::{add_frame, crop_region, rotate};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct FacsimileCropper {
    pub(crate) data: RgbImage,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl FacsimileCropper {
    pub fn new(encoded_file: &str) -> Self {
        let base64_to_vector = decode(encoded_file).unwrap();
        let data = load_from_memory_with_format(base64_to_vector.as_slice(), ImageFormat::Jpeg)
            .unwrap()
            .into_rgb8();

        FacsimileCropper { data }
    }

    pub fn get_url(&self) -> String {
        let mut cursor = Cursor::new(Vec::new());
        self.data.write_to(&mut cursor, Png).unwrap();
        let encoded_img = encode(cursor.get_ref());
        format!("data:image/png;base64,{}", encoded_img)
    }

    fn get_padded_image(&self, padding: u32) -> RgbImage {
        let mut im: RgbImage = image::ImageBuffer::new(
            self.data.width() + 400 + padding,
            self.data.height() + 400 + padding,
        );
        for pixel in im.pixels_mut() {
            *pixel = image::Rgb([255, 255, 255]);
        }
        replace(&mut im, &self.data, 200, 200);
        im
    }

    fn crop_bounding_area(&self, rect: &Rect, r: f32, padding: u32) -> RgbImage {
        let bounding_rect = rect.get_bounding_rect(r.round() as i32);
        let mut src = self.get_padded_image(padding);
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

    pub fn get_region(
        &self,
        p: &[u32],
        r: f32,
        frame_color: &[u32],
        padding_percentage: Option<f32>,
    ) -> String {
        let rect = Rect::from_polygon(p);
        let padding = rect.calculate_padding(padding_percentage);
        let bounding_area = self.crop_bounding_area(&rect, r, padding);
        let rotated = rotate(bounding_area, -r as f32);
        let mut region = crop_region(rotated, rect.width, rect.height, padding);
        add_frame(&mut region, rect.width, rect.height, padding, frame_color);
        let mut cursor = Cursor::new(Vec::new());
        region.write_to(&mut cursor, Png).unwrap();
        let encoded_img = encode(cursor.get_ref());
        format!("data:image/png;base64,{}", encoded_img)
    }
}

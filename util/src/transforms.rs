use image::imageops::{crop, grayscale};
use image::{GrayImage, Rgb, RgbImage};
use imageproc::contrast::threshold;
use imageproc::drawing::Canvas;
use imageproc::map::map_pixels_mut;
use imageproc::pixelops::interpolate;
use std::cmp;

pub fn rotate(src: RgbImage, degree: f32) -> RgbImage {
    let w1 = src.width() as i32;
    let h1 = src.height() as i32;

    // Using screen coords system, top left is always at (0,0)
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    let top_right_1: (i32, i32) = (w1, 0);
    let top_right_2: (i32, i32) = _rotate(top_right_1, degree);
    min_x = cmp::min(min_x, top_right_2.0);
    max_x = cmp::max(max_x, top_right_2.0);
    min_y = cmp::min(min_y, top_right_2.1);
    max_y = cmp::max(max_y, top_right_2.1);

    let bottom_right_1: (i32, i32) = (w1, h1);
    let bottom_right_2: (i32, i32) = _rotate(bottom_right_1, degree);
    min_x = cmp::min(min_x, bottom_right_2.0);
    max_x = cmp::max(max_x, bottom_right_2.0);
    min_y = cmp::min(min_y, bottom_right_2.1);
    max_y = cmp::max(max_y, bottom_right_2.1);

    let bottom_left_1: (i32, i32) = (0, h1);
    let bottom_left_2: (i32, i32) = _rotate(bottom_left_1, degree);
    min_x = cmp::min(min_x, bottom_left_2.0);
    max_x = cmp::max(max_x, bottom_left_2.0);
    min_y = cmp::min(min_y, bottom_left_2.1);
    max_y = cmp::max(max_y, bottom_left_2.1);

    let w2 = ((min_x as f32).abs() + (max_x as f32).abs()) as i32 + 1;
    let h2 = ((min_y as f32).abs() + (max_y as f32).abs()) as i32 + 1;
    let mut dest: RgbImage = image::ImageBuffer::new(w2 as u32, h2 as u32);

    for (dest_y, y) in (0..).zip(min_y..max_y + 1) {
        for (dest_x, x) in (0..).zip(min_x..max_x + 1) {
            let point: (i32, i32) = _rotate((x, y), -degree);

            if point.0 >= 0 && point.0 < w1 && point.1 >= 0 && point.1 < h1 {
                let pixel = src.get_pixel(point.0 as u32, point.1 as u32);
                dest.draw_pixel(dest_x, dest_y, *pixel);
            } else {
                dest.draw_pixel(dest_x, dest_y, Rgb([255, 255, 255]));
            }
        }
    }

    dest
}

fn _rotate(p: (i32, i32), deg: f32) -> (i32, i32) {
    let radians: f32 = deg.to_radians();
    let px: f32 = p.0 as f32;
    let py: f32 = p.1 as f32;
    let cos = radians.cos();
    let sin = radians.sin();
    let x = ((px * cos) - (py * sin)).round();
    let y = ((px * sin) + (py * cos)).round();
    (x as i32, y as i32)
}

pub fn crop_region(mut src: RgbImage, w: u32, h: u32, p: u32) -> RgbImage {
    let w = w + 2 * p;
    let h = h + 2 * p;
    let x = (src.width() - w) / 2;
    let y = (src.height() - h) / 2;

    crop(&mut src, x, y, w, h).to_image()
}

pub fn add_frame(src: &mut RgbImage, w: u32, h: u32, pad: u32, frame_color: &[u32]) {
    let color = Rgb::from([
        frame_color[0] as u8,
        frame_color[1] as u8,
        frame_color[2] as u8,
    ]);
    let is_pixel_in_frame = |x: u32, y: u32| y < pad || y > pad + h || x < pad || x > pad + w;
    map_pixels_mut(src, |x, y, p| match is_pixel_in_frame(x, y) {
        true => interpolate(color, p, 0.7),
        false => p,
    });
}

pub fn binarize(data: &RgbImage, thresh: u8) -> GrayImage {
    let img = grayscale(data);
    threshold(&img, thresh)
}

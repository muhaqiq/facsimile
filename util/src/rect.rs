use super::helpers::{get_distance, Normalization};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn from_polygon(p: &[u32]) -> Self {
        Rect {
            x: p[0],
            y: p[1],
            width: get_distance((p[0], p[1]), (p[2], p[3])),
            height: get_distance((p[2], p[3]), (p[4], p[5])),
        }
    }

    pub fn calculate_padding(&self, percentage: Option<f32>) -> u32 {
        let sum_of_powers = (u32::pow(self.width, 2) + u32::pow(self.height, 2)) as f64;
        (sum_of_powers.sqrt() * percentage.unwrap_or(3.0) as f64 / 100.0).round() as u32
    }

    pub fn get_bounding_rect(&self, rotation: i32) -> Self {
        fn calc(degrees: i32) -> (f64, f64) {
            let rad = (degrees as f64).normalize_and_convert_to_radians();
            (rad.cos(), rad.sin())
        }
        let dimensions = |(cos, sin): (f64, f64)| {
            let l1 = (self.width as f64 * sin).abs() + (self.height as f64 * cos).abs();
            let l2 = (self.height as f64 * sin).abs() + (self.width as f64 * cos).abs();
            (l1.round() as u32, l2.round() as u32)
        };

        match rotation {
            0..=90 => {
                let (cos, sin) = calc(rotation);
                let (height, width) = dimensions((cos, sin));
                let d_x = (self.height as f64 * sin).abs().round() as u32;
                Rect {
                    x: if d_x < self.x { self.x - d_x } else { 0 },
                    y: self.y,
                    width,
                    height,
                }
            }
            91..=180 => {
                let (cos, sin) = calc(90 - rotation);
                let (width, height) = dimensions((cos, sin));
                let d_y = (self.height as f64 * sin).abs().round() as u32;
                Rect {
                    x: if width < self.x { self.x - width } else { 0 },
                    y: if d_y < self.y { self.y - d_y } else { 0 },
                    width,
                    height,
                }
            }
            181..=270 => {
                let (cos, sin) = calc(180 - rotation);
                let (height, width) = dimensions((cos, sin));
                let d_x = (self.width as f64 * cos).abs().round() as u32;
                Rect {
                    x: if d_x < self.x { self.x - d_x } else { 0 },
                    y: if height < self.y { self.y - height } else { 0 },
                    width,
                    height,
                }
            }
            271..=359 => {
                let (cos, sin) = calc(270 - rotation);
                let (width, height) = dimensions((cos, sin));
                let d_y = (self.width as f64 * cos).abs().round() as u32;
                Rect {
                    x: self.x,
                    y: if d_y < self.y { self.y - d_y } else { 0 },
                    width,
                    height,
                }
            }
            _ => unreachable!(),
        }
    }
}

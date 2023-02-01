use image::GrayImage;

pub fn get_distance((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> u32 {
    let sum_of_powers = (u32::pow(x2 - x1, 2) + u32::pow(y2 - y1, 2)) as f64;
    sum_of_powers.sqrt().round() as u32
}

pub trait Normalization<T> {
    fn normalize_degrees(self) -> T;
    fn normalize_and_convert_to_radians(self) -> T;
}

impl Normalization<f64> for f64 {
    fn normalize_degrees(mut self) -> f64 {
        self %= 360.0;
        if self < 0.0 {
            return self + 360.0;
        }
        self
    }

    fn normalize_and_convert_to_radians(self) -> f64 {
        self.normalize_degrees().to_radians()
    }
}

pub fn get_row_black_density(img: &GrayImage) -> Vec<u32> {
    let (columns, rows) = img.dimensions();
    let mut density = vec![0_u32; rows as usize];
    for row in 0..rows {
        let mut black_count = 0_u32;
        for column in 0..columns {
            if img.get_pixel(column, row)[0] == 0 {
                black_count += 1;
            }
        }
        density[row as usize] = black_count
    }
    density
}

pub fn detect_rows(img: &GrayImage, density: u8) -> Vec<(u32, u32)> {
    let row_density = get_row_black_density(img);
    let mut range_start: i32 = -1;
    let mut ranges: Vec<(u32, u32)> = vec![];
    for row in 0..img.height() {
        if row_density[row as usize] > density as u32 && range_start == -1 {
            range_start = row as i32;
        }
        if row_density[row as usize] < density as u32 && range_start != -1 {
            ranges.push((range_start as u32, row as u32));
            range_start = -1
        }
    }
    ranges
}

pub fn detect_lines(img: GrayImage, density: u8) -> Vec<u32> {
    let mut result = vec![];
    let rows = detect_rows(&img, density);
    for row_limits in rows {
        let width = img.width();
        let height = row_limits.1.abs_diff(row_limits.0);
        if width != 0 && height != 0 {
            result.push(0); // left
            result.push(row_limits.0); // top
            result.push(width);
            result.push(height);
        }
    }
    result
}

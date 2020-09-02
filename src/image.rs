use std::convert::TryInto;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::colour::{ColourRgbF, ColourRgbU8};
use crate::math::Vec3;
use crate::util::Array2D;

pub struct ImageRgbU8 {
    data: Array2D<[u8; 3]>,
}

impl ImageRgbU8 {
    pub fn new(width: usize, height: usize) -> ImageRgbU8 {
        ImageRgbU8 {
            data: Array2D::new(height, width),
        }
    }

    pub fn get_colour(&self, row: usize, column: usize) -> ColourRgbU8 {
        ColourRgbU8 {
            values: self.data[row][column].try_into().expect("Wrong length."),
        }
    }

    pub fn set_colour(&mut self, row: usize, column: usize, colour: ColourRgbU8) {
        let slice = &mut self.data[row][column];
        slice.copy_from_slice(&colour.values[..]);
    }

    pub fn get_pixel_data(&self) -> &[u8] {
        let data = self.data.as_slice();
        unsafe { std::slice::from_raw_parts(data[0].as_ptr(), data.len() * 3) }
    }

    pub fn get_width(&self) -> usize {
        self.data.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.data.get_height()
    }

    pub fn num_channels() -> usize {
        3
    }

    pub fn update(&mut self, start_row: usize, start_column: usize, image: &ImageRgbU8) {
        self.data.update_block(start_row, start_column, &image.data);
    }

    pub fn write_png(&self, filename: &Path) -> Result<(), std::io::Error> {
        let file = File::create(filename)?;
        let ref mut file_buffer = BufWriter::new(file);

        let mut encoder = png::Encoder::new(
            file_buffer,
            self.get_width() as u32,
            self.get_height() as u32,
        );
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(self.get_pixel_data())?;
        Ok(())
    }
}

pub struct ImageRgbF {
    pixel_data: Vec<f64>,
    width: usize,
    height: usize,
}

impl ImageRgbF {
    pub fn new(width: usize, height: usize) -> ImageRgbF {
        ImageRgbF {
            width,
            height,
            pixel_data: vec![0.0; width * height * 3 as usize],
        }
    }

    pub fn clear(&mut self) -> &mut ImageRgbF {
        for elem in self.pixel_data.iter_mut() {
            *elem = 0.0;
        }
        self
    }

    pub fn get_colour(&self, row: usize, column: usize) -> ColourRgbF {
        assert!(row < self.height && column < self.width);
        let index = self.calculate_index(row, column);
        ColourRgbF::from_vec3(&Vec3::from_slice(&self.pixel_data[index..index + 3]))
    }

    pub fn set_colour(&mut self, row: usize, column: usize, colour: ColourRgbF) {
        assert!(row < self.height && column < self.width);
        let index = self.calculate_index(row, column);
        self.pixel_data[index..index + 3].copy_from_slice(&colour.as_vec3().as_slice());
    }

    pub fn get_pixel_data(&self) -> &Vec<f64> {
        &self.pixel_data
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn num_channels() -> usize {
        3
    }

    fn calculate_index(&self, row: usize, column: usize) -> usize {
        assert!(row < self.height && column < self.width);
        (((self.height - (row + 1)) * self.width + column) * Self::num_channels()) as usize
    }
}

pub trait NormalizedAsByte {
    fn normalized_to_byte(self) -> u8;
    fn byte_to_normalized(byte: u8) -> Self;
}

impl NormalizedAsByte for f32 {
    fn normalized_to_byte(self) -> u8 {
        (self * (std::u8::MAX as f32)) as u8
    }

    fn byte_to_normalized(byte: u8) -> f32 {
        (byte as f32) / (std::u8::MAX as f32)
    }
}

impl NormalizedAsByte for f64 {
    fn normalized_to_byte(self) -> u8 {
        (self * (std::u8::MAX as f64)) as u8
    }

    fn byte_to_normalized(byte: u8) -> f64 {
        (byte as f64) / (std::u8::MAX as f64)
    }
}

pub trait ToneMapper {
    fn apply_tone_mapping(&self, image_in: &ImageRgbF, image_out: &mut ImageRgbU8);
}

#[derive(Default)]
pub struct ClampingToneMapper {}

impl ClampingToneMapper {
    fn clamp(v: &f64) -> u8 {
        v.clamp(0.0, 1.0).normalized_to_byte()
    }
}

impl ToneMapper for ClampingToneMapper {
    fn apply_tone_mapping(&self, image_in: &ImageRgbF, image_out: &mut ImageRgbU8) {
        assert!(image_in.get_width() == image_out.get_width());
        assert!(image_in.get_height() == image_out.get_height());
        for column in 0..image_in.get_width() {
            for row in 0..image_in.get_height() {
                let colour = image_in.get_colour(row, column);
                image_out.set_colour(
                    row,
                    column,
                    ColourRgbU8 {
                        values: [
                            Self::clamp(&colour.red()),
                            Self::clamp(&colour.green()),
                            Self::clamp(&colour.blue()),
                        ],
                    },
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_pixel_data_returns_correct_values() {
        let mut target = ImageRgbU8::new(4, 3);
        for i in 0..3 {
            for j in 0..4 {
                target.set_colour(
                    i,
                    j,
                    ColourRgbU8 {
                        values: [i as u8, j as u8, i as u8],
                    },
                )
            }
        }
        for i in 0..3 {
            for j in 0..4 {
                let index = (i * 4 + j) * 3;
                assert!(target.get_pixel_data()[index] == i as u8);
                assert!(target.get_pixel_data()[index + 1] == j as u8);
                assert!(target.get_pixel_data()[index + 2] == i as u8);
            }
        }
    }

    mod normalized_as_byte {
        use super::*;

        #[test]
        fn normalized_to_byte_converts_1_to_255_for_f32() {
            assert!((1.0f32).normalized_to_byte() == 0xff);
        }

        #[test]
        fn byte_to_normalized_converts_255_to_1_for_f32() {
            assert!(f32::byte_to_normalized(0xff) == 1.0);
        }

        #[test]
        fn normalized_to_byte_converts_1_to_255_for_f64() {
            assert!((1.0f64).normalized_to_byte() == 255);
        }

        #[test]
        fn byte_to_normalized_converts_255_to_1_for_f64() {
            assert!(f64::byte_to_normalized(0xff) == 1.0);
        }

        #[test]
        fn normalized_to_byte_converts_0_to_0_for_f32() {
            assert!((0.0f32).normalized_to_byte() == 0);
        }

        #[test]
        fn byte_to_normalized_converts_0_to_0_for_f32() {
            assert!(f32::byte_to_normalized(0) == 0.0);
        }

        #[test]
        fn normalized_to_byte_converts_0_to_0_for_f64() {
            assert!((0.0f64).normalized_to_byte() == 0);
        }

        #[test]
        fn byte_to_normalized_converts_0_to_0_for_f64() {
            assert!(f64::byte_to_normalized(0) == 0.0);
        }

        #[test]
        fn normalized_to_byte_converts_half_to_127_for_f32() {
            assert!((0.5f32).normalized_to_byte() == 0x7f);
        }

        #[test]
        fn byte_to_normalized_converts_127_to_half_for_f32() {
            assert!((f32::byte_to_normalized(0x7f) - 0.5).abs() < 1.0 / 256.0);
        }

        #[test]
        fn normalized_to_byte_converts_half_to_127_for_f64() {
            assert!((0.5f64).normalized_to_byte() == 0x7f);
        }

        #[test]
        fn byte_to_normalized_converts_127_to_half_for_f64() {
            assert!((f64::byte_to_normalized(0x7f) - 0.5).abs() < 1.0 / 256.0);
        }
    }

    mod clamping_tone_mapper {
        use super::*;

        #[test]
        fn black_colourrgb_becomes_black_colourrgb24() {
            let target = ClampingToneMapper {};
            let mut image_in = ImageRgbF::new(1, 1);
            let mut image_out = ImageRgbU8::new(1, 1);
            image_in.set_colour(0, 0, ColourRgbF::new(0.0, 0.0, 0.0));
            target.apply_tone_mapping(&image_in, &mut image_out);
            assert!(image_out.get_colour(0, 0).values == [0, 0, 0]);
        }

        #[test]
        fn white_colourrgb_becomes_white_colourrgb24() {
            let target = ClampingToneMapper {};
            let mut image_in = ImageRgbF::new(1, 1);
            let mut image_out = ImageRgbU8::new(1, 1);
            image_in.set_colour(0, 0, ColourRgbF::new(1.0, 1.0, 1.0));
            target.apply_tone_mapping(&image_in, &mut image_out);
            assert!(image_out.get_colour(0, 0).values == [0xff, 0xff, 0xff]);
        }

        #[test]
        fn supersaturated_white_colourrgb_becomes_white_colourrgb24() {
            let target = ClampingToneMapper {};
            let mut image_in = ImageRgbF::new(1, 1);
            let mut image_out = ImageRgbU8::new(1, 1);
            image_in.set_colour(0, 0, ColourRgbF::new(2.0, 2.0, 2.0));
            target.apply_tone_mapping(&image_in, &mut image_out);
            assert!(image_out.get_colour(0, 0).values == [0xff, 0xff, 0xff]);
        }

        #[test]
        fn supersaturated_green_colourrgb_becomes_green_colourrgb24() {
            let target = ClampingToneMapper {};
            let mut image_in = ImageRgbF::new(1, 1);
            let mut image_out = ImageRgbU8::new(1, 1);
            image_in.set_colour(0, 0, ColourRgbF::new(0.0, 2.0, 0.0));
            target.apply_tone_mapping(&image_in, &mut image_out);
            assert!(image_out.get_colour(0, 0).values == [0x0, 0xff, 0x0]);
        }

        #[test]
        fn dark_red_colourrgb_becomes_dark_red_colourrgb24() {
            let target = ClampingToneMapper {};
            let mut image_in = ImageRgbF::new(1, 1);
            let mut image_out = ImageRgbU8::new(1, 1);
            image_in.set_colour(0, 0, ColourRgbF::new(0.5, 0.0, 0.0));
            target.apply_tone_mapping(&image_in, &mut image_out);
            assert!(image_out.get_colour(0, 0).values == [0x7f, 0x0, 0x0]);
        }
    }
}

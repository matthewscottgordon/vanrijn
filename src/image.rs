use std::convert::TryInto;

use nalgebra::{clamp, convert, RealField, Vector3};

use super::colour::{ColourRgbF, ColourRgbU8};

pub struct ImageRgbU8 {
    pixel_data: Vec<u8>,
    width: u32,
    height: u32,
}

impl ImageRgbU8 {
    pub fn new(width: u32, height: u32) -> ImageRgbU8 {
        ImageRgbU8 {
            width,
            height,
            pixel_data: vec![0; (width * height * 3) as usize],
        }
    }

    pub fn clear(&mut self) -> &mut ImageRgbU8 {
        for byte in self.pixel_data.iter_mut() {
            *byte = 0u8;
        }
        self
    }

    pub fn get_colour(&self, row: u32, column: u32) -> ColourRgbU8 {
        assert!(row < self.height && column < self.width);
        let index = self.calculate_index(row, column);
        ColourRgbU8 {
            values: self.pixel_data[index..index + 3]
                .try_into()
                .expect("Wrong length."),
        }
    }

    pub fn set_colour(&mut self, row: u32, column: u32, colour: ColourRgbU8) {
        assert!(row < self.height && column < self.width);
        let index = self.calculate_index(row, column);
        self.pixel_data[index..index + 3].copy_from_slice(&colour.values[..]);
    }

    pub fn get_pixel_data(&self) -> &Vec<u8> {
        &self.pixel_data
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn num_channels() -> u32 {
        3
    }

    fn calculate_index(&self, row: u32, column: u32) -> usize {
        assert!(row < self.height && column < self.width);
        (((self.height - (row + 1)) * self.width + column) * Self::num_channels()) as usize
    }
}

pub struct ImageRgbF<T: RealField> {
    pixel_data: Vec<T>,
    width: u32,
    height: u32,
}

impl<T: RealField> ImageRgbF<T> {
    pub fn new(width: u32, height: u32) -> ImageRgbF<T> {
        ImageRgbF {
            width,
            height,
            pixel_data: vec![convert(0.0); (width * height * 3) as usize],
        }
    }

    pub fn clear(&mut self) -> &mut ImageRgbF<T> {
        for elem in self.pixel_data.iter_mut() {
            *elem = T::zero();
        }
        self
    }

    pub fn get_colour(&self, row: u32, column: u32) -> ColourRgbF<T> {
        assert!(row < self.height && column < self.width);
        let index = self.calculate_index(row, column);
        ColourRgbF::from_vector3(&Vector3::from_row_slice(&self.pixel_data[index..index + 3]))
    }

    pub fn set_colour(&mut self, row: u32, column: u32, colour: ColourRgbF<T>) {
        assert!(row < self.height && column < self.width);
        let index = self.calculate_index(row, column);
        self.pixel_data[index..index + 3].copy_from_slice(&colour.as_vector3().as_slice());
    }

    pub fn get_pixel_data(&self) -> &Vec<T> {
        &self.pixel_data
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn num_channels() -> u32 {
        3
    }

    fn calculate_index(&self, row: u32, column: u32) -> usize {
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

pub trait ToneMapper<T: RealField> {
    fn apply_tone_mapping(&self, image_in: &ImageRgbF<T>, image_out: &mut ImageRgbU8);
}

#[derive(Default)]
pub struct ClampingToneMapper {}

impl ClampingToneMapper {
    fn clamp<T: RealField + NormalizedAsByte>(v: &T) -> u8 {
        clamp(v, &T::zero(), &T::one()).normalized_to_byte()
    }
}

impl<T: RealField + NormalizedAsByte> ToneMapper<T> for ClampingToneMapper {
    fn apply_tone_mapping(&self, image_in: &ImageRgbF<T>, image_out: &mut ImageRgbU8) {
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

use nalgebra::{clamp, convert, RealField, Vector3};

pub struct ColourRGB<T: RealField> {
    values: Vector3<T>,
}

pub trait NormalizedAsByte {
    fn normalized_to_byte(self) -> u8;
    fn byte_to_normalized(byte: u8) -> Self;
}

impl<T: RealField + NormalizedAsByte> ColourRGB<T> {
    pub fn new(red: T, green: T, blue: T) -> ColourRGB<T> {
        ColourRGB {
            values: Vector3::new(red, green, blue),
        }
    }

    pub fn red(&self) -> T {
        self.values[0]
    }

    pub fn green(&self) -> T {
        self.values[1]
    }

    pub fn blue(&self) -> T {
        self.values[2]
    }

    pub fn as_vector3(&self) -> &Vector3<T> {
        &self.values
    }
}

pub struct ColourRGB24 {
    pub values: [u8; 3],
}

pub trait ToneMapper<T: RealField> {
    fn apply_tone_mapping(&self, colour_in: &ColourRGB<T>) -> ColourRGB24;
}

pub struct ClampingToneMapper {}

impl ClampingToneMapper {
    pub fn new() -> ClampingToneMapper {
        ClampingToneMapper {}
    }

    fn clamp<T: RealField + NormalizedAsByte>(v: &T) -> u8 {
        clamp(v, &T::zero(), &T::one()).normalized_to_byte()
    }
}

impl<T: RealField + NormalizedAsByte> ToneMapper<T> for ClampingToneMapper {
    fn apply_tone_mapping(&self, colour_in: &ColourRGB<T>) -> ColourRGB24 {
        ColourRGB24 {
            values: [
                Self::clamp(&colour_in.values[0]),
                Self::clamp(&colour_in.values[1]),
                Self::clamp(&colour_in.values[2]),
            ],
        }
    }
}

pub enum NamedColour {
    Black,
    White,
    Red,
    Lime,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Gray,
    Maroon,
    Olive,
    Green,
    Purple,
    Teal,
    Navy,
}

pub fn named_colour<T: RealField+NormalizedAsByte>(colour: NamedColour) -> ColourRGB<T> {
    let zero: T = convert(0.0);
    let half: T = convert(0.5);
    let one: T = convert(1.0);
    match colour {
        NamedColour::Black => ColourRGB::new(zero, zero, zero),
        NamedColour::White => ColourRGB::new(one, one, one),
        NamedColour::Red => ColourRGB::new(one, zero, zero),
        NamedColour::Lime => ColourRGB::new(zero, one, zero),
        NamedColour::Blue => ColourRGB::new(zero, zero, one),
        NamedColour::Yellow => ColourRGB::new(one, one, zero),
        NamedColour::Cyan => ColourRGB::new(zero, one, one),
        NamedColour::Magenta => ColourRGB::new(one, zero, one),
        NamedColour::Gray => ColourRGB::new(half, half, half),
        NamedColour::Maroon => ColourRGB::new(half, zero, zero),
        NamedColour::Olive => ColourRGB::new(half, half, zero),
        NamedColour::Green => ColourRGB::new(half, half, half),
        NamedColour::Purple => ColourRGB::new(half, zero, half),
        NamedColour::Teal => ColourRGB::new(zero, half, half),
        NamedColour::Navy => ColourRGB::new(zero, zero, half),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    mod colour_rgb {
        use super::*;

        #[test]
        fn constructor_sets_correct_red_green_and_blue() {
            let target = ColourRGB::new(1.0, 2.0, 3.0);
            assert!(target.red() == 1.0);
            assert!(target.green() == 2.0);
            assert!(target.blue() == 3.0);
        }

        #[test]
        fn as_vector3_returns_expected_vector() {
            let target = ColourRGB::new(1.0, 2.0, 3.0);
            let result = target.as_vector3();
            assert!(result.x == 1.0);
        }
    }
    mod clamping_tone_mapper {
        use super::*;

        #[test]
        fn black_colourrgb_becomes_black_colourrgb24() {
            let target = ClampingToneMapper {};
            let colourf = ColourRGB::new(0.0, 0.0, 0.0);
            let colouru = target.apply_tone_mapping(&colourf);
            assert!(colouru.values == [0, 0, 0]);
        }

        #[test]
        fn white_colourrgb_becomes_white_colourrgb24() {
            let target = ClampingToneMapper {};
            let colourf = ColourRGB::new(1.0, 1.0, 1.0);
            let colouru = target.apply_tone_mapping(&colourf);
            assert!(colouru.values == [0xff, 0xff, 0xff]);
        }

        #[test]
        fn supersaturated_white_colourrgb_becomes_white_colourrgb24() {
            let target = ClampingToneMapper {};
            let colourf = ColourRGB::new(2.0, 2.0, 2.0);
            let colouru = target.apply_tone_mapping(&colourf);
            assert!(colouru.values == [0xff, 0xff, 0xff]);
        }

        #[test]
        fn supersaturated_green_colourrgb_becomes_green_colourrgb24() {
            let target = ClampingToneMapper {};
            let colourf = ColourRGB::new(0.0, 2.0, 0.0);
            let colouru = target.apply_tone_mapping(&colourf);
            assert!(colouru.values == [0x0, 0xff, 0x0]);
        }

        #[test]
        fn dark_red_colourrgb_becomes_dark_red_colourrgb24() {
            let target = ClampingToneMapper {};
            let colourf = ColourRGB::new(0.5, 0.0, 0.0);
            let colouru = target.apply_tone_mapping(&colourf);
            assert!(colouru.values == [0x7f, 0x0, 0x0]);
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
}

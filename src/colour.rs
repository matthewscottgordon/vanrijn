use nalgebra::{convert, RealField, Vector3};

#[derive(Debug)]
pub struct ColourRgbF<T: RealField> {
    values: Vector3<T>,
}

impl<T: RealField> ColourRgbF<T> {
    pub fn new(red: T, green: T, blue: T) -> ColourRgbF<T> {
        ColourRgbF {
            values: Vector3::new(red, green, blue),
        }
    }

    pub fn from_named(name: NamedColour) -> ColourRgbF<T> {
        let zero: T = convert(0.0);
        let half: T = convert(0.5);
        let one: T = convert(1.0);
        match name {
            NamedColour::Black => ColourRgbF::new(zero, zero, zero),
            NamedColour::White => ColourRgbF::new(one, one, one),
            NamedColour::Red => ColourRgbF::new(one, zero, zero),
            NamedColour::Lime => ColourRgbF::new(zero, one, zero),
            NamedColour::Blue => ColourRgbF::new(zero, zero, one),
            NamedColour::Yellow => ColourRgbF::new(one, one, zero),
            NamedColour::Cyan => ColourRgbF::new(zero, one, one),
            NamedColour::Magenta => ColourRgbF::new(one, zero, one),
            NamedColour::Gray => ColourRgbF::new(half, half, half),
            NamedColour::Maroon => ColourRgbF::new(half, zero, zero),
            NamedColour::Olive => ColourRgbF::new(half, half, zero),
            NamedColour::Green => ColourRgbF::new(zero, half, zero),
            NamedColour::Purple => ColourRgbF::new(half, zero, half),
            NamedColour::Teal => ColourRgbF::new(zero, half, half),
            NamedColour::Navy => ColourRgbF::new(zero, zero, half),
        }
    }

    pub fn from_vector3(v: &Vector3<T>) -> ColourRgbF<T> {
        ColourRgbF { values: *v }
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

pub struct ColourRgbU8 {
    pub values: [u8; 3],
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

#[cfg(test)]
mod tests {
    use super::*;

    mod colour_rgb {
        use super::*;

        #[test]
        fn constructor_sets_correct_red_green_and_blue() {
            let target = ColourRgbF::new(1.0, 2.0, 3.0);
            assert!(target.red() == 1.0);
            assert!(target.green() == 2.0);
            assert!(target.blue() == 3.0);
        }

        #[test]
        fn as_vector3_returns_expected_vector() {
            let target = ColourRgbF::new(1.0, 2.0, 3.0);
            let result = target.as_vector3();
            assert!(result.x == 1.0);
        }
    }
}

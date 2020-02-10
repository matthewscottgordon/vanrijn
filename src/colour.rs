use nalgebra::{convert, Vector3};

use crate::Real;

use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug)]
pub struct ColourRgbF<T: Real> {
    values: Vector3<T>,
}

impl<T: Real> ColourRgbF<T> {
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

impl<T: Real> Add<ColourRgbF<T>> for ColourRgbF<T> {
    type Output = ColourRgbF<T>;
    fn add(self, rhs: ColourRgbF<T>) -> ColourRgbF<T> {
        ColourRgbF {
            values: self.values + rhs.values,
        }
    }
}

impl<T: Real> Mul<T> for ColourRgbF<T> {
    type Output = ColourRgbF<T>;
    fn mul(self, rhs: T) -> ColourRgbF<T> {
        ColourRgbF {
            values: self.values * rhs,
        }
    }
}

impl<T: Real> Mul<ColourRgbF<T>> for ColourRgbF<T> {
    type Output = ColourRgbF<T>;
    fn mul(self, rhs: ColourRgbF<T>) -> ColourRgbF<T> {
        ColourRgbF {
            values: self.values.component_mul(&rhs.values),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod colour_rgb {
        use super::*;
        use quickcheck::{Arbitrary, Gen};
        use quickcheck_macros::quickcheck;
        impl<T: Arbitrary + Real> Arbitrary for ColourRgbF<T> {
            fn arbitrary<G: Gen>(g: &mut G) -> ColourRgbF<T> {
                let values = <Vector3<T> as Arbitrary>::arbitrary(g);
                ColourRgbF { values }
            }
        }

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

        #[quickcheck]
        fn any_colour_multiplied_by_zero_is_black(colour: ColourRgbF<f64>) {
            let target = colour * 0.0;
            assert!(target.red() == 0.0);
            assert!(target.green() == 0.0);
            assert!(target.blue() == 0.0);
        }

        #[quickcheck]
        fn red_channel_multiplied_by_scalar_yields_correct_result(
            colour: ColourRgbF<f64>,
            scalar: f64,
        ) {
            let target = colour * scalar;
            assert!(target.red() == colour.red() * scalar);
        }

        #[quickcheck]
        fn green_channel_multiplied_by_scalar_yields_correct_result(
            colour: ColourRgbF<f64>,
            scalar: f64,
        ) {
            let target = colour * scalar;
            assert!(target.green() == colour.green() * scalar);
        }

        #[quickcheck]
        fn blue_channel_multiplied_by_scalar_yields_correct_result(
            colour: ColourRgbF<f64>,
            scalar: f64,
        ) {
            let target = colour * scalar;
            assert!(target.blue() == colour.blue() * scalar);
        }

        #[quickcheck]
        fn adding_colourrgbf_adds_individual_channels(
            colour1: ColourRgbF<f64>,
            colour2: ColourRgbF<f64>,
        ) {
            let target = colour1 + colour2;
            assert!(target.red() == colour1.red() + colour2.red());
            assert!(target.green() == colour1.green() + colour2.green());
            assert!(target.blue() == colour1.blue() + colour2.blue());
        }

        #[quickcheck]
        fn multiplying_colourrgbf_adds_individual_channels(
            colour1: ColourRgbF<f64>,
            colour2: ColourRgbF<f64>,
        ) {
            let target = colour1 * colour2;
            assert!(target.red() == colour1.red() * colour2.red());
            assert!(target.green() == colour1.green() * colour2.green());
            assert!(target.blue() == colour1.blue() * colour2.blue());
        }
    }
}

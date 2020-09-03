use crate::math::Vec3;

use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug, Default)]
pub struct ColourRgbF {
    pub values: Vec3,
}

impl ColourRgbF {
    pub fn new(red: f64, green: f64, blue: f64) -> ColourRgbF {
        ColourRgbF {
            values: Vec3::new(red, green, blue),
        }
    }

    pub fn from_named(name: NamedColour) -> ColourRgbF {
        match name {
            NamedColour::Black => ColourRgbF::new(0.0, 0.0, 0.0),
            NamedColour::White => ColourRgbF::new(1.0, 1.0, 1.0),
            NamedColour::Red => ColourRgbF::new(1.0, 0.0, 0.0),
            NamedColour::Lime => ColourRgbF::new(0.0, 1.0, 0.0),
            NamedColour::Blue => ColourRgbF::new(0.0, 0.0, 1.0),
            NamedColour::Yellow => ColourRgbF::new(1.0, 1.0, 0.0),
            NamedColour::Cyan => ColourRgbF::new(0.0, 1.0, 1.0),
            NamedColour::Magenta => ColourRgbF::new(1.0, 0.0, 1.0),
            NamedColour::Gray => ColourRgbF::new(0.5, 0.5, 0.5),
            NamedColour::Maroon => ColourRgbF::new(0.5, 0.0, 0.0),
            NamedColour::Olive => ColourRgbF::new(0.5, 0.5, 0.0),
            NamedColour::Green => ColourRgbF::new(0.0, 0.5, 0.0),
            NamedColour::Purple => ColourRgbF::new(0.5, 0.0, 0.5),
            NamedColour::Teal => ColourRgbF::new(0.0, 0.5, 0.5),
            NamedColour::Navy => ColourRgbF::new(0.0, 0.0, 0.5),
        }
    }

    pub fn from_vec3(v: &Vec3) -> ColourRgbF {
        ColourRgbF { values: *v }
    }

    pub fn red(&self) -> f64 {
        self.values.x()
    }

    pub fn green(&self) -> f64 {
        self.values.y()
    }

    pub fn blue(&self) -> f64 {
        self.values.z()
    }

    pub fn as_vec3(&self) -> &Vec3 {
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

impl Add<ColourRgbF> for ColourRgbF {
    type Output = ColourRgbF;
    fn add(self, rhs: ColourRgbF) -> ColourRgbF {
        ColourRgbF {
            values: self.values + rhs.values,
        }
    }
}

impl Mul<f64> for ColourRgbF {
    type Output = ColourRgbF;
    fn mul(self, rhs: f64) -> ColourRgbF {
        ColourRgbF {
            values: self.values * rhs,
        }
    }
}

impl Mul<ColourRgbF> for ColourRgbF {
    type Output = ColourRgbF;
    fn mul(self, rhs: ColourRgbF) -> ColourRgbF {
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
        impl Arbitrary for ColourRgbF {
            fn arbitrary<G: Gen>(g: &mut G) -> ColourRgbF {
                let values = <Vec3 as Arbitrary>::arbitrary(g);
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
            let result = target.as_vec3();
            assert!(result.x() == 1.0);
        }

        #[quickcheck]
        fn any_colour_multiplied_by_zero_is_black(colour: ColourRgbF) {
            let target = colour * 0.0;
            assert!(target.red() == 0.0);
            assert!(target.green() == 0.0);
            assert!(target.blue() == 0.0);
        }

        #[quickcheck]
        fn red_channel_multiplied_by_scalar_yields_correct_result(colour: ColourRgbF, scalar: f64) {
            let target = colour * scalar;
            assert!(target.red() == colour.red() * scalar);
        }

        #[quickcheck]
        fn green_channel_multiplied_by_scalar_yields_correct_result(
            colour: ColourRgbF,
            scalar: f64,
        ) {
            let target = colour * scalar;
            assert!(target.green() == colour.green() * scalar);
        }

        #[quickcheck]
        fn blue_channel_multiplied_by_scalar_yields_correct_result(
            colour: ColourRgbF,
            scalar: f64,
        ) {
            let target = colour * scalar;
            assert!(target.blue() == colour.blue() * scalar);
        }

        #[quickcheck]
        fn adding_colourrgbf_adds_individual_channels(colour1: ColourRgbF, colour2: ColourRgbF) {
            let target = colour1 + colour2;
            assert!(target.red() == colour1.red() + colour2.red());
            assert!(target.green() == colour1.green() + colour2.green());
            assert!(target.blue() == colour1.blue() + colour2.blue());
        }

        #[quickcheck]
        fn multiplying_colourrgbf_adds_individual_channels(
            colour1: ColourRgbF,
            colour2: ColourRgbF,
        ) {
            let target = colour1 * colour2;
            assert!(target.red() == colour1.red() * colour2.red());
            assert!(target.green() == colour1.green() * colour2.green());
            assert!(target.blue() == colour1.blue() * colour2.blue());
        }
    }
}

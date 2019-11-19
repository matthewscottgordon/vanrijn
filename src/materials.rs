use nalgebra::RealField;

use super::colour::{ColourRGB, NormalizedAsByte};

#[derive(Debug)]
pub struct Material<T: RealField> {
    pub colour: ColourRGB<T>,
    pub smoothness: T,
}

impl<T: RealField+NormalizedAsByte> Material<T> {
    pub fn new_dummy() -> Material<T> {
        Material {
            colour: ColourRGB::new(T::one(), T::one(), T::one()),
            smoothness: T::zero(),
        }
    }
}

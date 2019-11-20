use nalgebra::RealField;

use super::colour::ColourRgbF;

#[derive(Debug)]
pub struct Material<T: RealField> {
    pub colour: ColourRgbF<T>,
    pub smoothness: T,
}

impl<T: RealField> Material<T> {
    pub fn new_dummy() -> Material<T> {
        Material {
            colour: ColourRgbF::new(T::one(), T::one(), T::one()),
            smoothness: T::zero(),
        }
    }
}

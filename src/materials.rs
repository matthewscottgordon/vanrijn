use nalgebra::RealField;

use super::colour::ColourRGB;

pub struct PhongMaterial<T: RealField> {
    pub colour: ColourRGB<T>,
    pub smoothness: T,
}

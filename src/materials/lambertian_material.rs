use nalgebra::Vector3;

use crate::colour::ColourRgbF;
use crate::Real;

use super::{Bsdf, Material};

use std::fmt::Debug;

#[derive(Debug)]
pub struct LambertianMaterial<T: Real> {
    pub colour: ColourRgbF<T>,
    pub diffuse_strength: T,
}

impl<T: Real> LambertianMaterial<T> {
    pub fn new_dummy() -> LambertianMaterial<T> {
        LambertianMaterial {
            colour: ColourRgbF::new(T::one(), T::one(), T::one()),
            diffuse_strength: T::one(),
        }
    }
}

impl<T: Real> Material<T> for LambertianMaterial<T> {
    fn bsdf(&self) -> Bsdf<T> {
        let colour = self.colour * self.diffuse_strength;
        Box::new(
            move |_w_o: Vector3<T>, _w_i: Vector3<T>, colour_in: ColourRgbF<T>| colour * colour_in,
        )
    }
}

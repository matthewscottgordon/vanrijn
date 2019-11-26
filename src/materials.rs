use nalgebra::{RealField, Vector3};

use super::colour::ColourRgbF;

use std::fmt::Debug;

pub trait Material<T: RealField>: Debug {
    fn bsdf<'a>(
        &'a self,
    ) -> Box<dyn Fn(Vector3<T>, Vector3<T>, ColourRgbF<T>) -> ColourRgbF<T> + 'a>;
}

#[derive(Debug)]
pub struct LambertianMaterial<T: RealField> {
    pub colour: ColourRgbF<T>,
    pub diffuse_strength: T,
}

impl<T: RealField> LambertianMaterial<T> {
    pub fn new_dummy() -> LambertianMaterial<T> {
        LambertianMaterial {
            colour: ColourRgbF::new(T::one(), T::one(), T::one()),
            diffuse_strength: T::one(),
        }
    }
}

impl<T: RealField> Material<T> for LambertianMaterial<T> {
    fn bsdf<'a>(
        &'a self,
    ) -> Box<dyn Fn(Vector3<T>, Vector3<T>, ColourRgbF<T>) -> ColourRgbF<T> + 'a> {
        Box::new(
            move |_w_o: Vector3<T>, _w_i: Vector3<T>, colour_in: ColourRgbF<T>| {
                self.colour * colour_in * self.diffuse_strength
            },
        )
    }
}

use nalgebra::{RealField, Vector3};

use super::colour::{ColourRgbF, NamedColour};

use std::fmt::Debug;

pub trait Material<T: RealField>: Debug {
    fn bsdf<'a>(
        &'a self,
    ) -> Box<dyn Fn(Vector3<T>, Vector3<T>, ColourRgbF<T>) -> ColourRgbF<T> + 'a>;

    fn sample(&self, _w_o: &Vector3<T>) -> Vec<Vector3<T>> {
        vec![]
    }
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

#[derive(Debug)]
pub struct PhongMaterial<T: RealField> {
    pub colour: ColourRgbF<T>,
    pub diffuse_strength: T,
    pub specular_strength: T,
    pub smoothness: T,
}

impl<T: RealField> Material<T> for PhongMaterial<T> {
    fn bsdf<'a>(
        &'a self,
    ) -> Box<dyn Fn(Vector3<T>, Vector3<T>, ColourRgbF<T>) -> ColourRgbF<T> + 'a> {
        Box::new(
            move |w_o: Vector3<T>, w_i: Vector3<T>, colour_in: ColourRgbF<T>| {
                if w_i.z < T::zero() || w_o.z < T::zero() {
                    ColourRgbF::from_vector3(&Vector3::zeros())
                } else {
                    let reflection_vector = Vector3::new(-w_i.x, -w_i.y, w_i.z);
                    self.colour * colour_in * self.diffuse_strength
                        + ColourRgbF::from_named(NamedColour::White)
                            * w_o.dot(&reflection_vector).abs().powf(self.smoothness)
                            * (self.specular_strength / w_i.dot(&Vector3::z_axis()))
                }
            },
        )
    }
}

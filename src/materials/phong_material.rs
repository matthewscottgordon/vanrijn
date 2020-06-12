use nalgebra::Vector3;

use crate::colour::{ColourRgbF, NamedColour};
use crate::Real;

use std::fmt::Debug;

use super::{Bsdf, Material};

#[derive(Debug)]
pub struct PhongMaterial<T: Real> {
    pub colour: ColourRgbF<T>,
    pub diffuse_strength: T,
    pub specular_strength: T,
    pub smoothness: T,
}

impl<T: Real> Material<T> for PhongMaterial<T> {
    fn bsdf(&self) -> Bsdf<T> {
        let smoothness = self.smoothness;
        let specular_strength = self.specular_strength;
        let colour = self.colour * self.diffuse_strength;
        Box::new(
            move |w_o: Vector3<T>, w_i: Vector3<T>, colour_in: ColourRgbF<T>| {
                if w_i.z < T::zero() || w_o.z < T::zero() {
                    ColourRgbF::from_vector3(&Vector3::zeros())
                } else {
                    let reflection_vector = Vector3::new(-w_i.x, -w_i.y, w_i.z);
                    colour * colour_in
                        + ColourRgbF::from_named(NamedColour::White)
                            * w_o.dot(&reflection_vector).abs().powf(smoothness)
                            * (specular_strength / w_i.dot(&Vector3::z_axis()))
                }
            },
        )
    }
}

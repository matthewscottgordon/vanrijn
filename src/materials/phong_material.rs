use crate::colour::{ColourRgbF, NamedColour};
use crate::math::Vec3;

use std::fmt::Debug;

use super::{Bsdf, Material};

#[derive(Debug)]
pub struct PhongMaterial {
    pub colour: ColourRgbF,
    pub diffuse_strength: f64,
    pub specular_strength: f64,
    pub smoothness: f64,
}

impl Material for PhongMaterial {
    fn bsdf(&self) -> Bsdf {
        let smoothness = self.smoothness;
        let specular_strength = self.specular_strength;
        let colour = self.colour * self.diffuse_strength;
        Box::new(move |w_o: Vec3, w_i: Vec3, colour_in: ColourRgbF| {
            if w_i.z() < 0.0 || w_o.z() < 0.0 {
                ColourRgbF::from_vec3(&Vec3::zeros())
            } else {
                let reflection_vector = Vec3::new(-w_i.x(), -w_i.y(), w_i.z());
                colour * colour_in
                    + ColourRgbF::from_named(NamedColour::White)
                        * w_o.dot(&reflection_vector).abs().powf(smoothness)
                        * (specular_strength / w_i.dot(&Vec3::unit_z()))
            }
        })
    }
}

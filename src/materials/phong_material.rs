use nalgebra::Vector3;

use crate::colour::{ColourRgbF, NamedColour};

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
        Box::new(
            move |w_o: Vector3<f64>, w_i: Vector3<f64>, colour_in: ColourRgbF| {
                if w_i.z < 0.0 || w_o.z < 0.0 {
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

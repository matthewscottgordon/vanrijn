use crate::colour::ColourRgbF;
use crate::math::Vec3;

use std::fmt::Debug;

use super::{Bsdf, Material};

#[derive(Debug)]
pub struct ReflectiveMaterial {
    pub colour: ColourRgbF,
    pub diffuse_strength: f64,
    pub reflection_strength: f64,
}

impl Material for ReflectiveMaterial {
    fn bsdf(&self) -> Bsdf {
        let diffuse_colour_factor = self.colour * self.diffuse_strength;
        let reflection_strength = self.reflection_strength;
        Box::new(move |w_o: Vec3, w_i: Vec3, colour_in: ColourRgbF| {
            if w_i.z() <= 0.0 || w_o.z() <= 0.0 {
                ColourRgbF::new(0.0, 0.0, 0.0)
            } else {
                let reflection_vector = Vec3::new(-w_o.x(), -w_o.y(), w_o.z());
                let reflection_colour = colour_in * reflection_strength;
                let diffuse_colour = diffuse_colour_factor * colour_in;
                let sigma = 0.05;
                let two = 2.0;
                // These are normalized vectors, but sometimes rounding errors cause the
                // dot product to be slightly above 1 or below 0. The call to clamp
                // ensures the values stay within the domain of acos,
                let theta = w_i.dot(&reflection_vector).clamp(0.0, 1.0).abs().acos();
                let reflection_factor = (-(theta.powf(two)) / (two * sigma * sigma)).exp();
                reflection_colour * reflection_factor + diffuse_colour * (1.0 - reflection_factor)
            }
        })
    }

    fn sample(&self, w_o: &Vec3) -> Vec<Vec3> {
        vec![Vec3::new(-w_o.x(), -w_o.y(), w_o.z())]
    }
}

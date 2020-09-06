use crate::colour::{Photon, Spectrum};
use crate::math::Vec3;

use std::fmt::Debug;

use super::Material;

#[derive(Debug)]
pub struct ReflectiveMaterial {
    pub colour: Spectrum,
    pub diffuse_strength: f64,
    pub reflection_strength: f64,
}

impl Material for ReflectiveMaterial {
    fn bsdf<'a>(&'a self) -> Box<dyn Fn(&Vec3, &Vec3, &Photon) -> Photon + 'a> {
        Box::new(move |w_o: &Vec3, w_i: &Vec3, photon_in: &Photon| {
            if w_i.z() <= 0.0 || w_o.z() <= 0.0 {
                Photon {
                    wavelength: photon_in.wavelength,
                    intensity: 0.0,
                }
            } else {
                let reflection_vector = Vec3::new(-w_o.x(), -w_o.y(), w_o.z());
                let mut photon_out = self.colour.scale_photon(photon_in);
                photon_out.intensity *= self.diffuse_strength;
                let sigma = 0.05;
                let two = 2.0;
                // These are normalized vectors, but sometimes rounding errors cause the
                // dot product to be slightly above 1 or below 0. The call to clamp
                // ensures the values stay within the domain of acos,
                let theta = w_i.dot(&reflection_vector).clamp(0.0, 1.0).abs().acos();
                let reflection_factor =
                    self.reflection_strength * (-(theta.powf(two)) / (two * sigma * sigma)).exp();
                photon_out.intensity =
                    photon_out.intensity * (1.0 - reflection_factor) + reflection_factor;
                photon_out
            }
        })
    }

    fn sample(&self, w_o: &Vec3) -> Vec<Vec3> {
        vec![Vec3::new(-w_o.x(), -w_o.y(), w_o.z())]
    }
}

use crate::colour::{Photon, Spectrum};
use crate::math::Vec3;

use std::fmt::Debug;

use super::Material;

#[derive(Debug)]
pub struct PhongMaterial {
    pub colour: Spectrum,
    pub diffuse_strength: f64,
    pub specular_strength: f64,
    pub smoothness: f64,
}

impl Material for PhongMaterial {
    fn bsdf<'a>(&'a self) -> Box<dyn Fn(&Vec3, &Vec3, &Photon) -> Photon + 'a> {
        Box::new(move |w_o: &Vec3, w_i: &Vec3, photon_in: &Photon| {
            if w_i.z() < 0.0 || w_o.z() < 0.0 {
                Photon {
                    wavelength: photon_in.wavelength,
                    intensity: 0.0,
                }
            } else {
                let reflection_vector = Vec3::new(-w_i.x(), -w_i.y(), w_i.z());
                let intensity = self.colour.scale_photon(photon_in).intensity
                    * self.diffuse_strength
                    + w_o.dot(&reflection_vector).abs().powf(self.smoothness)
                        * (self.specular_strength / w_i.dot(&Vec3::unit_z()));
                Photon {
                    wavelength: photon_in.wavelength,
                    intensity,
                }
            }
        })
    }
}

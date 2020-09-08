use crate::colour::{Photon, Spectrum};
use crate::math::Vec3;

use super::Material;

use std::fmt::Debug;

#[derive(Debug)]
pub struct LambertianMaterial {
    pub colour: Spectrum,
    pub diffuse_strength: f64,
}

impl LambertianMaterial {
    pub fn new_dummy() -> LambertianMaterial {
        LambertianMaterial {
            colour: Spectrum::black(),
            diffuse_strength: 1.0,
        }
    }
}

impl Material for LambertianMaterial {
    fn bsdf<'a>(&'a self) -> Box<dyn Fn(&Vec3, &Vec3, &Photon) -> Photon + 'a> {
        Box::new(move |_w_o: &Vec3, _w_i: &Vec3, photon_in: &Photon| {
            let mut result = self.colour.scale_photon(photon_in);
            result.intensity *= self.diffuse_strength;
            result
        })
    }
}

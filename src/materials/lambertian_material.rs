use crate::colour::{Photon, Spectrum};
use crate::math::Vec3;

use super::{Material, MaterialSampleResult};

use rand::distributions::Open01;
use rand::{thread_rng, Rng};

use std::f64::consts::PI;
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

    fn sample(&self, _w_i: &Vec3, _photon: &Photon) -> MaterialSampleResult {
        let mut rng = thread_rng();
        let mut w_o = Vec3::new(
            2.0 * rng.sample::<f64, _>(Open01) - 1.0,
            2.0 * rng.sample::<f64, _>(Open01) - 1.0,
            0.0,
        );
        while w_o.norm_squared() > 1.0 {
            w_o = Vec3::new(
                2.0 * rng.sample::<f64, _>(Open01) - 1.0,
                2.0 * rng.sample::<f64, _>(Open01) - 1.0,
                0.0,
            );
        }
        w_o.coords[2] = (1.0 - w_o.x() * w_o.x() - w_o.y() * w_o.y())
            .sqrt()
            .max(0.0);
        let cos_theta = w_o.dot(&Vec3::unit_z());
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        MaterialSampleResult {
            direction: w_o.normalize(),
            pdf: (cos_theta * sin_theta) / PI,
        }
    }
}

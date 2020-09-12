use crate::math::Vec3;

use super::colour::Photon;

use rand::distributions::{Open01, OpenClosed01};
use rand::{thread_rng, Rng};
use std::fmt::Debug;

pub mod lambertian_material;
pub use lambertian_material::LambertianMaterial;

pub mod phong_material;
pub use phong_material::PhongMaterial;

pub mod reflective_material;
pub use reflective_material::ReflectiveMaterial;

pub mod smooth_transparent_dialectric;
pub use smooth_transparent_dialectric::SmoothTransparentDialectric;

pub trait Material: Debug + Sync + Send {
    fn bsdf<'a>(&'a self) -> Box<dyn Fn(&Vec3, &Vec3, &Photon) -> Photon + 'a>;

    fn sample(&self, _w_i: &Vec3, _photon: &Photon) -> Vec3 {
        let mut rng = thread_rng();
        let mut w_o = Vec3::new(
            2.0 * rng.sample::<f64, _>(Open01) - 1.0,
            2.0 * rng.sample::<f64, _>(Open01) - 1.0,
            rng.sample::<f64, _>(OpenClosed01),
        );
        while w_o.norm_squared() > 1.0 {
            w_o = Vec3::new(
                2.0 * rng.sample::<f64, _>(Open01) - 1.0,
                2.0 * rng.sample::<f64, _>(Open01) - 1.0,
                rng.sample::<f64, _>(OpenClosed01),
            );
        }
        w_o
    }

    fn pdf(&self, _w_i: &Vec3, _w_o: &Vec3) -> f64 {
        1.0
    }
}

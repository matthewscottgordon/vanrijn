use crate::math::Vec3;

use super::colour::Photon;
use super::random_distributions::{CosineWeightedHemisphere, RandomDistribution};

use std::fmt::Debug;

pub mod lambertian_material;
pub use lambertian_material::LambertianMaterial;

pub mod phong_material;
pub use phong_material::PhongMaterial;

pub mod reflective_material;
pub use reflective_material::ReflectiveMaterial;

pub mod smooth_transparent_dialectric;
pub use smooth_transparent_dialectric::SmoothTransparentDialectric;

pub struct MaterialSampleResult {
    pub direction: Vec3,
    pub pdf: f64,
}

pub trait Material: Debug + Sync + Send {
    fn bsdf<'a>(&'a self) -> Box<dyn Fn(&Vec3, &Vec3, &Photon) -> Photon + 'a>;

    fn sample(&self, _w_i: &Vec3, _photon: &Photon) -> MaterialSampleResult {
        let distribution = CosineWeightedHemisphere::new();
        let direction = distribution.value();
        let pdf = distribution.pdf(direction);
        MaterialSampleResult { direction, pdf }
    }
}

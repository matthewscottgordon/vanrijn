use nalgebra::Vector3;

use super::colour::ColourRgbF;
use crate::Real;

use std::fmt::Debug;

type Bsdf<T> = Box<dyn Fn(Vector3<T>, Vector3<T>, ColourRgbF<T>) -> ColourRgbF<T>>;

pub mod lambertian_material;
pub use lambertian_material::LambertianMaterial;

pub mod phong_material;
pub use phong_material::PhongMaterial;

pub mod reflective_material;
pub use reflective_material::ReflectiveMaterial;

pub mod rgb_sampled_bsdf_material;
pub use rgb_sampled_bsdf_material::RgbSampledBsdfMaterial;

pub trait Material<T: Real>: Debug + Sync + Send {
    fn bsdf(&self) -> Bsdf<T>;

    fn sample(&self, _w_o: &Vector3<T>) -> Vec<Vector3<T>> {
        vec![]
    }
}

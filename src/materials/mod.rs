use nalgebra::Vector3;

use super::colour::ColourRgbF;

use std::fmt::Debug;

type Bsdf = Box<dyn Fn(Vector3<f64>, Vector3<f64>, ColourRgbF) -> ColourRgbF>;

pub mod lambertian_material;
pub use lambertian_material::LambertianMaterial;

pub mod phong_material;
pub use phong_material::PhongMaterial;

pub mod reflective_material;
pub use reflective_material::ReflectiveMaterial;

pub mod rgb_sampled_bsdf_material;
pub use rgb_sampled_bsdf_material::RgbSampledBsdfMaterial;

pub trait Material: Debug + Sync + Send {
    fn bsdf(&self) -> Bsdf;

    fn sample(&self, _w_o: &Vector3<f64>) -> Vec<Vector3<f64>> {
        vec![]
    }
}

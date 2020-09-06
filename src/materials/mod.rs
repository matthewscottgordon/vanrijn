use crate::math::Vec3;

use super::colour::Photon;

use std::fmt::Debug;

pub mod lambertian_material;
pub use lambertian_material::LambertianMaterial;

pub mod phong_material;
pub use phong_material::PhongMaterial;

pub mod reflective_material;
pub use reflective_material::ReflectiveMaterial;

pub trait Material: Debug + Sync + Send {
    fn bsdf<'a>(&'a self) -> Box<dyn Fn(&Vec3, &Vec3, &Photon) -> Photon + 'a>;

    fn sample(&self, _w_o: &Vec3) -> Vec<Vec3> {
        vec![]
    }
}

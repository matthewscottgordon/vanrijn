use nalgebra::{RealField, Vector3};

use super::colour::{ColourRGB, NormalizedAsByte};
use super::raycasting::IntersectionInfo;

pub trait Integrator<T: RealField + NormalizedAsByte> {
    fn integrate(&self, info: &IntersectionInfo<T>) -> ColourRGB<T>;
}

pub struct DirectionalLight<T: RealField + NormalizedAsByte> {
    pub direction: Vector3<T>,
    pub intensity: T,
}

pub struct PhongIntegrator<T: RealField + NormalizedAsByte> {
    pub ambient_light: T,
    pub lights: Vec<DirectionalLight<T>>,
}

impl<T: RealField + NormalizedAsByte> Integrator<T> for PhongIntegrator<T> {
    fn integrate(&self, info: &IntersectionInfo<T>) -> ColourRGB<T> {
        let intensity = self.lights
            .iter()
            .map(|light| light.intensity * light.direction.dot(&info.normal))
            .fold(self.ambient_light, |a, b| a + b);
        ColourRGB::from_vector3(&(info.material.colour.as_vector3() * intensity))
    }
}

use nalgebra::{RealField, Vector3};

use super::raycasting::IntersectionInfo;

pub trait Integrator<T: RealField> {
    fn integrate(&self, info: &IntersectionInfo<T>) -> T;
}

pub struct DirectionalLight<T: RealField> {
    pub direction: Vector3<T>,
    pub intensity: T,
}

pub struct PhongIntegrator<T: RealField> {
    pub ambient_light: T,
    pub lights: Vec<DirectionalLight<T>>,
}

impl<T: RealField> Integrator<T> for PhongIntegrator<T> {
    fn integrate(&self, info: &IntersectionInfo<T>) -> T {
        self.lights
            .iter()
            .map(|light| light.intensity * light.direction.dot(&info.normal))
            .fold(self.ambient_light, |a, b| a + b)
    }
}

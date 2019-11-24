use nalgebra::{RealField, Vector3};

use super::colour::ColourRgbF;
use super::raycasting::IntersectionInfo;

pub trait Integrator<T: RealField> {
    fn integrate(&self, info: &IntersectionInfo<T>) -> ColourRgbF<T>;
}

pub struct DirectionalLight<T: RealField> {
    pub direction: Vector3<T>,
    pub colour: ColourRgbF<T>,
}

pub struct PhongIntegrator<T: RealField> {
    pub ambient_light: ColourRgbF<T>,
    pub lights: Vec<DirectionalLight<T>>,
}

impl<T: RealField> Integrator<T> for PhongIntegrator<T> {
    fn integrate(&self, info: &IntersectionInfo<T>) -> ColourRgbF<T> {
        self.lights
            .iter()
            .map(|light| {
                info.material.bsdf()(info.retro, light.direction, light.colour)
                    * light.direction.dot(&info.normal)
            })
            .fold(self.ambient_light, |a, b| a + b)
    }
}

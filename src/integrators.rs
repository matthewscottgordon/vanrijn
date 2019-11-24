use nalgebra::{RealField, Vector3};

use super::colour::ColourRgbF;
use super::raycasting::{IntersectionInfo, Ray};
use super::sampler::Sampler;

pub trait Integrator<T: RealField> {
    fn integrate(&self, sampler: &Sampler<T>, info: &IntersectionInfo<T>) -> ColourRgbF<T>;
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
    fn integrate(&self, sampler: &Sampler<T>, info: &IntersectionInfo<T>) -> ColourRgbF<T> {
        self.lights
            .iter()
            .map(
                |light| match sampler.sample(&Ray::new(info.location, light.direction)) {
                    Some(_) => self.ambient_light,
                    None => {
                        info.material.bsdf()(info.retro, light.direction, light.colour)
                            * light.direction.dot(&info.normal)
                    }
                },
            )
            .fold(self.ambient_light, |a, b| a + b)
    }
}

use nalgebra::{convert, RealField, Vector3};

use super::algebra_utils::try_change_of_basis_matrix;
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

pub struct WhittedIntegrator<T: RealField> {
    pub ambient_light: ColourRgbF<T>,
    pub lights: Vec<DirectionalLight<T>>,
}

// TODO: Get rid of the magic bias number, which should be calculated base on expected error
// bounds and tangent direction
impl<T: RealField> Integrator<T> for WhittedIntegrator<T> {
    fn integrate(&self, sampler: &Sampler<T>, info: &IntersectionInfo<T>) -> ColourRgbF<T> {
        self.lights
            .iter()
            .map(|light| {
                let basis_change =
                    try_change_of_basis_matrix(&info.tangent, &info.cotangent, &info.normal)
                        .expect("Normal, tangent and cotangent don't for a valid basis.");
                match sampler
                    .sample(&Ray::new(info.location, light.direction).bias(convert(0.000000001)))
                {
                    Some(_) => self.ambient_light,
                    None => {
                        info.material.bsdf()(
                            basis_change * info.retro,
                            basis_change * light.direction,
                            light.colour,
                        ) * light.direction.dot(&info.normal).abs()
                    }
                }
            })
            .fold(self.ambient_light, |a, b| a + b)
    }
}

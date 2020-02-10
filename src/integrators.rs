use nalgebra::{convert, Vector3};

use super::colour::ColourRgbF;
use super::raycasting::{IntersectionInfo, Ray};
use super::sampler::Sampler;
use super::util::algebra_utils::try_change_of_basis_matrix;

use crate::Real;

pub trait Integrator<T: Real> {
    fn integrate(&self, sampler: &Sampler<T>, info: &IntersectionInfo<T>) -> ColourRgbF<T>;
}

pub struct DirectionalLight<T: Real> {
    pub direction: Vector3<T>,
    pub colour: ColourRgbF<T>,
}

pub struct WhittedIntegrator<T: Real> {
    pub ambient_light: ColourRgbF<T>,
    pub lights: Vec<DirectionalLight<T>>,
}

// TODO: Get rid of the magic bias number, which should be calculated base on expected error
// bounds and tangent direction
impl<T: Real> Integrator<T> for WhittedIntegrator<T> {
    fn integrate(&self, sampler: &Sampler<T>, info: &IntersectionInfo<T>) -> ColourRgbF<T> {
        let world_to_bsdf_space =
            try_change_of_basis_matrix(&info.tangent, &info.cotangent, &info.normal)
                .expect("Normal, tangent and cotangent don't for a valid basis.");
        let bsdf_to_world_space = world_to_bsdf_space
            .try_inverse()
            .expect("Expected matrix to be invertable.");
        self.lights
            .iter()
            .map(|light| {
                match sampler
                    .sample(&Ray::new(info.location, light.direction).bias(convert(0.000_000_1)))
                {
                    Some(_) => self.ambient_light,
                    None => {
                        info.material.bsdf()(
                            world_to_bsdf_space * info.retro,
                            world_to_bsdf_space * light.direction,
                            light.colour,
                        ) * light.direction.dot(&info.normal).abs()
                    }
                }
            })
            .chain(
                info.material
                    .sample(&(world_to_bsdf_space * info.retro))
                    .iter()
                    .map(|direction| {
                        let world_space_direction = bsdf_to_world_space * direction;
                        match sampler.sample(
                            &Ray::new(info.location, world_space_direction)
                                .bias(convert(0.000_000_1)),
                        ) {
                            Some(recursive_hit) => {
                                info.material.bsdf()(
                                    world_to_bsdf_space * info.retro,
                                    *direction,
                                    self.integrate(&sampler, &recursive_hit),
                                ) * world_space_direction.dot(&info.normal).abs()
                            }
                            None => ColourRgbF::new(T::zero(), T::zero(), T::zero()),
                        }
                    }),
            )
            .fold(self.ambient_light, |a, b| a + b)
    }
}

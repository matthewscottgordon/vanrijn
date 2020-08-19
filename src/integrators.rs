use nalgebra::{convert, Vector3};

use super::colour::ColourRgbF;
use super::raycasting::{IntersectionInfo, Ray};
use super::sampler::Sampler;
use super::util::algebra_utils::try_change_of_basis_matrix;

pub trait Integrator {
    fn integrate(
        &self,
        sampler: &Sampler,
        info: &IntersectionInfo,
        recursion_limit: u16,
    ) -> ColourRgbF;
}

pub struct DirectionalLight {
    pub direction: Vector3<f64>,
    pub colour: ColourRgbF,
}

pub struct WhittedIntegrator {
    pub ambient_light: ColourRgbF,
    pub lights: Vec<DirectionalLight>,
}

// TODO: Get rid of the magic bias number, which should be calculated base on expected error
// bounds and tangent direction
impl Integrator for WhittedIntegrator {
    fn integrate(
        &self,
        sampler: &Sampler,
        info: &IntersectionInfo,
        recursion_limit: u16,
    ) -> ColourRgbF {
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
                                if recursion_limit > 0 {
                                    info.material.bsdf()(
                                        world_to_bsdf_space * info.retro,
                                        *direction,
                                        self.integrate(
                                            &sampler,
                                            &recursive_hit,
                                            recursion_limit - 1,
                                        ),
                                    ) * world_space_direction.dot(&info.normal).abs()
                                } else {
                                    ColourRgbF::new(0.0, 0.0, 0.0)
                                }
                            }
                            None => ColourRgbF::new(0.0, 0.0, 0.0),
                        }
                    }),
            )
            .fold(self.ambient_light, |a, b| a + b)
    }
}

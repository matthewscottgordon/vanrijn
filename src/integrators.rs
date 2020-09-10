use crate::math::Vec3;

use super::colour::{Photon, Spectrum};
use super::raycasting::{IntersectionInfo, Ray};
use super::sampler::Sampler;
use super::util::algebra_utils::try_change_of_basis_matrix;

pub trait Integrator {
    fn integrate(
        &self,
        sampler: &Sampler,
        info: &IntersectionInfo,
        photon: &Photon,
        recursion_limit: u16,
    ) -> Photon;
}

pub struct DirectionalLight {
    pub direction: Vec3,
    pub spectrum: Spectrum,
}

pub struct WhittedIntegrator {
    pub ambient_light: Spectrum,
    pub lights: Vec<DirectionalLight>,
}

impl Integrator for WhittedIntegrator {
    fn integrate(
        &self,
        sampler: &Sampler,
        info: &IntersectionInfo,
        photon: &Photon,
        recursion_limit: u16,
    ) -> Photon {
        let world_to_bsdf_space =
            try_change_of_basis_matrix(&info.tangent, &info.cotangent, &info.normal)
                .expect("Normal, tangent and cotangent don't for a valid basis.");
        let bsdf_to_world_space = world_to_bsdf_space
            .try_inverse()
            .expect("Expected matrix to be invertable.");
        self.lights
            .iter()
            .map(|light| {
                match sampler.sample(&Ray::new(info.location, light.direction).bias(0.000_000_1)) {
                    Some(_) => self.ambient_light.emit_photon(&photon),
                    None => info.material.bsdf()(
                        &(world_to_bsdf_space * info.retro),
                        &(world_to_bsdf_space * light.direction),
                        &light
                            .spectrum
                            .emit_photon(&photon)
                            .scale_intensity(light.direction.dot(&info.normal).abs()),
                    ),
                }
            })
            .chain(
                [info.material.sample(&(world_to_bsdf_space * info.retro))]
                    .iter()
                    .map(|direction| {
                        let world_space_direction = bsdf_to_world_space * direction;
                        match sampler.sample(
                            &Ray::new(info.location, world_space_direction).bias(0.000_000_1),
                        ) {
                            Some(recursive_hit) => {
                                if recursion_limit > 0 {
                                    let photon = info.material.bsdf()(
                                        &(world_to_bsdf_space * info.retro),
                                        direction,
                                        &self.integrate(
                                            &sampler,
                                            &recursive_hit,
                                            &photon,
                                            recursion_limit - 1,
                                        ),
                                    );
                                    photon.scale_intensity(
                                        world_space_direction.dot(&info.normal).abs(),
                                    )
                                } else {
                                    photon.scale_intensity(0.0)
                                }
                            }
                            None => photon.scale_intensity(0.0),
                        }
                    }),
            )
            .fold(photon.clone(), |a, b| {
                let mut result = a;
                result.intensity += b.intensity;
                result
            })
    }
}

pub struct SimpleRandomIntegrator {}

impl Integrator for SimpleRandomIntegrator {
    fn integrate(
        &self,
        sampler: &Sampler,
        info: &IntersectionInfo,
        photon: &Photon,
        recursion_limit: u16,
    ) -> Photon {
        if recursion_limit == 0 {
            return Photon {
                wavelength: 0.0,
                intensity: 0.0,
            };
        }
        let world_to_bsdf_space =
            try_change_of_basis_matrix(&info.tangent, &info.cotangent, &info.normal)
                .expect("Normal, tangent and cotangent don't form a valid basis.");
        let bsdf_to_world_space = world_to_bsdf_space
            .try_inverse()
            .expect("Expected matrix to be invertable.");
        let w_i = info.material.sample(&(world_to_bsdf_space * info.retro));
        let world_space_w_i = bsdf_to_world_space * w_i;
        info.material.bsdf()(
            &(world_to_bsdf_space * info.retro),
            &w_i,
            &match sampler.sample(&Ray::new(info.location, world_space_w_i).bias(0.000_000_1)) {
                None => photon.set_intensity(world_space_w_i.y()),
                Some(recursive_hit) => {
                    self.integrate(&sampler, &recursive_hit, &photon, recursion_limit - 1)
                }
            }
            .scale_intensity(world_space_w_i.dot(&info.normal).abs()),
        )
    }
}

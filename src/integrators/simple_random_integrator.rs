use crate::colour::{ColourRgbF, Photon, Spectrum};
use crate::materials::MaterialSampleResult;
use crate::math::Vec3;
use crate::raycasting::{IntersectionInfo, Ray};
use crate::sampler::Sampler;
use crate::util::algebra_utils::try_change_of_basis_matrix;

use super::Integrator;

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
        let world_space_w_i = info.retro;
        let w_i = world_to_bsdf_space * world_space_w_i;
        let MaterialSampleResult {
            direction: w_o,
            pdf: w_o_pdf,
        } = info.material.sample(&w_i, &photon);
        let world_space_w_o = bsdf_to_world_space * w_o;
        info.material.bsdf()(
            &w_o,
            &w_i,
            &match sampler.sample(&Ray::new(info.location, world_space_w_o).bias(0.000_000_1)) {
                None => photon.set_intensity(test_lighting_environment(
                    &world_space_w_o,
                    photon.wavelength,
                )),
                Some(recursive_hit) => {
                    self.integrate(&sampler, &recursive_hit, &photon, recursion_limit - 1)
                }
            }
            .scale_intensity(w_o_pdf)
            .scale_intensity(world_space_w_o.dot(&info.normal).abs()),
        )
    }
}

pub fn test_lighting_environment(w_o: &Vec3, wavelength: f64) -> f64 {
    let sun_direction = Vec3::new(1.0, 1.0, -1.0).normalize();
    if w_o.dot(&sun_direction) >= 0.99 {
        300.0
    } else {
        let sky_colour = ColourRgbF::new(w_o.y(), w_o.y(), 1.0);
        Spectrum::reflection_from_linear_rgb(&sky_colour).intensity_at_wavelength(wavelength)
    }
}

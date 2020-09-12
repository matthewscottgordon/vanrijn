use crate::colour::{Photon, Spectrum};
use crate::materials::{Material, MaterialSampleResult};
use crate::math::Vec3;

use rand::random;

#[derive(Debug)]
struct FresnelResult {
    reflection_direction: Vec3,
    reflection_strength: f64,
    transmission_direction: Vec3,
    transmission_strength: f64,
}

fn fresnel(w_i: &Vec3, eta1: f64, eta2: f64) -> FresnelResult {
    let normal = if w_i.z() > 0.0 {
        Vec3::unit_z()
    } else {
        -Vec3::unit_z()
    };
    let reflection_direction = Vec3::new(-w_i.x(), -w_i.y(), w_i.z());
    let r = eta1 / eta2;
    let cos_theta1 = normal.dot(w_i);
    let cos_theta2_squared = 1.0 - r * r * (1.0 - cos_theta1 * cos_theta1);
    let mut result = if cos_theta2_squared >= 0.0 {
        let cos_theta2 = cos_theta2_squared.sqrt();
        let reflection_strength_parallel_sqrt =
            (eta1 * cos_theta2 - eta2 * cos_theta1) / (eta1 * cos_theta2 + eta2 * cos_theta1);
        let reflection_strength_perpendicular_sqrt =
            (eta1 * cos_theta1 - eta2 * cos_theta2) / (eta1 * cos_theta1 + eta2 * cos_theta2);
        let reflection_strength = 0.5
            * (reflection_strength_parallel_sqrt * reflection_strength_parallel_sqrt
                + reflection_strength_perpendicular_sqrt * reflection_strength_perpendicular_sqrt);
        let transmission_direction =
            (-r * w_i + (r * cos_theta1 - cos_theta2) * normal).normalize();
        let transmission_strength = 1.0 - reflection_strength;
        FresnelResult {
            reflection_direction,
            reflection_strength,
            transmission_direction,
            transmission_strength,
        }
    } else {
        let reflection_strength = 1.0;
        let transmission_strength = 0.0;
        let transmission_direction = Default::default();
        FresnelResult {
            reflection_direction,
            reflection_strength,
            transmission_direction,
            transmission_strength,
        }
    };
    if w_i.z() < 0.0 {
        result.reflection_direction.coords[2] *= -1.0;
        result.transmission_direction.coords[2] *= -1.0;
    }
    result
}

#[derive(Debug)]
pub struct SmoothTransparentDialectric {
    eta: Spectrum,
}

impl SmoothTransparentDialectric {
    pub fn new(eta: Spectrum) -> SmoothTransparentDialectric {
        SmoothTransparentDialectric { eta }
    }
}

impl Material for SmoothTransparentDialectric {
    fn bsdf<'a>(&'a self) -> Box<dyn Fn(&Vec3, &Vec3, &Photon) -> Photon + 'a> {
        Box::new(move |w_o: &Vec3, w_i: &Vec3, photon_in: &Photon| {
            let (eta1, eta2) = if w_i.z() >= 0.0 {
                (1.0, self.eta.intensity_at_wavelength(photon_in.wavelength))
            } else {
                (self.eta.intensity_at_wavelength(photon_in.wavelength), 1.0)
            };
            let fresnel = fresnel(w_i, eta1, eta2);
            if (*w_o - fresnel.reflection_direction).norm_squared() < 0.0000000001 {
                photon_in.scale_intensity(fresnel.reflection_strength)
            } else if (*w_o - fresnel.transmission_direction).norm_squared() < 0.0000000001 {
                photon_in.scale_intensity(fresnel.transmission_strength)
            } else {
                photon_in.set_intensity(0.0)
            }
        })
    }

    fn sample(&self, w_i: &Vec3, photon: &Photon) -> MaterialSampleResult {
        let (eta1, eta2) = if w_i.z() >= 0.0 {
            (1.0, self.eta.intensity_at_wavelength(photon.wavelength))
        } else {
            (self.eta.intensity_at_wavelength(photon.wavelength), 1.0)
        };
        let fresnel = fresnel(w_i, eta1, eta2);
        if fresnel.transmission_strength <= 0.0000000001 {
            MaterialSampleResult {
                direction: fresnel.reflection_direction,
                pdf: 0.5,
            }
        } else if fresnel.reflection_strength <= 0.0000000001 || random() {
            MaterialSampleResult {
                direction: fresnel.transmission_direction,
                pdf: 0.5,
            }
        } else {
            MaterialSampleResult {
                direction: fresnel.reflection_direction,
                pdf: 0.5,
            }
        }
    }
}

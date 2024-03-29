use crate::colour::{LONGEST_VISIBLE_WAVELENGTH, SHORTEST_VISIBLE_WAVELENGTH};

use rand::random;

/// A quantum of light with a given wavelength and intensity
#[derive(Clone, Default, Debug)]
pub struct Photon {
    /// The wavelength in nanometres
    pub wavelength: f64,
    /// The intensity of the light
    ///
    /// Depending on context, this might represent actual intensity in W/sr,
    /// radiant flux in W, irradiance in W/m^2, or radiance in W/(m^2sr).
    pub intensity: f64,
}

impl Photon {
    pub fn random_wavelength() -> Photon {
        Photon {
            wavelength: SHORTEST_VISIBLE_WAVELENGTH
                + (LONGEST_VISIBLE_WAVELENGTH - SHORTEST_VISIBLE_WAVELENGTH) * random::<f64>(),
            intensity: 0.0,
        }
    }

    pub fn random_wavelength_pdf(_wavelength: f64) -> f64 {
        LONGEST_VISIBLE_WAVELENGTH - SHORTEST_VISIBLE_WAVELENGTH
    }

    pub fn scale_intensity(&self, scale_factor: f64) -> Photon {
        Photon {
            wavelength: self.wavelength,
            intensity: self.intensity * scale_factor,
        }
    }

    pub fn set_intensity(&self, intensity: f64) -> Photon {
        Photon {
            wavelength: self.wavelength,
            intensity,
        }
    }
}

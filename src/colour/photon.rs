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
    pub fn scale_intensity(&self, scale_factor: f64) -> Photon {
        Photon {
            wavelength: self.wavelength,
            intensity: self.intensity * scale_factor,
        }
    }

    pub fn set_intensity(&self, intensity: f64) -> Photon {
        Photon {
            wavelength: self.wavelength,
            intensity: intensity,
        }
    }
}

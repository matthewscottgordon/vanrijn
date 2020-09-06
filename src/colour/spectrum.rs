use crate::colour::{ColourRgbF, Photon};

#[derive(Debug)]
pub struct Spectrum {}

impl Spectrum {
    pub fn from_linear_rgb(colour: &ColourRgbF) -> Spectrum {
        Spectrum {}
    }

    pub fn intensity_at_wavelength(&self, wavelength: f64) -> f64 {
        1.0
    }

    pub fn scale_photon(&self, photon: &Photon) -> Photon {
        let wavelength = photon.wavelength;
        photon.scale_intensity(self.intensity_at_wavelength(wavelength))
    }

    pub fn emit_photon(&self, photon: &Photon) -> Photon {
        let wavelength = photon.wavelength;
        photon.set_intensity(self.intensity_at_wavelength(wavelength))
    }
}

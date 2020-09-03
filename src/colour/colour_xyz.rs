use crate::math::{Mat3, Vec3};

use super::ColourRgbF;

/// A CIE XYZ Colour Value
#[derive(Default, Clone, Copy, Debug)]
pub struct ColourXyz {
    pub values: Vec3,
}

impl ColourXyz {
    /// Construct a ColourXyz with the specified XYZ values
    pub fn new(x: f64, y: f64, z: f64) -> ColourXyz {
        ColourXyz {
            values: Vec3::new(x, y, z),
        }
    }

    /// Calculate the XYZ colour of a laser light with the given wavelength
    ///
    /// The wavelength is in nanometres.
    pub fn for_wavelength(wavelength: f64) -> ColourXyz {
        let values = Vec3::new(
            colour_matching_function_x(wavelength),
            colour_matching_function_y(wavelength),
            colour_matching_function_z(wavelength),
        );
        ColourXyz { values }
    }

    pub fn x(&self) -> f64 {
        self.values.x()
    }

    pub fn y(&self) -> f64 {
        self.values.y()
    }

    pub fn z(&self) -> f64 {
        self.values.z()
    }

    pub fn to_linear_rgb(&self) -> ColourRgbF {
        let transform = Mat3::from_rows(
            &Vec3::new(3.24096994, -1.53738318, -0.49861076),
            &Vec3::new(-0.96924364, 1.87596750, 0.04155506),
            &Vec3::new(0.05563008, -0.20397696, 1.05697151),
        );
        ColourRgbF::from_vec3(&(transform * self.values))
    }

    pub fn from_linear_rgb(rgb: &ColourRgbF) -> ColourXyz {
        let transform = Mat3::from_rows(
            &Vec3::new(0.41239080, 0.35758434, 0.18048079),
            &Vec3::new(0.21263901, 0.71516868, 0.07219232),
            &Vec3::new(0.01933082, 0.11919478, 0.95053215),
        );
        ColourXyz {
            values: transform * rgb.values,
        }
    }
}

fn gaussian(wavelength: f64, alpha: f64, mu: f64, sigma1: f64, sigma2: f64) -> f64 {
    let denominator = 2.0 * (if wavelength < mu { sigma1 } else { sigma2 }).powi(2);
    alpha * (-(wavelength - mu).powi(2) / denominator).exp()
}

fn colour_matching_function_x(wavelength: f64) -> f64 {
    gaussian(wavelength, 1.056, 599.8, 37.9, 31.0)
        + gaussian(wavelength, 0.362, 442.0, 16.0, 26.7)
        + gaussian(wavelength, -0.065, 501.1, 20.4, 26.2)
}

fn colour_matching_function_y(wavelength: f64) -> f64 {
    gaussian(wavelength, 0.821, 568.8, 46.9, 40.5) + gaussian(wavelength, 0.286, 530.9, 16.3, 31.1)
}

fn colour_matching_function_z(wavelength: f64) -> f64 {
    gaussian(wavelength, 1.217, 437.0, 11.8, 36.0) + gaussian(wavelength, 0.681, 459.0, 26.0, 13.8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_returns_zero_for_default() {
        let target: ColourXyz = Default::default();
        assert!(target.x() == 0.0);
    }

    #[test]
    fn x_returns_specified_value_after_constructioni_with_new() {
        let target = ColourXyz::new(0.1, 0.2, 0.3);
        assert!(target.x() == 0.1);
    }

    #[test]
    fn z_returns_specified_value_after_constructioni_with_new() {
        let target = ColourXyz::new(0.1, 0.2, 0.3);
        assert!(target.z() == 0.3);
    }

    #[test]
    fn roundtrip_to_linear_rgb_yields_original_values() {
        let target = ColourXyz::new(0.1, 0.2, 0.3);
        let rgb = target.to_linear_rgb();
        let xyz = ColourXyz::from_linear_rgb(&rgb);
        assert!((target.values - xyz.values).norm() < 0.00000001);
    }
}

pub mod colour_rgb;
pub use colour_rgb::{ColourRgbF, ColourRgbU8, NamedColour};

pub mod photon;
pub use photon::Photon;

pub mod colour_xyz;
pub use colour_xyz::ColourXyz;

pub mod spectrum;
pub use spectrum::Spectrum;

pub const SHORTEST_VISIBLE_WAVELENGTH: f64 = 380.0;
pub const LONGEST_VISIBLE_WAVELENGTH: f64 = 740.0;

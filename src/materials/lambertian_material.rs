use crate::colour::ColourRgbF;
use crate::math::Vec3;

use super::{Bsdf, Material};

use std::fmt::Debug;

#[derive(Debug)]
pub struct LambertianMaterial {
    pub colour: ColourRgbF,
    pub diffuse_strength: f64,
}

impl LambertianMaterial {
    pub fn new_dummy() -> LambertianMaterial {
        LambertianMaterial {
            colour: ColourRgbF::new(1.0, 1.0, 1.0),
            diffuse_strength: 1.0,
        }
    }
}

impl Material for LambertianMaterial {
    fn bsdf(&self) -> Bsdf {
        let colour = self.colour * self.diffuse_strength;
        Box::new(move |_w_o: Vec3, _w_i: Vec3, colour_in: ColourRgbF| colour * colour_in)
    }
}

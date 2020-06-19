use nalgebra::{clamp, convert, Vector3};

use crate::colour::ColourRgbF;
use crate::Real;

use std::fmt::Debug;

use super::{Bsdf, Material};
#[derive(Debug)]
pub struct ReflectiveMaterial<T: Real> {
    pub colour: ColourRgbF<T>,
    pub diffuse_strength: T,
    pub reflection_strength: T,
}

impl<T: Real> Material<T> for ReflectiveMaterial<T> {
    fn bsdf(&self) -> Bsdf<T> {
        let diffuse_colour_factor = self.colour * self.diffuse_strength;
        let reflection_strength = self.reflection_strength;
        Box::new(
            move |w_o: Vector3<T>, w_i: Vector3<T>, colour_in: ColourRgbF<T>| {
                if w_i.z <= T::zero() || w_o.z <= T::zero() {
                    ColourRgbF::new(T::zero(), T::zero(), T::zero())
                } else {
                    let reflection_vector = Vector3::new(-w_o.x, -w_o.y, w_o.z);
                    let reflection_colour = colour_in * reflection_strength;
                    let diffuse_colour = diffuse_colour_factor * colour_in;
                    let sigma: T = convert(0.05);
                    let two: T = convert(2.0);
                    // These are normalized vectors, but sometimes rounding errors cause the
                    // dot product to be slightly above 1 or below 0. The call to clamp
                    // ensures the values stay within the domain of acos,
                    let theta = clamp(w_i.dot(&reflection_vector), T::zero(), T::one())
                        .abs()
                        .acos();
                    let reflection_factor = (-(theta.powf(two)) / (two * sigma * sigma)).exp();
                    reflection_colour * reflection_factor
                        + diffuse_colour * (T::one() - reflection_factor)
                }
            },
        )
    }

    fn sample(&self, w_o: &Vector3<T>) -> Vec<Vector3<T>> {
        vec![Vector3::new(-w_o.x, -w_o.y, w_o.z)]
    }
}

use nalgebra::{clamp, convert, Vector3};

use super::colour::{ColourRgbF, NamedColour};
use crate::Real;

use std::fmt::Debug;

type Bsdf<T> = Box<dyn Fn(Vector3<T>, Vector3<T>, ColourRgbF<T>) -> ColourRgbF<T>>;

pub trait Material<T: Real>: Debug + Sync + Send {
    fn bsdf(&self) -> Bsdf<T>;

    fn sample(&self, _w_o: &Vector3<T>) -> Vec<Vector3<T>> {
        vec![]
    }
}

#[derive(Debug)]
pub struct LambertianMaterial<T: Real> {
    pub colour: ColourRgbF<T>,
    pub diffuse_strength: T,
}

impl<T: Real> LambertianMaterial<T> {
    pub fn new_dummy() -> LambertianMaterial<T> {
        LambertianMaterial {
            colour: ColourRgbF::new(T::one(), T::one(), T::one()),
            diffuse_strength: T::one(),
        }
    }
}

impl<T: Real> Material<T> for LambertianMaterial<T> {
    fn bsdf(&self) -> Bsdf<T> {
        let colour = self.colour * self.diffuse_strength;
        Box::new(
            move |_w_o: Vector3<T>, _w_i: Vector3<T>, colour_in: ColourRgbF<T>| colour * colour_in,
        )
    }
}

#[derive(Debug)]
pub struct PhongMaterial<T: Real> {
    pub colour: ColourRgbF<T>,
    pub diffuse_strength: T,
    pub specular_strength: T,
    pub smoothness: T,
}

impl<T: Real> Material<T> for PhongMaterial<T> {
    fn bsdf(&self) -> Bsdf<T> {
        let smoothness = self.smoothness;
        let specular_strength = self.specular_strength;
        let colour = self.colour * self.diffuse_strength;
        Box::new(
            move |w_o: Vector3<T>, w_i: Vector3<T>, colour_in: ColourRgbF<T>| {
                if w_i.z < T::zero() || w_o.z < T::zero() {
                    ColourRgbF::from_vector3(&Vector3::zeros())
                } else {
                    let reflection_vector = Vector3::new(-w_i.x, -w_i.y, w_i.z);
                    colour * colour_in
                        + ColourRgbF::from_named(NamedColour::White)
                            * w_o.dot(&reflection_vector).abs().powf(smoothness)
                            * (specular_strength / w_i.dot(&Vector3::z_axis()))
                }
            },
        )
    }
}

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
                if w_i.z < T::zero() || w_o.z < T::zero() {
                    ColourRgbF::new(T::zero(), T::one(), T::one())
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

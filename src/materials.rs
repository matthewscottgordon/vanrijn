use nalgebra::{convert, RealField, Vector3};

use super::colour::{ColourRgbF, NamedColour};

use std::fmt::Debug;

pub trait Material<T: RealField>: Debug {
    fn bsdf<'a>(
        &'a self,
    ) -> Box<dyn Fn(Vector3<T>, Vector3<T>, ColourRgbF<T>) -> ColourRgbF<T> + 'a>;

    fn sample(&self, _w_o: &Vector3<T>) -> Vec<Vector3<T>> {
        vec![]
    }
}

#[derive(Debug)]
pub struct LambertianMaterial<T: RealField> {
    pub colour: ColourRgbF<T>,
    pub diffuse_strength: T,
}

impl<T: RealField> LambertianMaterial<T> {
    pub fn new_dummy() -> LambertianMaterial<T> {
        LambertianMaterial {
            colour: ColourRgbF::new(T::one(), T::one(), T::one()),
            diffuse_strength: T::one(),
        }
    }
}

impl<T: RealField> Material<T> for LambertianMaterial<T> {
    fn bsdf<'a>(
        &'a self,
    ) -> Box<dyn Fn(Vector3<T>, Vector3<T>, ColourRgbF<T>) -> ColourRgbF<T> + 'a> {
        Box::new(
            move |_w_o: Vector3<T>, _w_i: Vector3<T>, colour_in: ColourRgbF<T>| {
                self.colour * colour_in * self.diffuse_strength
            },
        )
    }
}

#[derive(Debug)]
pub struct PhongMaterial<T: RealField> {
    pub colour: ColourRgbF<T>,
    pub diffuse_strength: T,
    pub specular_strength: T,
    pub smoothness: T,
}

impl<T: RealField> Material<T> for PhongMaterial<T> {
    fn bsdf<'a>(
        &'a self,
    ) -> Box<dyn Fn(Vector3<T>, Vector3<T>, ColourRgbF<T>) -> ColourRgbF<T> + 'a> {
        Box::new(
            move |w_o: Vector3<T>, w_i: Vector3<T>, colour_in: ColourRgbF<T>| {
                if w_i.z < T::zero() || w_o.z < T::zero() {
                    ColourRgbF::from_vector3(&Vector3::zeros())
                } else {
                    let reflection_vector = Vector3::new(-w_i.x, -w_i.y, w_i.z);
                    self.colour * colour_in * self.diffuse_strength
                        + ColourRgbF::from_named(NamedColour::White)
                            * w_o.dot(&reflection_vector).abs().powf(self.smoothness)
                            * (self.specular_strength / w_i.dot(&Vector3::z_axis()))
                }
            },
        )
    }
}

#[derive(Debug)]
pub struct ReflectiveMaterial<T: RealField> {
    pub colour: ColourRgbF<T>,
    pub diffuse_strength: T,
    pub reflection_strength: T,
}

impl<T: RealField> Material<T> for ReflectiveMaterial<T> {
    fn bsdf<'a>(
        &'a self,
    ) -> Box<dyn Fn(Vector3<T>, Vector3<T>, ColourRgbF<T>) -> ColourRgbF<T> + 'a> {
        Box::new(
            move |w_o: Vector3<T>, w_i: Vector3<T>, colour_in: ColourRgbF<T>| {
                if w_i.z < T::zero() || w_o.z < T::zero() {
                    ColourRgbF::new(T::zero(), T::one(), T::one())
                } else {
                    let reflection_vector = Vector3::new(-w_o.x, -w_o.y, w_o.z);
                    let reflection_colour = colour_in * self.reflection_strength;
                    let diffuse_colour = self.colour * colour_in * self.diffuse_strength;
                    let sigma: T = convert(0.001);
                    let two: T = convert(2.0);
                    let reflection_factor = (-w_i.dot(&reflection_vector).acos().powf(two)
                        / (two * sigma * sigma))
                    .exp();
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

#![feature(test)]
extern crate test;

pub mod algebra_utils;
pub mod camera;
pub mod colour;
pub mod image;
pub mod integrators;
pub mod materials;
pub mod mesh;
pub mod raycasting;
pub mod sampler;
pub mod scene;

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::camera::render_scene;
    use super::colour::{ColourRgbF, NamedColour};
    use super::image::ImageRgbF;
    use super::materials::{LambertianMaterial, PhongMaterial, ReflectiveMaterial};
    use super::mesh::load_obj;
    use super::raycasting::{Intersect, Plane, Sphere};
    use super::scene::Scene;

    use nalgebra::{Point3, Vector3};

    use std::path::Path;
    use std::sync::{Arc, Mutex};

    #[bench]
    fn simple_scene(b: &mut Bencher) {
        let image_width = 3;
        let image_height = 3;

        let output_image =
            Arc::new(Mutex::new(ImageRgbF::<f64>::new(image_width, image_height)));

        let scene = Arc::new(Scene {
            camera_location: Point3::new(-2.0, 1.0, -5.0),
            objects: load_obj(
                Path::new("/home/matthew/Downloads/bunny.obj"),
                Arc::new(ReflectiveMaterial {
                    colour: ColourRgbF::from_named(NamedColour::Yellow),
                    diffuse_strength: 0.05,
                    reflection_strength: 0.9,
                }),
            )
            .unwrap()
            .into_iter()
            .map(|triangle| Box::new(triangle) as Box<dyn Intersect<f64>>)
            .chain(vec![
                Box::new(Plane::new(
                    Vector3::new(0.0, 1.0, 0.0),
                    -2.0,
                    Arc::new(LambertianMaterial {
                        colour: ColourRgbF::new(0.55, 0.27, 0.04),
                        diffuse_strength: 0.1,
                    }),
                )) as Box<dyn Intersect<f64>>,
                Box::new(Sphere::new(
                    Point3::new(-6.25, -0.5, 1.0),
                    1.0,
                    Arc::new(LambertianMaterial {
                        colour: ColourRgbF::from_named(NamedColour::Green),
                        diffuse_strength: 0.1,
                    }),
                )),
                Box::new(Sphere::new(
                    Point3::new(-4.25, -0.5, 2.0),
                    1.0,
                    Arc::new(ReflectiveMaterial {
                        colour: ColourRgbF::from_named(NamedColour::Blue),
                        diffuse_strength: 0.01,
                        reflection_strength: 0.99,
                    }),
                )),
                Box::new(Sphere::new(
                    Point3::new(-5.0, 1.5, 1.0),
                    1.0,
                    Arc::new(PhongMaterial {
                        colour: ColourRgbF::from_named(NamedColour::Red),
                        diffuse_strength: 0.05,
                        smoothness: 100.0,
                        specular_strength: 1.0,
                    }),
                )),
            ])
            .collect(),
        });
        b.iter(|| render_scene(output_image.clone(), scene.clone()));
    }
}

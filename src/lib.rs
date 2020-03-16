#![feature(test)]

pub mod camera;
pub mod colour;
pub mod image;
pub mod integrators;
pub mod materials;
pub mod mesh;
pub mod raycasting;
pub mod realtype;
pub mod sampler;
pub mod scene;
pub mod util;

use realtype::Real;

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use crate::camera::partial_render_scene;
    use crate::colour::{ColourRgbF, NamedColour};
    use crate::materials::{LambertianMaterial, PhongMaterial, ReflectiveMaterial};
    use crate::mesh::load_obj;
    use crate::raycasting::{BoundingVolumeHierarchy, Plane, Primitive, Sphere};
    use crate::scene::Scene;
    use crate::util::Tile;

    use nalgebra::{Point3, Vector3};

    use std::path::Path;
    use std::sync::Arc;

    #[cfg(feature = "slow_tests")]
    #[bench]
    fn simple_scene(b: &mut Bencher) {
        let image_width = 4;
        let image_height = 4;

        let scene = Arc::new(Scene {
            camera_location: Point3::new(-2.0, 1.0, -5.0),
            objects: vec![
                Arc::new(Plane::new(
                    Vector3::new(0.0, 1.0, 0.0),
                    -2.0,
                    Arc::new(LambertianMaterial {
                        colour: ColourRgbF::new(0.55, 0.27, 0.04),
                        diffuse_strength: 0.1,
                    }),
                )) as Arc<dyn Primitive<f64>>,
                Arc::new(Sphere::new(
                    Point3::new(-6.25, -0.5, 1.0),
                    1.0,
                    Arc::new(LambertianMaterial {
                        colour: ColourRgbF::from_named(NamedColour::Green),
                        diffuse_strength: 0.1,
                    }),
                )),
                Arc::new(Sphere::new(
                    Point3::new(-4.25, -0.5, 2.0),
                    1.0,
                    Arc::new(ReflectiveMaterial {
                        colour: ColourRgbF::from_named(NamedColour::Blue),
                        diffuse_strength: 0.01,
                        reflection_strength: 0.99,
                    }),
                )),
                Arc::new(Sphere::new(
                    Point3::new(-5.0, 1.5, 1.0),
                    1.0,
                    Arc::new(PhongMaterial {
                        colour: ColourRgbF::from_named(NamedColour::Red),
                        diffuse_strength: 0.05,
                        smoothness: 100.0,
                        specular_strength: 1.0,
                    }),
                )),
                Arc::new(BoundingVolumeHierarchy::build(
                    &load_obj(
                        Path::new("/home/matthew/Downloads/bunny.obj"),
                        Arc::new(PhongMaterial {
                            colour: ColourRgbF::from_named(NamedColour::Yellow),
                            diffuse_strength: 0.05,
                            smoothness: 100.0,
                            specular_strength: 1.0,
                        }),
                    )
                    .unwrap(),
                )),
            ],
        });

        b.iter(move || {
            let scene_ptr = Arc::clone(&scene);
            let tile = Tile {
                start_column: 0,
                end_column: image_width,
                start_row: 0,
                end_row: image_height,
            };
            partial_render_scene(scene_ptr, tile, image_height, image_width);
        });
    }
}

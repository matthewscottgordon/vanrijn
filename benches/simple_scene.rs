use criterion::{criterion_group, criterion_main, Criterion};

use vanrijn::camera::partial_render_scene;
use vanrijn::colour::{ColourRgbF, NamedColour};
use vanrijn::materials::{LambertianMaterial, PhongMaterial, ReflectiveMaterial};
use vanrijn::mesh::load_obj;
use vanrijn::raycasting::{BoundingVolumeHierarchy, Plane, Primitive, Sphere};
use vanrijn::scene::Scene;
use vanrijn::util::Tile;

use nalgebra::{Point3, Vector3};

use std::path::Path;
use std::sync::Arc;

fn simple_scene(bencher: &mut Criterion) {
    let image_width = 6;
    let image_height = 6;

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

    bencher.bench_function("simple_scene", move |b| {
        let scene = Arc::clone(&scene);
        b.iter(move || {
            let scene = Arc::clone(&scene);
            let tile = Tile {
                start_column: 0,
                end_column: image_width,
                start_row: 0,
                end_row: image_height,
            };
            partial_render_scene(scene, tile, image_height, image_width);
        })
    });
}

criterion_group!(benches, simple_scene);
criterion_main!(benches);
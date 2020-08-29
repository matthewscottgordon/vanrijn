use criterion::{criterion_group, criterion_main, Criterion};

use vanrijn::colour::{ColourRgbF, NamedColour};
use vanrijn::materials::ReflectiveMaterial;
use vanrijn::math::Vec3;
use vanrijn::mesh::load_obj;
use vanrijn::partial_render_scene;
use vanrijn::raycasting::BoundingVolumeHierarchy;
use vanrijn::scene::Scene;
use vanrijn::util::Tile;

use std::path::Path;
use std::sync::Arc;

fn simple_scene(bencher: &mut Criterion) {
    let image_width = 6;
    let image_height = 6;

    let model_file_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("test_data/stanford_bunny.obj");

    bencher.bench_function("simple_scene", |b| {
        let scene = Scene {
            camera_location: Vec3::new(-2.0, 1.0, -5.0),
            objects: vec![Box::new(BoundingVolumeHierarchy::build(
                load_obj(
                    &model_file_path,
                    Arc::new(ReflectiveMaterial {
                        colour: ColourRgbF::from_named(NamedColour::Yellow),
                        diffuse_strength: 0.05,
                        reflection_strength: 0.9,
                    }),
                )
                .unwrap()
                .as_mut_slice(),
            ))],
        };
        b.iter(|| {
            let tile = Tile {
                start_column: 0,
                end_column: image_width,
                start_row: 0,
                end_row: image_height,
            };
            partial_render_scene(&scene, tile, image_height, image_width);
        })
    });
}

criterion_group!(benches, simple_scene);
criterion_main!(benches);

use rayon::prelude::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::Sdl;

use nalgebra::{convert, Point3, Rotation3, Vector3};

use clap::Arg;

use std::path::PathBuf;
use std::sync::{mpsc, Arc};
use std::time::Duration;

use vanrijn::colour::{ColourRgbF, NamedColour};
use vanrijn::image::{ClampingToneMapper, ImageRgbU8, ToneMapper};
use vanrijn::materials::{LambertianMaterial, PhongMaterial, ReflectiveMaterial};
use vanrijn::partial_render_scene;
use vanrijn::raycasting::{Aggregate, Plane, Primitive, Sphere, Transform};
use vanrijn::scene::Scene;
use vanrijn::util::polyhedra::generate_dodecahedron;
use vanrijn::util::{Tile, TileIterator};

#[derive(Debug)]
struct CommandLineParameters {
    width: usize,
    height: usize,
    output_file: Option<PathBuf>,
}

fn parse_args() -> CommandLineParameters {
    let matches = clap::App::new("vanrijn")
        .version("alpha")
        .author("Matthew Gordon <matthew@gordon.earth")
        .arg(
            Arg::with_name("size")
                .long("size")
                .value_name("SIZE")
                .help("The width and height of the output image, in pixels.")
                .takes_value(true)
                .number_of_values(2)
                .required(true),
        )
        .arg(
            Arg::with_name("output_png")
                .long("out")
                .value_name("FILENAME")
                .help("Filename for output PNG.")
                .takes_value(true)
                .required(false),
        )
        .get_matches();
    let mut size_iter = matches.values_of("size").unwrap();
    let width = size_iter.next().unwrap().parse().unwrap();
    let height = size_iter.next().unwrap().parse().unwrap();
    let output_file = matches.value_of_os("output_png").map(|f| PathBuf::from(f));
    CommandLineParameters {
        width,
        height,
        output_file,
    }
}

fn update_texture(tile: &Tile, image: &ImageRgbU8, texture: &mut Texture) {
    texture
        .update(
            Rect::new(
                tile.start_column as i32,
                texture.query().height as i32 - (tile.start_row as i32 + tile.height() as i32),
                tile.width() as u32,
                tile.height() as u32,
            ),
            image.get_pixel_data().as_slice(),
            (image.get_width() * ImageRgbU8::num_channels()) as usize,
        )
        .expect("Couldn't update texture.");
}

fn update_image(tile: &Tile, tile_image: &ImageRgbU8, image: &mut ImageRgbU8) {
    image.update(tile.start_row, tile.start_column, tile_image);
}

fn init_canvas(
    image_width: usize,
    image_height: usize,
) -> Result<(Sdl, Canvas<sdl2::video::Window>), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("van Rijn", image_width as u32, image_height as u32)
        .position_centered()
        .build()?;

    let canvas = window.into_canvas().build().unwrap();

    Ok((sdl_context, canvas))
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parameters = parse_args();
    let image_width = parameters.width;
    let image_height = parameters.height;

    let mut rendered_image = ImageRgbU8::new(image_width, image_height);

    let (sdl_context, mut canvas) = init_canvas(image_width, image_height)?;

    let texture_creator = canvas.texture_creator();
    let mut rendered_image_texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24,
        image_width as u32,
        image_height as u32,
    )?;

    /*let model_file_path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("test_data/stanford_bunny.obj");
        println!("Loading object...");
        let model_object = load_obj(
            &model_file_path,
            Arc::new(ReflectiveMaterial {
                colour: ColourRgbF::from_named(NamedColour::Yellow),
                diffuse_strength: 0.05,
                reflection_strength: 0.9,
            }),
    )?;*/
    let model_object: Vec<_> = generate_dodecahedron(
        Point3::new(0.25, 1.5, 2.5),
        1.0,
        Arc::new(LambertianMaterial {
            colour: ColourRgbF::from_named(NamedColour::Green),
            diffuse_strength: 0.1,
        }),
    )
    .iter()
    .map(|elem| elem.transform(&convert(Rotation3::from_euler_angles(0.0, 0.0, 1.0))))
    .map(|elem| Box::new(elem.clone()) as Box<dyn Primitive<f64>>)
    .collect();
    /*println!("Building BVH...");
    let model_bvh: Box<dyn Aggregate<_>> = Box::new(BoundingVolumeHierarchy::build(model_object));
    println!("Constructing Scene...");*/

    let scene = Scene {
        camera_location: Point3::new(-2.0, 1.0, -5.0),
        objects: vec![
            Box::new(vec![
                Box::new(Plane::new(
                    Vector3::new(0.0, 1.0, 0.0),
                    -2.0,
                    Arc::new(LambertianMaterial {
                        colour: ColourRgbF::new(0.55, 0.27, 0.04),
                        diffuse_strength: 0.1,
                    }),
                )) as Box<dyn Primitive<f64>>,
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
            ]) as Box<dyn Aggregate<f64>>,
            Box::new(model_object),
        ],
    };
    println!("Done.");

    let mut event_pump = sdl_context.event_pump()?;

    let (tile_tx, tile_rx) = mpsc::channel();
    let mut tile_rx = Some(tile_rx);

    let worker_boss = std::thread::spawn(move || {
        let end_tx = tile_tx.clone();
        TileIterator::new(image_width as usize, image_height as usize, 32)
            .map(move |tile| (tile, tile_tx.clone()))
            .par_bridge()
            .try_for_each(|(tile, tx)| {
                let rendered_tile = partial_render_scene(&scene, tile, image_height, image_width);

                // There's nothing we can do if this fails, and we're already
                // at the end of the function anyway, so just ignore result.
                tx.send(Some((tile, rendered_tile))).ok()
            });
        end_tx.send(None).ok();
    });

    'running: loop {
        if let Some(ref tile_rx) = tile_rx {
            for message in tile_rx.try_iter() {
                if let Some((tile, tile_image)) = message {
                    let mut tile_image_rgbu8 = ImageRgbU8::new(tile.width(), tile.height());
                    ClampingToneMapper {}.apply_tone_mapping(&tile_image, &mut tile_image_rgbu8);
                    update_texture(&tile, &tile_image_rgbu8, &mut rendered_image_texture);
                    update_image(&tile, &tile_image_rgbu8, &mut rendered_image);
                    canvas.copy(&rendered_image_texture, None, None).unwrap();
                    canvas.present();
                } else if let Some(image_filename) = parameters.output_file {
                    rendered_image.write_png(&image_filename)?;
                    break 'running;
                }
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    drop(tile_rx.take());
    worker_boss.join().expect("Couldn't join worker threads.");
    Ok(())
}

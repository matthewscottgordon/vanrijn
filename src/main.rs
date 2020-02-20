use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture};
use sdl2::Sdl;
use std::time::Duration;

use nalgebra::{Point3, Vector3};

use std::path::Path;
use std::sync::{Arc, Mutex};

use vanrijn::camera::partial_render_scene;
use vanrijn::colour::{ColourRgbF, NamedColour};
use vanrijn::image::{ClampingToneMapper, ImageRgbF, ImageRgbU8, ToneMapper};
use vanrijn::materials::{LambertianMaterial, PhongMaterial, ReflectiveMaterial};
use vanrijn::mesh::load_obj;
use vanrijn::raycasting::{Plane, Primitive, Sphere};
use vanrijn::scene::Scene;
use vanrijn::util::TileIterator;

fn update_texture(image: &ImageRgbU8, texture: &mut Texture) {
    texture
        .update(
            None,
            image.get_pixel_data().as_slice(),
            (image.get_width() * ImageRgbU8::num_channels()) as usize,
        )
        .expect("Couldn't update texture.");
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
    let image_width = 320usize;
    let image_height = 240usize;

    let (sdl_context, mut canvas) = init_canvas(image_width, image_height)?;

    let texture_creator = canvas.texture_creator();
    let mut rendered_image_texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24,
        image_width as u32,
        image_height as u32,
    )?;
    let output_image = Arc::new(Mutex::new(ImageRgbF::<f64>::new(image_width, image_height)));

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
        .map(|triangle| Arc::new(triangle) as Arc<dyn Primitive<f64>>)
        .chain(vec![
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
        ])
        .collect(),
    });

    let mut event_pump = sdl_context.event_pump()?;

    'running: for tile in TileIterator::new(image_width as usize, image_height as usize, 16) {
        let image_ptr = output_image.clone();
        let scene_ptr = scene.clone();
        partial_render_scene(
            image_ptr,
            scene_ptr,
            tile.start_row,
            tile.end_row,
            tile.start_column,
            tile.end_column,
            image_height,
            image_width,
        );

        let locked_image = output_image.lock().unwrap();
        let mut output_image_rgbu8 = ImageRgbU8::new(image_width, image_height);
        ClampingToneMapper {}.apply_tone_mapping(&locked_image, &mut output_image_rgbu8);
        update_texture(&output_image_rgbu8, &mut rendered_image_texture);
        canvas.copy(&rendered_image_texture, None, None)?;
        canvas.present();

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
    }

    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
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
    Ok(())
}

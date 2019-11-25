use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture};
use sdl2::Sdl;
use std::time::Duration;

use nalgebra::Vector3;

use std::rc::Rc;

use vanrijn::camera::render_scene;
use vanrijn::colour::{ColourRgbF, NamedColour};
use vanrijn::image::{ClampingToneMapper, ImageRgbF, ImageRgbU8, ToneMapper};
use vanrijn::materials::LambertianMaterial;
use vanrijn::raycasting::{Plane, Sphere};
use vanrijn::scene::Scene;

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
    image_width: u32,
    image_height: u32,
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
    let image_width = 1200;
    let image_height = 900;

    let (sdl_context, mut canvas) = init_canvas(image_width, image_height)?;

    let texture_creator = canvas.texture_creator();
    let mut rendered_image_texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24,
        image_width as u32,
        image_height as u32,
    )?;
    let mut output_image = ImageRgbF::<f64>::new(image_width, image_height);

    let scene = Scene {
        camera_location: Vector3::new(0.0, 0.0, 0.0),
        objects: vec![
            Box::new(Plane::new(
                Vector3::new(0.0, 1.0, 0.0),
                -2.0,
                Rc::new(LambertianMaterial {
                    colour: ColourRgbF::new(0.55, 0.27, 0.04),
                }),
            )),
            Box::new(Sphere::new(
                Vector3::new(1.25, -0.5, 6.0),
                1.0,
                Rc::new(LambertianMaterial {
                    colour: ColourRgbF::from_named(NamedColour::Green),
                }),
            )),
            Box::new(Sphere::new(
                Vector3::new(-1.25, -0.5, 6.0),
                1.0,
                Rc::new(LambertianMaterial {
                    colour: ColourRgbF::from_named(NamedColour::Blue),
                }),
            )),
            Box::new(Sphere::new(
                Vector3::new(0.0, 1.5, 6.0),
                1.0,
                Rc::new(LambertianMaterial {
                    colour: ColourRgbF::from_named(NamedColour::Red),
                }),
            )),
        ],
    };
    render_scene(&mut output_image, &scene);
    let mut output_image_rgbu8 = ImageRgbU8::new(image_width, image_height);
    ClampingToneMapper {}.apply_tone_mapping(&output_image, &mut output_image_rgbu8);
    update_texture(&output_image_rgbu8, &mut rendered_image_texture);
    canvas.copy(&rendered_image_texture, None, None)?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
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

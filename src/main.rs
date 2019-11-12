use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture};
use sdl2::Sdl;
use std::time::Duration;

use nalgebra::Vector3;

use vanrijn::camera::render_scene;
use vanrijn::image::OutputImage;
use vanrijn::raycasting::{Plane, Sphere};
use vanrijn::scene::Scene;

fn update_texture(image: &OutputImage, texture: &mut Texture) {
    texture
        .update(
            None,
            image.get_pixel_data().as_slice(),
            (image.get_width() * image.get_num_channels()) as usize,
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
    let mut output_image = OutputImage::new(image_width, image_height);

    let scene = Scene {
        camera_location: Vector3::new(0.0, 0.0, 0.0),
        objects: vec![
            Box::new(Plane::new(Vector3::new(0.0, 1.0, 0.0), -2.0)),
            Box::new(Sphere::new(Vector3::new(0.0, 1.0, 5.0), 1.0)),
        ],
    };
    render_scene(&mut output_image, &scene);
    update_texture(&output_image, &mut rendered_image_texture);
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

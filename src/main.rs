use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture};
use sdl2::Sdl;
use std::time::Duration;

struct OutputImage {
    pixel_data: Vec<u8>,
    width: u32,
    height: u32,
    channels: u32,
}

impl OutputImage {
    fn new(width: u32, height: u32) -> OutputImage {
        OutputImage {
            width: width,
            height: height,
            channels: 3,
            pixel_data: vec![0; (width * height * 3) as usize],
        }
    }

    fn clear(&mut self) -> &mut OutputImage {
        for byte in self.pixel_data.iter_mut() {
            *byte = 0u8;
        }
        self
    }

    fn set_color(&mut self, row: u32, column: u32, red: u8, green: u8, blue: u8) {
        assert!(row < self.height && column < self.width);
        let index = ((row * self.width + column) * self.channels) as usize;
        self.pixel_data[index] = red;
        self.pixel_data[index + 1] = green;
        self.pixel_data[index + 2] = blue;
    }

    fn update_texture(&self, texture: &mut Texture) {
        texture
            .update(
                None,
                self.pixel_data.as_slice(),
                (self.width * self.channels) as usize,
            )
            .expect("Couldn't update texture.");
    }
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
    output_image.clear();

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

        output_image.update_texture(&mut rendered_image_texture);
        for row in 0..image_height {
            for column in 0..image_width {
                output_image.set_color(row, column, i, i, i);
            }
        }

        canvas.copy(&rendered_image_texture, None, None)?;
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

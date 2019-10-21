use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::Texture;
use std::time::Duration;

struct OutputImage {
    pixel_data: Vec<u8>,
    width: usize,
    height: usize,
    channels: usize,
}

impl OutputImage {
    fn new(width: usize, height: usize) -> OutputImage {
        OutputImage {
            width: width,
            height: height,
            channels: 3,
            pixel_data: vec![0; width * height * 3],
        }
    }

    fn clear(&mut self) -> &mut OutputImage {
        for byte in self.pixel_data.iter_mut() {
            *byte = 0u8;
        }
        self
    }

    fn set_color(&mut self, row: usize, column: usize, red: u8, green: u8, blue: u8) {
        let index = (row * self.width + column) * self.channels;
        self.pixel_data[index] = red;
        self.pixel_data[index + 1] = green;
        self.pixel_data[index + 2] = blue;
    }

    fn update_texture(&self, texture: &mut Texture) {
        texture
            .update(None, self.pixel_data.as_slice(), self.width * self.channels)
            .expect("Couldn't update texture.");
    }
}

pub fn main() {
    let image_width = 1200;
    let image_height = 900;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("van Rijn", image_width as u32, image_height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut rendered_image_texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGB24,
            image_width as u32,
            image_height as u32,
        )
        .unwrap();
    let mut output_image = OutputImage::new(image_width, image_height);
    output_image.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();
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

        canvas.copy(&rendered_image_texture, None, None);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

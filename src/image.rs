pub struct OutputImage {
    pixel_data: Vec<u8>,
    width: u32,
    height: u32,
    channels: u32,
}

impl OutputImage {
    pub fn new(width: u32, height: u32) -> OutputImage {
        OutputImage {
            width: width,
            height: height,
            channels: 3,
            pixel_data: vec![0; (width * height * 3) as usize],
        }
    }

    pub fn clear(&mut self) -> &mut OutputImage {
        for byte in self.pixel_data.iter_mut() {
            *byte = 0u8;
        }
        self
    }

    pub fn set_color(&mut self, row: u32, column: u32, red: u8, green: u8, blue: u8) {
        assert!(row < self.height && column < self.width);
        let index = ((row * self.width + column) * self.channels) as usize;
        self.pixel_data[index] = red;
        self.pixel_data[index + 1] = green;
        self.pixel_data[index + 2] = blue;
    }

    pub fn get_pixel_data(&self) -> &Vec<u8> {
        &self.pixel_data
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_num_channels(&self) -> u32 {
        self.channels
    }
}

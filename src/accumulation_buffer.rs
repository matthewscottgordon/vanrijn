use crate::image::{ImageRgbU8, ToneMapper};
use crate::util::{Array2D, Tile};

use std::ops::{Add, Mul};

pub struct AccumulationBuffer<T> {
    colour_buffer: Array2D<T>,
    weight_buffer: Array2D<f64>,
}

impl<T> AccumulationBuffer<T>
where
    T: Add<T, Output = T> + Mul<f64, Output = T> + Default + Copy,
{
    pub fn new(width: usize, height: usize) -> AccumulationBuffer<T> {
        let colour_buffer = Array2D::new(width, height);
        let weight_buffer = Array2D::new(width, height);
        AccumulationBuffer {
            colour_buffer,
            weight_buffer,
        }
    }

    pub fn width(&self) -> usize {
        self.colour_buffer.get_width()
    }

    pub fn height(&self) -> usize {
        self.colour_buffer.get_height()
    }

    pub fn to_image_rgb_u8<Op: ToneMapper<T>>(&self, tone_mapper: &Op) -> ImageRgbU8 {
        let mut result = ImageRgbU8::new(self.width(), self.height());
        tone_mapper.apply_tone_mapping(&self.colour_buffer, &mut result);
        result
    }

    pub fn update_pixel(&mut self, row: usize, column: usize, colour: &T, weight: f64) {
        let buffer_colour = &mut self.colour_buffer[row][column];
        let buffer_weight = &mut self.weight_buffer[row][column];

        *buffer_colour = blend(buffer_colour, *buffer_weight, colour, weight);
        *buffer_weight += weight;
    }

    pub fn update_tile(&mut self, tile: &Tile, colour: &Array2D<T>, weights: &Array2D<f64>) {
        assert!(tile.width() == colour.get_width());
        assert!(tile.height() == colour.get_height());
        assert!(tile.width() == weights.get_width());
        assert!(tile.height() == weights.get_height());
        for i in 0..tile.height() {
            for j in 0..tile.width() {
                self.update_pixel(
                    tile.start_row + i,
                    tile.start_column + j,
                    &colour[i][j],
                    weights[i][j],
                );
            }
        }
    }
}

fn blend<T>(value1: &T, weight1: f64, value2: &T, weight2: f64) -> T
where
    T: Add<T, Output = T> + Mul<f64, Output = T> + Copy,
{
    (*value1 * weight1 + *value2 * weight2) * (1.0 / (weight1 + weight2))
}

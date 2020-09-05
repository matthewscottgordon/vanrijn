use crate::colour::{ColourXyz, Photon};
use crate::image::{ImageRgbU8, ToneMapper};
use crate::util::{Array2D, Tile};

#[derive(Clone)]
pub struct AccumulationBuffer {
    colour_buffer: Array2D<ColourXyz>,
    weight_buffer: Array2D<f64>,
}

impl AccumulationBuffer {
    pub fn new(height: usize, width: usize) -> AccumulationBuffer {
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

    pub fn to_image_rgb_u8<Op: ToneMapper<ColourXyz>>(&self, tone_mapper: &Op) -> ImageRgbU8 {
        let mut result = ImageRgbU8::new(self.width(), self.height());
        tone_mapper.apply_tone_mapping(&self.colour_buffer, &mut result);
        result
    }

    pub fn update_pixel(&mut self, row: usize, column: usize, photon: &Photon, weight: f64) {
        let buffer_colour = &mut self.colour_buffer[row][column];
        let buffer_weight = &mut self.weight_buffer[row][column];

        *buffer_colour = blend(
            buffer_colour,
            *buffer_weight,
            &ColourXyz::from_photon(&photon),
            weight,
        );
        *buffer_weight += weight;
    }

    pub fn merge_tile(&mut self, tile: &Tile, src: &AccumulationBuffer) {
        assert!(tile.width() == src.width());
        assert!(tile.height() == src.height());
        for i in 0..tile.height() {
            for j in 0..tile.width() {
                let dst_colour = &mut self.colour_buffer[tile.start_row + i][tile.start_column + j];
                let dst_weight = &mut self.weight_buffer[tile.start_row + i][tile.start_column + j];
                *dst_colour = blend(
                    dst_colour,
                    *dst_weight,
                    &src.colour_buffer[i][j],
                    src.weight_buffer[i][j],
                );
                *dst_weight += src.weight_buffer[i][j];
            }
        }
    }
}

fn blend(colour1: &ColourXyz, weight1: f64, colour2: &ColourXyz, weight2: f64) -> ColourXyz {
    ColourXyz {
        values: (colour1.values * weight1 + colour2.values * weight2) * (1.0 / (weight1 + weight2)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_expected_width() {
        let target = AccumulationBuffer::new(16, 12);
        assert!(target.width() == 16);
    }

    #[test]
    fn has_expected_height() {
        let target = AccumulationBuffer::new(16, 12);
        assert!(target.height() == 12);
    }

    #[test]
    fn update_pixel_does_not_panic_inside_bounds() {
        let mut target = AccumulationBuffer::new(16, 12);
        for i in 0..12 {
            for j in 0..16 {
                target.update_pixel(i, j, &Default::default(), 1.0);
            }
        }
    }

    #[test]
    #[should_panic]
    fn update_pixel_panics_when_row_to_large() {
        let mut target = AccumulationBuffer::new(16, 12);
        target.update_pixel(12, 0, &Default::default(), 1.0);
    }

    #[test]
    #[should_panic]
    fn update_pixel_panics_when_column_to_large() {
        let mut target = AccumulationBuffer::new(16, 12);
        target.update_pixel(0, 16, &Default::default(), 1.0);
    }

    #[test]
    fn first_update_sets_expected_value() {
        let mut target = AccumulationBuffer::new(16, 12);
        let photon = Photon {
            wavelength: 589.0,
            intensity: 1.5,
        };
        let row = 4;
        let column = 5;
        let weight = 0.8;
        target.update_pixel(row, column, &photon, weight);
        assert!(target.colour_buffer[row][column] == ColourXyz::from_photon(&photon));
        assert!(target.weight_buffer[row][column] == weight);
    }

    #[test]
    fn first_update_only_sets_expected_value() {
        let mut target = AccumulationBuffer::new(16, 12);
        let original = target.clone();
        let photon = Photon {
            wavelength: 589.0,
            intensity: 1.5,
        };
        let set_row = 4;
        let set_column = 5;
        target.update_pixel(set_row, set_column, &photon, 0.8);
        for i in 0..12 {
            for j in 0..16 {
                if i != set_row && j != set_column {
                    assert!(target.colour_buffer[i][j] == original.colour_buffer[i][j]);
                    assert!(target.weight_buffer[i][j] == original.weight_buffer[i][j]);
                }
            }
        }
    }

    #[test]
    fn second_update_blends_colours() {
        let mut target = AccumulationBuffer::new(16, 12);
        let photon1 = Photon {
            wavelength: 589.0,
            intensity: 0.5,
        };
        let photon2 = Photon {
            wavelength: 656.0,
            intensity: 1.5,
        };
        let colour1 = ColourXyz::from_photon(&photon1);
        let colour2 = ColourXyz::from_photon(&photon2);
        let expected_x = (colour1.x() + colour2.x()) / 2.0;
        let expected_y = (colour1.y() + colour2.y()) / 2.0;
        let expected_z = (colour1.z() + colour2.z()) / 2.0;
        let row = 4;
        let column = 5;
        target.update_pixel(row, column, &photon1, 1.0);
        target.update_pixel(row, column, &photon2, 1.0);
        assert!(target.colour_buffer[row][column].x() == expected_x);
        assert!(target.colour_buffer[row][column].y() == expected_y);
        assert!(target.colour_buffer[row][column].z() == expected_z);
    }

    #[test]
    fn second_update_blends_colours_proportionally() {
        let mut target = AccumulationBuffer::new(16, 12);
        let photon1 = Photon {
            wavelength: 589.0,
            intensity: 0.5,
        };
        let photon2 = Photon {
            wavelength: 656.0,
            intensity: 1.5,
        };
        let colour1 = ColourXyz::from_photon(&photon1);
        let colour2 = ColourXyz::from_photon(&photon2);
        let weight1 = 0.75;
        let weight2 = 1.25;
        let expected_x = (colour1.x() * weight1 + colour2.x() * weight2) / (weight1 + weight2);
        let expected_y = (colour1.y() * weight1 + colour2.y() * weight2) / (weight1 + weight2);
        let expected_z = (colour1.z() * weight1 + colour2.z() * weight2) / (weight1 + weight2);
        let row = 4;
        let column = 5;
        target.update_pixel(row, column, &photon1, weight1);
        target.update_pixel(row, column, &photon2, weight2);
        assert!(target.colour_buffer[row][column].x() == expected_x);
        assert!(target.colour_buffer[row][column].y() == expected_y);
        assert!(target.colour_buffer[row][column].z() == expected_z);
    }

    #[test]
    fn third_update_blends_colours_proportionally() {
        let mut target = AccumulationBuffer::new(16, 12);
        let photon1 = Photon {
            wavelength: 589.0,
            intensity: 0.5,
        };
        let photon2 = Photon {
            wavelength: 656.0,
            intensity: 1.5,
        };
        let photon3 = Photon {
            wavelength: 393.0,
            intensity: 1.2,
        };
        let colour1 = ColourXyz::from_photon(&photon1);
        let colour2 = ColourXyz::from_photon(&photon2);
        let colour3 = ColourXyz::from_photon(&photon3);
        let weight1 = 0.75;
        let weight2 = 1.25;
        let weight3 = 0.5;
        let expected_x = (colour1.x() * weight1 + colour2.x() * weight2 + colour3.x() * weight3)
            / (weight1 + weight2 + weight3);
        let expected_y = (colour1.y() * weight1 + colour2.y() * weight2 + colour3.y() * weight3)
            / (weight1 + weight2 + weight3);
        let expected_z = (colour1.z() * weight1 + colour2.z() * weight2 + colour3.z() * weight3)
            / (weight1 + weight2 + weight3);
        let row = 4;
        let column = 5;
        target.update_pixel(row, column, &photon1, weight1);
        target.update_pixel(row, column, &photon2, weight2);
        target.update_pixel(row, column, &photon3, weight3);
        assert!(target.colour_buffer[row][column].x() == expected_x);
        assert!(target.colour_buffer[row][column].y() == expected_y);
        assert!(target.colour_buffer[row][column].z() == expected_z);
    }

    #[test]
    fn merge_tile_produces_same_results_as_applying_photons_directly() {
        let mut single_buffer = AccumulationBuffer::new(16, 12);
        let mut large_buffer = AccumulationBuffer::new(16, 12);
        let mut small_buffer = AccumulationBuffer::new(4, 5);
        let tile = Tile {
            start_column: 3,
            end_column: 7,
            start_row: 4,
            end_row: 9,
        };

        for i in 0..12 {
            for j in 0..16 {
                let wavelength = 350.0 + (i * j) as f64;
                let intensity = 1.0;
                let weight = 0.2 + i as f64 * 0.02 + j as f64 * 0.3;
                single_buffer.update_pixel(
                    i,
                    j,
                    &Photon {
                        wavelength,
                        intensity,
                    },
                    weight,
                );
                large_buffer.update_pixel(
                    i,
                    j,
                    &Photon {
                        wavelength,
                        intensity,
                    },
                    weight,
                );
            }
        }
        for i in 0..5 {
            for j in 0..4 {
                let wavelength = 700.0 - (i * j) as f64;
                let intensity = 1.0;
                let weight = 0.2 + i as f64 * 0.02 + j as f64 * 0.3;
                small_buffer.update_pixel(
                    i,
                    j,
                    &Photon {
                        wavelength,
                        intensity,
                    },
                    weight,
                );
                single_buffer.update_pixel(
                    tile.start_row + i,
                    tile.start_column + j,
                    &Photon {
                        wavelength,
                        intensity,
                    },
                    weight,
                );
            }
        }
        large_buffer.merge_tile(&tile, &small_buffer);

        for i in 0..12 {
            for j in 0..16 {
                assert!(
                    (large_buffer.colour_buffer[i][j].values
                        - single_buffer.colour_buffer[i][j].values)
                        .norm()
                        < 0.0000000001
                );
                assert!(large_buffer.weight_buffer[i][j] == single_buffer.weight_buffer[i][j]);
            }
        }
    }
}

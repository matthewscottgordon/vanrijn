use nalgebra::{RealField, Vector3};

use super::image::OutputImage;
use super::raycasting::{Intersect, IntersectionInfo, Ray};

struct ImageSampler<T: RealField + From<u32> + From<f64>> {
    image_height_pixels: u32,
    image_width_pixels: u32,

    fov_width: T,
    fov_height: T,

    current_row: u32,
    current_column: u32,

    camera_location: Vector3<T>,
    film_plane_distance: T,
}

impl<T: RealField + From<u32> + From<f64>> ImageSampler<T> {
    pub fn new(width: u32, height: u32, camera_location: Vector3<T>) -> ImageSampler<T> {
        let (fov_width, fov_height) = if (width > height) {
            (T::from(width) / (T::from(height)), T::from(1.0))
        } else {
            (T::from(1.0), T::from(width) / T::from(height))
        };
        ImageSampler {
            image_height_pixels: height,
            image_width_pixels: width,
            current_row: 0,
            current_column: 0,
            film_plane_distance: T::from(1.0),
            fov_width,
            fov_height,
            camera_location,
        }
    }

    fn scale(i: u32, n: u32, l: T) -> T {
        let pixel_size = l * (T::from(1.0) / T::from(n));
        (T::from(i) + T::from(0.5)) * pixel_size
    }
}

impl<T: RealField + From<u32> + From<f64>> Iterator for ImageSampler<T> {
    type Item = Ray<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row >= self.image_height_pixels {
            return None;
        }

        let result = Ray::new(
            self.camera_location,
            Vector3::new(
                Self::scale(self.current_column, self.image_width_pixels, self.fov_width),
                Self::scale(self.current_row, self.image_height_pixels, self.fov_height),
                self.film_plane_distance,
            ),
        );

        self.current_column += 1;
        if (self.current_column >= self.image_width_pixels) {
            self.current_column = 0;
            self.current_row += 1;
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    mod imagesampler {
        use super::*;

        #[test]
        fn scale_returns_correct_value_for_zero() {
            let correct_value = (3.0 / 10.0) / 2.0;
            assert!((ImageSampler::scale(0, 10, 3.0) - correct_value).abs() < 0.0000000001)
        }

        #[test]
        fn scale_returns_correct_value_for_last_pixel() {
            let correct_value = 3.0 - (3.0 / 10.0) / 2.0;
            assert!((ImageSampler::scale(9, 10, 3.0) - correct_value).abs() < 0.0000000001)
        }

        #[test]
        fn iterates_over_correct_number_of_samples() {
            let width = 100;
            let height = 200;
            let target = ImageSampler::new(width, height, Vector3::new(0.0, 0.0, 0.0));
            let mut count = 0;
            for sample in target {
                count += 1;
            }
            assert!(count == width * height);
        }
    }
}

use crate::math::Vec3;

use super::accumulation_buffer::AccumulationBuffer;
use super::colour::Photon;
use super::integrators::{Integrator, SimpleRandomIntegrator};
use super::raycasting::Ray;
use super::sampler::Sampler;
use super::scene::Scene;
use super::util::Tile;

use rand::random;

struct ImageSampler {
    image_height_pixels: usize,
    image_width_pixels: usize,

    film_width: f64,
    film_height: f64,
    camera_location: Vec3,
    film_distance: f64,
}

impl ImageSampler {
    pub fn new(width: usize, height: usize, camera_location: Vec3) -> ImageSampler {
        let (film_width, film_height) = {
            let width = width as f64;
            let height = height as f64;
            let film_size = 1.0;
            if width > height {
                (width / height, film_size)
            } else {
                (film_size, width / height)
            }
        };
        ImageSampler {
            image_height_pixels: height,
            image_width_pixels: width,
            film_distance: 1.0,
            film_width,
            film_height,
            camera_location,
        }
    }

    fn scale(i: usize, n: usize, l: f64) -> f64 {
        let n = n as f64;
        let i = i as f64;
        let pixel_size = l * (1.0 / n);
        (i + random::<f64>()) * pixel_size
    }

    fn ray_for_pixel(&self, row: usize, column: usize) -> Ray {
        Ray::new(
            self.camera_location,
            Vec3::new(
                Self::scale(column, self.image_width_pixels, self.film_width)
                    - self.film_width * 0.5,
                Self::scale(
                    self.image_height_pixels - (row + 1),
                    self.image_height_pixels,
                    self.film_height,
                ) - self.film_height * 0.5,
                self.film_distance,
            ),
        )
    }
}

const RECURSION_LIMIT: u16 = 128;

/// Render a rectangular section of the image.
///
/// The contents and the image, along with the camera, are defined by `scene`.
///
/// Assuming an overall image size given by `width` and `height`, the part of the image
/// defined by `tile` is rendered and returned. Rendering a tile at a time allows a partially-
/// rendered image to be displayed to the user.
///
/// # Examples
//
/// ```
/// # use vanrijn::math::Vec3;
/// # use vanrijn::scene::Scene;
/// # use vanrijn::util::TileIterator;
/// # use vanrijn::partial_render_scene;
/// # let scene = Scene { camera_location: Vec3::new(0.0, 0.0, 0.0), objects: vec![] };
/// let image_width = 640;
/// let image_height = 480;
/// let time_size = 32;
/// for tile in TileIterator::new(640, 480, 32) {
///     let tile_image = partial_render_scene( &scene, tile, image_height, image_width );
///     // display and/or save tile_image
/// }
/// ```
pub fn partial_render_scene(
    scene: &Scene,
    tile: Tile,
    height: usize,
    width: usize,
) -> AccumulationBuffer {
    let mut output_image_tile = AccumulationBuffer::new(tile.width(), tile.height());
    let image_sampler = ImageSampler::new(width, height, scene.camera_location);
    let integrator = SimpleRandomIntegrator {};
    let sampler = Sampler { scene };
    for column in 0..tile.width() {
        for row in 0..tile.height() {
            let ray = image_sampler.ray_for_pixel(tile.start_row + row, tile.start_column + column);
            let hit = sampler.sample(&ray);
            let photon = match hit {
                None => Photon {
                    wavelength: 0.0,
                    intensity: 0.0,
                },
                Some(intersection_info) => integrator.integrate(
                    &sampler,
                    &intersection_info,
                    &Photon::random_wavelength(),
                    RECURSION_LIMIT,
                ),
            };
            output_image_tile.update_pixel(
                row,
                column,
                &photon.scale_intensity(Photon::random_wavelength_pdf(photon.wavelength)),
                1.0,
            );
        }
    }
    output_image_tile
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::materials::LambertianMaterial;
    use crate::raycasting::{Intersect, IntersectionInfo, Plane};
    use std::sync::Arc;

    #[cfg(test)]
    mod imagesampler {
        use super::*;

        #[test]
        fn scale_returns_correct_value_for_zero() {
            let correct_value = (3.0 / 10.0) / 2.0;
            assert!((ImageSampler::scale(0, 10, 3.0f64) - correct_value).abs() < 0.5)
        }

        #[test]
        fn scale_returns_correct_value_for_last_pixel() {
            let correct_value = 3.0 - (3.0 / 10.0) / 2.0;
            assert!((ImageSampler::scale(9, 10, 3.0f64) - correct_value).abs() < 0.5)
        }

        #[test]
        fn ray_for_pixel_returns_value_that_intersects_film_plane_at_expected_location() {
            let target = ImageSampler::new(800, 600, Vec3::new(0.0, 0.0, 0.0));
            let ray = target.ray_for_pixel(100, 200);
            let film_plane = Plane::new(
                Vec3::new(0.0, 0.0, 1.0),
                target.film_distance,
                Arc::new(LambertianMaterial::new_dummy()),
            );
            let point_on_film_plane = match film_plane.intersect(&ray) {
                Some(IntersectionInfo {
                    location,
                    distance: _,
                    normal: _,
                    tangent: _,
                    cotangent: _,
                    retro: _,
                    material: _,
                }) => location,
                None => panic!(),
            };
            let expected_x: f64 =
                ImageSampler::scale(200, 800, target.film_width) - target.film_width * 0.5;
            assert!((point_on_film_plane.x() - expected_x).abs() < 0.5 / 200.0);
            let expected_y =
                -ImageSampler::scale(100, 600, target.film_height) + target.film_height * 0.5;
            assert!((point_on_film_plane.y() - expected_y).abs() < 0.5 / 800.0);
        }
    }
}

use nalgebra::{convert, RealField, Vector3};

use super::colour::{ColourRgbF, NamedColour};
use super::image::ImageRgbF;
use super::integrators::{DirectionalLight, Integrator, PhongIntegrator};
use super::raycasting::Ray;
use super::scene::Scene;

struct ImageSampler<T: RealField> {
    image_height_pixels: u32,
    image_width_pixels: u32,

    film_width: T,
    film_height: T,

    camera_location: Vector3<T>,
    film_distance: T,
}

impl<T: RealField> ImageSampler<T> {
    pub fn new(width: u32, height: u32, camera_location: Vector3<T>) -> ImageSampler<T> {
        let (film_width, film_height) = {
            let width: T = convert(width as f64);
            let height: T = convert(height as f64);
            let film_size: T = convert(1.0);
            if width > height {
                (width / height, film_size)
            } else {
                (film_size, width / height)
            }
        };
        ImageSampler {
            image_height_pixels: height,
            image_width_pixels: width,
            film_distance: convert(1.0),
            film_width,
            film_height,
            camera_location,
        }
    }

    fn scale(i: u32, n: u32, l: T) -> T {
        let one: T = convert(1.0);
        let n: T = convert(n as f64);
        let i: T = convert(i as f64);
        let pixel_size: T = l * (one / n);
        (i + convert(0.5)) * pixel_size
    }

    fn ray_for_pixel(&self, row: u32, column: u32) -> Ray<T> {
        Ray::new(
            self.camera_location,
            Vector3::new(
                Self::scale(column, self.image_width_pixels, self.film_width)
                    - self.film_width * convert(0.5),
                Self::scale(row, self.image_height_pixels, self.film_height)
                    - self.film_height * convert(0.5),
                self.film_distance,
            ),
        )
    }
}

pub fn render_scene<T: RealField>(output_image: &mut ImageRgbF<T>, scene: &Scene<T>)
{
    let image_sampler = ImageSampler::new(
        output_image.get_width(),
        output_image.get_height(),
        scene.camera_location,
    );
    let integrator = PhongIntegrator::<T> {
        ambient_light: convert(0.1),
        lights: vec![DirectionalLight {
            direction: Vector3::new(convert(1.0), convert(1.0), convert(-1.0)).normalize(),
            intensity: convert(0.3),
        }],
    };
    for column in 0..output_image.get_width() {
        for row in 0..output_image.get_height() {
            let ray = image_sampler.ray_for_pixel(row, column);
            let hit = scene
                .objects
                .iter()
                .flat_map(|object| object.intersect(&ray))
                .min_by(
                    |a, b| match PartialOrd::partial_cmp(&a.distance, &b.distance) {
                        None => std::cmp::Ordering::Less,
                        Some(ordering) => ordering,
                    },
                );
            let colour = match hit {
                None => ColourRgbF::from_named(NamedColour::Black),
                Some(intersection_info) => integrator.integrate(&intersection_info),
            };
            output_image.set_colour(row, column, colour);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::materials::Material;
    use crate::raycasting::{Intersect, IntersectionInfo, Plane};

    #[cfg(test)]
    mod imagesampler {
        use super::*;

        #[test]
        fn scale_returns_correct_value_for_zero() {
            let correct_value = (3.0 / 10.0) / 2.0;
            assert!((ImageSampler::scale(0, 10, 3.0f64) - correct_value).abs() < 0.0000000001)
        }

        #[test]
        fn scale_returns_correct_value_for_last_pixel() {
            let correct_value = 3.0 - (3.0 / 10.0) / 2.0;
            assert!((ImageSampler::scale(9, 10, 3.0f64) - correct_value).abs() < 0.0000000001)
        }

        #[test]
        fn ray_for_pixel_returns_value_that_intersects_film_plane_at_expected_location() {
            let target = ImageSampler::new(800, 600, Vector3::new(0.0, 0.0, 0.0));
            let ray = target.ray_for_pixel(100, 200);
            let film_plane = Plane::new(
                Vector3::new(0.0, 0.0, 1.0),
                target.film_distance,
                Material::<f64>::new_dummy(),
            );
            let point_on_film_plane = match film_plane.intersect(&ray) {
                Some(IntersectionInfo {
                    location,
                    distance: _,
                    normal: _,
                    retro: _,
                    material: _,
                }) => location,
                None => panic!(),
            };
            let expected_x: f64 =
                ImageSampler::scale(200, 800, target.film_width) - target.film_width * 0.5;
            print!("{}, {}", expected_x, point_on_film_plane);
            assert!((point_on_film_plane.x - expected_x).abs() < 0.0000000001);
            let expected_y =
                ImageSampler::scale(100, 600, target.film_height) - target.film_height * 0.5;
            assert!((point_on_film_plane.y - expected_y).abs() < 0.0000000001);
        }
    }
}

use nalgebra::{convert, Point3, Vector3};

use super::colour::{ColourRgbF, NamedColour};
use super::image::ImageRgbF;
use super::integrators::{DirectionalLight, Integrator, WhittedIntegrator};
use super::raycasting::Ray;
use super::sampler::Sampler;
use super::scene::Scene;

use crate::Real;

use std::sync::{Arc, Mutex};

struct ImageSampler<T: Real> {
    image_height_pixels: usize,
    image_width_pixels: usize,

    film_width: T,
    film_height: T,

    camera_location: Point3<T>,
    film_distance: T,
}

impl<T: Real> ImageSampler<T> {
    pub fn new(width: usize, height: usize, camera_location: Point3<T>) -> ImageSampler<T> {
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

    fn scale(i: usize, n: usize, l: T) -> T {
        let one: T = convert(1.0);
        let n: T = convert(n as f64);
        let i: T = convert(i as f64);
        let pixel_size: T = l * (one / n);
        (i + convert(0.5)) * pixel_size
    }

    fn ray_for_pixel(&self, row: usize, column: usize) -> Ray<T> {
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

pub fn render_scene<T: Real>(output_image: Arc<Mutex<ImageRgbF<T>>>, scene: Arc<Scene<T>>) {
    let height = output_image.lock().unwrap().get_height();
    let width = output_image.lock().unwrap().get_width();
    partial_render_scene(output_image, scene, 0, height, 0, width, height, width)
}

pub fn partial_render_scene<T: Real>(
    output_image: Arc<Mutex<ImageRgbF<T>>>,
    scene: Arc<Scene<T>>,
    row_start: usize,
    row_end: usize,
    column_start: usize,
    column_end: usize,
    height: usize,
    width: usize,
) {
    let image_sampler = ImageSampler::new(width, height, scene.camera_location);
    let ambient_intensity: T = convert(0.0);
    let directional_intensity1: T = convert(7.0);
    let directional_intensity2: T = convert(3.0);
    let directional_intensity3: T = convert(2.0);
    let integrator = WhittedIntegrator::<T> {
        ambient_light: ColourRgbF::from_named(NamedColour::White) * ambient_intensity,
        lights: vec![
            DirectionalLight {
                direction: Vector3::new(convert(1.0), convert(1.0), convert(-1.0)).normalize(),
                colour: ColourRgbF::from_named(NamedColour::White) * directional_intensity1,
            },
            DirectionalLight {
                direction: Vector3::new(convert(-0.5), convert(2.0), convert(-0.5)).normalize(),
                colour: ColourRgbF::from_named(NamedColour::White) * directional_intensity2,
            },
            DirectionalLight {
                direction: Vector3::new(convert(-3.0), convert(0.1), convert(-0.5)).normalize(),
                colour: ColourRgbF::from_named(NamedColour::White) * directional_intensity3,
            },
        ],
    };
    let sampler = Sampler { scene: &scene };
    for column in column_start..column_end {
        for row in row_start..row_end {
            let ray = image_sampler.ray_for_pixel(row, column);
            let hit = sampler.sample(&ray);
            let colour = match hit {
                None => ColourRgbF::from_named(NamedColour::Black),
                Some(intersection_info) => integrator.integrate(&sampler, &intersection_info),
            };
            let mut locked_image = output_image.lock().unwrap();
            locked_image.set_colour(row, column, colour);
        }
    }
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
            assert!((ImageSampler::scale(0, 10, 3.0f64) - correct_value).abs() < 0.0000000001)
        }

        #[test]
        fn scale_returns_correct_value_for_last_pixel() {
            let correct_value = 3.0 - (3.0 / 10.0) / 2.0;
            assert!((ImageSampler::scale(9, 10, 3.0f64) - correct_value).abs() < 0.0000000001)
        }

        #[test]
        fn ray_for_pixel_returns_value_that_intersects_film_plane_at_expected_location() {
            let target = ImageSampler::new(800, 600, Point3::new(0.0, 0.0, 0.0));
            let ray = target.ray_for_pixel(100, 200);
            let film_plane = Plane::new(
                Vector3::new(0.0, 0.0, 1.0),
                target.film_distance,
                Arc::new(LambertianMaterial::<f64>::new_dummy()),
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
            print!("{}, {}", expected_x, point_on_film_plane);
            assert!((point_on_film_plane.x - expected_x).abs() < 0.0000000001);
            let expected_y =
                ImageSampler::scale(100, 600, target.film_height) - target.film_height * 0.5;
            assert!((point_on_film_plane.y - expected_y).abs() < 0.0000000001);
        }
    }
}

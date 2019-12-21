use nalgebra::{convert, Point3, RealField, Vector3};

use crate::materials::Material;

use super::{Intersect, IntersectionInfo, Ray};

use std::sync::Arc;

pub struct Sphere<T: RealField> {
    centre: Point3<T>,
    radius: T,
    material: Arc<dyn Material<T>>,
}

impl<T: RealField> Sphere<T> {
    pub fn new(centre: Point3<T>, radius: T, material: Arc<dyn Material<T>>) -> Sphere<T> {
        Sphere {
            centre,
            radius,
            material,
        }
    }
}

impl<T: RealField> Intersect<T> for Sphere<T> {
    fn intersect<'a>(&'a self, ray: &Ray<T>) -> Option<IntersectionInfo<T>> {
        /*let ray_origin_to_sphere_centre = self.centre - ray.origin;
        let radius_squared = self.radius * self.radius;
        let is_inside_sphere = ray_origin_to_sphere_centre.norm_squared() <= radius_squared;
        // t0/p0 is the point on the ray that's closest to the centre of the sphere
        // ray.direction is normalized, so it's not necessary to divide by its length.
        let t0 = ray_origin_to_sphere_centre.dot(&ray.direction);
        if !is_inside_sphere && t0 < T::zero() {
            // Sphere is behind ray origin
            return None;
        }
        // Squared distance between ray origin and sphere centre
        let d0_squared = (ray_origin_to_sphere_centre).norm_squared();
        // p0, ray.origin and sphere.centre form a right triangle, with p0 at the right corner,
        // Squared distance petween p0 and sphere centre, using Pythagoras
        let p0_dist_from_centre_squared = d0_squared - t0 * t0;
        if p0_dist_from_centre_squared > radius_squared {
            // Sphere is in front of ray but ray misses
            return None;
        }
        let p0_dist_from_centre =p0_dist_from_centre_squared.sqrt();
        // Two more right triangles are formed by p0, the sphere centre, and the two places
        // where the ray intersects the sphere. (Or the ray may be a tangent to the sphere
        // in which case these triangles are degenerate. Here we use Pythagoras again to find
        .// find the distance between p0 and the two intersection points.
        let delta = (radius_squared - p0_dist_from_centre_squared).sqrt();
        let distance = if is_inside_sphere {
            // radius origin is inside sphere
            t0 + delta
        } else {
            t0 - delta
        };
        let location = ray.point_at(distance);
        let normal = (location - self.centre).normalize();
        let tangent = normal.cross(&Vector3::z_axis());
        let cotangent = normal.cross(&tangent);
        let retro = -ray.direction;*/
        let two: T = convert(2.0);
        let four: T = convert(4.0);
        let r_o = ray.origin.coords;
        let centre_coords = self.centre.coords;
        let a = ray
            .direction
            .component_mul(&ray.direction)
            .iter()
            .fold(T::zero(), |a, b| a + *b);
        let b = ((r_o.component_mul(&ray.direction) - centre_coords.component_mul(&ray.direction))
            * two)
            .iter()
            .fold(T::zero(), |a, b| a + *b);
        let c = (r_o.component_mul(&r_o) + centre_coords.component_mul(&centre_coords)
            - centre_coords.component_mul(&r_o) * two)
            .iter()
            .fold(T::zero(), |a, b| a + *b)
            - self.radius * self.radius;
        let delta_squared: T = b * b - four * a * c;
        if delta_squared < T::zero() {
            None
        } else {
            let delta = delta_squared.sqrt();
            let one_over_2_a = T::one() / (two * a);
            let t1 = (-b - delta) * one_over_2_a;
            let t2 = (-b + delta) * one_over_2_a;
            let distance = if t1 < T::zero() || (t2 >= T::zero() && t1 >= t2) {
                t2
            } else {
                t1
            };
            if distance <= T::zero() {
                None
            } else {
                let location = ray.point_at(distance);
                let normal = (location - self.centre).normalize();
                let tangent = normal.cross(&Vector3::z_axis()).normalize();
                let cotangent = normal.cross(&tangent);
                let retro = -ray.direction;
                Some(IntersectionInfo {
                    distance,
                    location,
                    normal,
                    tangent,
                    cotangent,
                    retro,
                    material: Arc::clone(&self.material),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;
    use quickcheck::TestResult;
    
    use super::*;
    use crate::materials::LambertianMaterial;

    #[test]
    fn ray_intersects_sphere() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Point3::new(1.5, 1.5, 15.0),
            5.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        if let None = s.intersect(&r) {
            panic!("Intersection failed");
        }
    }

    #[test]
    fn ray_does_not_intersect_sphere_when_sphere_is_in_front() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Point3::new(-5.0, 1.5, 15.0),
            5.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        if let Some(_) = s.intersect(&r) {
            panic!("Intersection passed.");
        }
    }

    #[test]
    fn ray_does_not_intersect_sphere_when_sphere_is_behind() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Point3::new(1.5, 1.5, -15.0),
            5.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        if let Some(_) = s.intersect(&r) {
            panic!("Intersection failed");
        }
    }

    #[test]
    fn ray_intersects_sphere_when_origin_is_inside() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Point3::new(1.5, 1.5, 2.0),
            5.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        if let None = s.intersect(&r) {
            panic!("Intersection failed");
        }
    }

    #[quickcheck]
    fn ray_intersects_sphere_centre_at_correct_distance(
        ray_origin: Point3<f64>,
        sphere_centre: Point3<f64>,
        radius: f64,
    ) -> TestResult {
        if radius <= 0.0 || radius + 0.000001 >= (ray_origin - sphere_centre).norm() {
            return TestResult::discard();
        };
        let sphere = Sphere::new(
            sphere_centre,
            radius,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let ray = Ray::new(ray_origin, sphere_centre - ray_origin);
        let info = sphere.intersect(&ray).unwrap();
        let distance_to_centre = (sphere_centre - ray.origin).norm();
        TestResult::from_bool(
            (distance_to_centre - (info.distance + sphere.radius)).abs() < 0.00001,
        )
    }
}

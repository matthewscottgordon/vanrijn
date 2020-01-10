use nalgebra::{convert, Point3, RealField, Vector3};

use crate::materials::Material;

use super::{BoundingBox, HasBoundingBox, Intersect, IntersectionInfo, Ray};

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

impl<T: RealField> HasBoundingBox<T> for Sphere<T> {
    fn bounding_box(&self) -> BoundingBox<T> {
        let radius_xyz = Vector3::new(self.radius, self.radius, self.radius);
        BoundingBox::from_corners(self.centre + radius_xyz, self.centre - radius_xyz)
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

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

    #[quickcheck]
    fn all_points_on_sphere_are_in_bounding_box(
        sphere_centre: Point3<f64>,
        radius_vector: Vector3<f64>,
    ) -> bool {
        let target_sphere = Sphere::new(
            sphere_centre,
            radius_vector.norm(),
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let bounding_box = target_sphere.bounding_box();
        bounding_box.contains_point(sphere_centre + radius_vector)
    }
}

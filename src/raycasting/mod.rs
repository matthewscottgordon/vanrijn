use nalgebra::{Point3, RealField, Vector3};

use super::materials::Material;

use std::sync::Arc;


pub mod sphere;
pub use sphere::Sphere;

pub mod plane;
pub use plane::Plane;

pub mod triangle;
pub use triangle::Triangle;

#[derive(Clone, Debug)]
pub struct Ray<T: RealField> {
    pub origin: Point3<T>,
    pub direction: Vector3<T>,
}

impl<T: RealField> Ray<T> {
    pub fn new(origin: Point3<T>, direction: Vector3<T>) -> Ray<T> {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn point_at(&self, t: T) -> Point3<T> {
        self.origin + self.direction * t
    }

    pub fn bias(&self, amount: T) -> Ray<T> {
        Ray::new(self.origin + self.direction * amount, self.direction)
    }
}

#[derive(Debug)]
pub struct IntersectionInfo<T: RealField> {
    pub distance: T,
    pub location: Point3<T>,
    pub normal: Vector3<T>,
    pub tangent: Vector3<T>,
    pub cotangent: Vector3<T>,
    pub retro: Vector3<T>,
    pub material: Arc<dyn Material<T>>,
}

pub trait Intersect<T: RealField>: Send + Sync {
    /// Test if the ray intersects the object, and return information about the object and intersection.
    fn intersect<'a>(&'a self, ray: &Ray<T>) -> Option<IntersectionInfo<T>>;
}

pub trait IntersectP<T: RealField>: Send + Sync {
    /// Test if the ray intersects the object, without calculating any extra information.
    fn intersect(&self, ray: &Ray<T>) -> bool;
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;
    use quickcheck::{Arbitrary, Gen};
    impl<T: Arbitrary + RealField> Arbitrary for Ray<T> {
        fn arbitrary<G: Gen>(g: &mut G) -> Ray<T> {
            let origin = <Point3<T> as Arbitrary>::arbitrary(g);
            let direction = <Vector3<T> as Arbitrary>::arbitrary(g);
            return Ray::new(origin, direction);
        }
    }

    #[quickcheck]
    fn t0_is_origin(ray: Ray<f64>) -> bool {
        ray.point_at(0.0) == ray.origin
    }

    #[quickcheck]
    fn t1_is_origin_plus_direction(ray: Ray<f64>) -> bool {
        ray.point_at(1.0) == ray.origin + ray.direction
    }

    #[quickcheck]
    fn points_are_colinear(ray: Ray<f64>, t1: f64, t2: f64, t3: f64) -> bool {
        let p1 = ray.point_at(t1);
        let p2 = ray.point_at(t2);
        let p3 = ray.point_at(t3);
        let epsilon = [t1, t2, t3, ray.origin[0], ray.origin[1], ray.origin[2]]
            .iter()
            .fold(0.0, |a, &b| a.max(b.abs()))
            * std::f64::EPSILON
            * 256.0;
        (p2 - p1).cross(&(p3 - p2)).norm() < epsilon
    }

    #[quickcheck]
    fn t_is_distance(ray: Ray<f64>, t: f64) -> bool {
        (ray.point_at(t) - ray.origin).norm() - t.abs() < 0.0000000001
    }
}

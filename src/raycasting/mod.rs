use nalgebra::{convert, Point3, RealField, Vector3};

use super::materials::Material;

use std::sync::Arc;


pub mod sphere;
pub use sphere::Sphere;

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
    fn intersect<'a>(&'a self, ray: &Ray<T>) -> Option<IntersectionInfo<T>>;
}



pub struct Plane<T: RealField> {
    normal: Vector3<T>,
    tangent: Vector3<T>,
    cotangent: Vector3<T>,
    distance_from_origin: T,
    material: Arc<dyn Material<T>>,
}

impl<T: RealField> Plane<T> {
    pub fn new(
        normal: Vector3<T>,
        distance_from_origin: T,
        material: Arc<dyn Material<T>>,
    ) -> Plane<T> {
        normal.normalize();
        let mut axis_closest_to_tangent = Vector3::zeros();
        axis_closest_to_tangent[normal.iamin()] = T::one();
        let cotangent = normal.cross(&axis_closest_to_tangent).normalize();
        let tangent = normal.cross(&cotangent);
        Plane {
            normal,
            tangent,
            cotangent,
            distance_from_origin,
            material,
        }
    }
}

impl<T: RealField> Intersect<T> for Plane<T> {
    fn intersect<'a>(&'a self, ray: &Ray<T>) -> Option<IntersectionInfo<T>> {
        let ray_direction_dot_plane_normal = ray.direction.dot(&self.normal);
        let point_on_plane = self.normal * self.distance_from_origin;
        let point_on_plane_minus_ray_origin_dot_normal =
            (point_on_plane - ray.origin.coords).dot(&self.normal);
        if ray_direction_dot_plane_normal == convert(0.0) {
            //Ray is parallel to plane
            if point_on_plane_minus_ray_origin_dot_normal != convert(0.0) {
                //Ray is not in plane
                return None;
            }
        }
        let t = point_on_plane_minus_ray_origin_dot_normal / ray_direction_dot_plane_normal;
        if t < convert(0.0) {
            return None;
        }
        Some(IntersectionInfo {
            distance: t,
            location: ray.point_at(t),
            normal: self.normal,
            tangent: self.tangent,
            cotangent: self.cotangent,
            retro: -ray.direction,
            material: Arc::clone(&self.material),
        })
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    macro_rules! assert_matches {
        ($expression:expr, $($pattern:tt)+) => {
            match $expression {
                $($pattern)+ => (),
                ref e => panic!("assertion failed: `{:?}` does not match `{}`", e,
                                stringify!($($pattern)+)),
            }
        }
    }

    use super::*;
    use crate::materials::LambertianMaterial;
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

    #[test]
    fn ray_intersects_plane() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(-1.0, 0.0, 1.0));
        let p = Plane::new(
            Vector3::new(1.0, 0.0, 0.0),
            -5.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        assert_matches!(p.intersect(&r), Some(_));
    }

    #[test]
    fn ray_does_not_intersect_plane() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(1.0, 0.0, 1.0));
        let p = Plane::new(
            Vector3::new(1.0, 0.0, 0.0),
            -5.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        assert_matches!(p.intersect(&r), None);
    }

    #[test]
    fn intersection_point_is_on_plane() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(-1.0, 0.0, 1.0));
        let p = Plane::new(
            Vector3::new(1.0, 0.0, 0.0),
            -5.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        match p.intersect(&r) {
            Some(IntersectionInfo {
                distance: _,
                location,
                normal: _,
                tangent: _,
                cotangent: _,
                retro: _,
                material: _,
            }) => assert!((location.x - (-5.0f64)).abs() < 0.0000000001),
            None => panic!(),
        }
    }
}

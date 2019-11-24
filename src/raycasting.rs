use nalgebra::{convert, RealField, Vector3};

use super::materials::Material;

use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Ray<T: RealField> {
    origin: Vector3<T>,
    direction: Vector3<T>,
}

impl<T: RealField> Ray<T> {
    pub fn new(origin: Vector3<T>, direction: Vector3<T>) -> Ray<T> {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn point_at(&self, t: T) -> Vector3<T> {
        return self.origin + self.direction * t;
    }

    pub fn bias(&self, amount: T) -> Ray<T> {
        Ray::new(self.origin + self.direction * amount, self.direction)
    }
}

#[derive(Debug)]
pub struct IntersectionInfo<T: RealField> {
    pub distance: T,
    pub location: Vector3<T>,
    pub normal: Vector3<T>,
    pub retro: Vector3<T>,
    pub material: Rc<dyn Material<T>>,
}

pub trait Intersect<T: RealField> {
    fn intersect<'a>(&'a self, ray: &Ray<T>) -> Option<IntersectionInfo<T>>;
}

pub struct Sphere<T: RealField> {
    centre: Vector3<T>,
    radius: T,
    material: Rc<dyn Material<T>>,
}

impl<T: RealField> Sphere<T> {
    pub fn new(centre: Vector3<T>, radius: T, material: Rc<dyn Material<T>>) -> Sphere<T> {
        Sphere {
            centre,
            radius,
            material,
        }
    }
}

impl<T: RealField> Intersect<T> for Sphere<T> {
    fn intersect<'a>(&'a self, ray: &Ray<T>) -> Option<IntersectionInfo<T>> {
        let ray_origin_to_sphere_centre = self.centre - ray.origin;
        let radius_squared = self.radius * self.radius;
        let is_inside_sphere = ray_origin_to_sphere_centre.norm_squared() <= radius_squared;
        // t0/p0 is the point on the ray that's closest to the centre of the sphere
        let t0 = ray_origin_to_sphere_centre.dot(&ray.direction);
        if !is_inside_sphere && t0 < T::zero() {
            // Sphere is behind ray origin
            return None;
        }
        // Squared distance between ray origin and sphere centre
        let d0_squared = (ray.origin - self.centre).norm_squared();
        // Squared distance petween p0 and sphere centre
        let p0_dist_from_centre_squared = d0_squared - t0 * t0;
        if p0_dist_from_centre_squared > radius_squared {
            // Sphere is in front of ray but ray misses
            return None;
        }
        let delta = (radius_squared - p0_dist_from_centre_squared).sqrt();
        let distance = if is_inside_sphere {
            // radius origin is inside sphere
            t0 + delta
        } else {
            t0 - delta
        };
        let location = ray.point_at(distance);
        let normal = (location - self.centre).normalize();
        let retro = -ray.direction;
        Some(IntersectionInfo {
            distance,
            location,
            normal,
            retro,
            material: Rc::clone(&self.material),
        })
    }
}

pub struct Plane<T: RealField> {
    normal: Vector3<T>,
    distance_from_origin: T,
    material: Rc<dyn Material<T>>,
}

impl<T: RealField> Plane<T> {
    pub fn new(
        normal: Vector3<T>,
        distance_from_origin: T,
        material: Rc<dyn Material<T>>,
    ) -> Plane<T> {
        normal.normalize();
        Plane {
            normal,
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
            (point_on_plane - ray.origin).dot(&self.normal);
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
            retro: -ray.direction,
            material: Rc::clone(&self.material),
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
            let origin = <Vector3<T> as Arbitrary>::arbitrary(g);
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
    fn ray_intersects_sphere() {
        let r = Ray::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Vector3::new(1.5, 1.5, 15.0),
            5.0,
            Rc::new(LambertianMaterial::new_dummy()),
        );
        assert_matches!(s.intersect(&r), Some(_));
    }

    #[test]
    fn ray_does_not_intersect_sphere_when_sphere_is_in_front() {
        let r = Ray::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Vector3::new(-5.0, 1.5, 15.0),
            5.0,
            Rc::new(LambertianMaterial::new_dummy()),
        );
        assert_matches!(s.intersect(&r), None);
    }

    #[test]
    fn ray_does_not_intersect_sphere_when_sphere_is_behind() {
        let r = Ray::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Vector3::new(1.5, 1.5, -15.0),
            5.0,
            Rc::new(LambertianMaterial::new_dummy()),
        );
        assert_matches!(s.intersect(&r), None);
    }

    #[test]
    fn ray_intersects_sphere_when_origin_is_inside() {
        let r = Ray::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Vector3::new(1.5, 1.5, 2.0),
            5.0,
            Rc::new(LambertianMaterial::new_dummy()),
        );
        assert_matches!(s.intersect(&r), Some(_));
    }

    #[test]
    fn ray_intersects_plane() {
        let r = Ray::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(-1.0, 0.0, 1.0));
        let p = Plane::new(
            Vector3::new(1.0, 0.0, 0.0),
            -5.0,
            Rc::new(LambertianMaterial::new_dummy()),
        );
        assert_matches!(p.intersect(&r), Some(_));
    }

    #[test]
    fn ray_does_not_intersect_plane() {
        let r = Ray::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(1.0, 0.0, 1.0));
        let p = Plane::new(
            Vector3::new(1.0, 0.0, 0.0),
            -5.0,
            Rc::new(LambertianMaterial::new_dummy()),
        );
        assert_matches!(p.intersect(&r), None);
    }

    #[test]
    fn intersection_point_is_on_plane() {
        let r = Ray::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(-1.0, 0.0, 1.0));
        let p = Plane::new(
            Vector3::new(1.0, 0.0, 0.0),
            -5.0,
            Rc::new(LambertianMaterial::new_dummy()),
        );
        match p.intersect(&r) {
            Some(IntersectionInfo {
                distance: _,
                location,
                normal: _,
                retro: _,
                material: _,
            }) => assert!((location.x - (-5.0f64)).abs() < 0.0000000001),
            None => panic!(),
        }
    }
}

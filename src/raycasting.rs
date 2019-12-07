use nalgebra::{convert, Point3, RealField, Vector3};

use super::materials::Material;

use std::rc::Rc;

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
    pub material: Rc<dyn Material<T>>,
}

pub trait Intersect<T: RealField> {
    fn intersect<'a>(&'a self, ray: &Ray<T>) -> Option<IntersectionInfo<T>>;
}

pub struct Sphere<T: RealField> {
    centre: Point3<T>,
    radius: T,
    material: Rc<dyn Material<T>>,
}

impl<T: RealField> Sphere<T> {
    pub fn new(centre: Point3<T>, radius: T, material: Rc<dyn Material<T>>) -> Sphere<T> {
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
                    material: Rc::clone(&self.material),
                })
            }
        }
    }
}

pub struct Plane<T: RealField> {
    normal: Vector3<T>,
    tangent: Vector3<T>,
    cotangent: Vector3<T>,
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
    use quickcheck::{Arbitrary, Gen, TestResult};
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
    fn ray_intersects_sphere() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Point3::new(1.5, 1.5, 15.0),
            5.0,
            Rc::new(LambertianMaterial::new_dummy()),
        );
        assert_matches!(s.intersect(&r), Some(_));
    }

    #[test]
    fn ray_does_not_intersect_sphere_when_sphere_is_in_front() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Point3::new(-5.0, 1.5, 15.0),
            5.0,
            Rc::new(LambertianMaterial::new_dummy()),
        );
        assert_matches!(s.intersect(&r), None);
    }

    #[test]
    fn ray_does_not_intersect_sphere_when_sphere_is_behind() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Point3::new(1.5, 1.5, -15.0),
            5.0,
            Rc::new(LambertianMaterial::new_dummy()),
        );
        assert_matches!(s.intersect(&r), None);
    }

    #[test]
    fn ray_intersects_sphere_when_origin_is_inside() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Point3::new(1.5, 1.5, 2.0),
            5.0,
            Rc::new(LambertianMaterial::new_dummy()),
        );
        assert_matches!(s.intersect(&r), Some(_));
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
            Rc::new(LambertianMaterial::new_dummy()),
        );
        let ray = Ray::new(ray_origin, sphere_centre - ray_origin);
        let info = sphere.intersect(&ray).unwrap();
        let distance_to_centre = (sphere_centre - ray.origin).norm();
        TestResult::from_bool(
            (distance_to_centre - (info.distance + sphere.radius)).abs() < 0.00001,
        )
    }

    #[test]
    fn ray_intersects_plane() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(-1.0, 0.0, 1.0));
        let p = Plane::new(
            Vector3::new(1.0, 0.0, 0.0),
            -5.0,
            Rc::new(LambertianMaterial::new_dummy()),
        );
        assert_matches!(p.intersect(&r), Some(_));
    }

    #[test]
    fn ray_does_not_intersect_plane() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(1.0, 0.0, 1.0));
        let p = Plane::new(
            Vector3::new(1.0, 0.0, 0.0),
            -5.0,
            Rc::new(LambertianMaterial::new_dummy()),
        );
        assert_matches!(p.intersect(&r), None);
    }

    #[test]
    fn intersection_point_is_on_plane() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(-1.0, 0.0, 1.0));
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
                tangent: _,
                cotangent: _,
                retro: _,
                material: _,
            }) => assert!((location.x - (-5.0f64)).abs() < 0.0000000001),
            None => panic!(),
        }
    }
}

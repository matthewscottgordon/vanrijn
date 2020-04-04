use nalgebra::{convert, Affine3, Point3, Vector3};

use crate::materials::Material;
use crate::Real;

use super::{BoundingBox, HasBoundingBox, Intersect, IntersectionInfo, Primitive, Ray, Transform};

use std::sync::Arc;

pub struct Plane<T: Real> {
    normal: Vector3<T>,
    tangent: Vector3<T>,
    cotangent: Vector3<T>,
    distance_from_origin: T,
    material: Arc<dyn Material<T>>,
}

impl<T: Real> Plane<T> {
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

impl<T: Real> Transform<T> for Plane<T> {
    fn transform(&mut self, transformation: &Affine3<T>) -> &Self {
        self.normal = transformation.transform_vector(&self.normal).normalize();
        self.cotangent = transformation.transform_vector(&self.cotangent).normalize();
        self.cotangent = self.normal.cross(&self.cotangent);
        self.distance_from_origin = transformation
            .transform_vector(&(self.normal * self.distance_from_origin))
            .norm();
        self
    }
}

impl<T: Real> Intersect<T> for Plane<T> {
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

impl<T: Real> HasBoundingBox<T> for Plane<T> {
    fn bounding_box(&self) -> BoundingBox<T> {
        let p0 = Point3::from(self.normal * self.distance_from_origin);
        let f = |v: Vector3<T>| {
            let infinity: T = convert(std::f64::INFINITY);
            Vector3::from_iterator(v.iter().map(|&elem| {
                if elem == T::zero() {
                    T::zero()
                } else {
                    infinity
                }
            }))
        };
        let tangent = f(self.tangent);
        let cotangent = f(self.cotangent);
        let p1 = p0 + tangent;
        let p2 = p0 - tangent;
        let p3 = p0 + cotangent;
        let p4 = p0 - cotangent;
        BoundingBox::from_points(&[p1, p2, p3, p4])
    }
}

impl<T: Real> Primitive<T> for Plane<T> {}

#[cfg(test)]
mod tests {
    use nalgebra::Point3;

    use super::*;
    use crate::materials::LambertianMaterial;

    #[test]
    fn ray_intersects_plane() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(-1.0, 0.0, 1.0));
        let p = Plane::new(
            Vector3::new(1.0, 0.0, 0.0),
            -5.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        if let None = p.intersect(&r) {
            panic!("Intersection failed.");
        }
    }

    #[test]
    fn ray_does_not_intersect_plane() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vector3::new(1.0, 0.0, 1.0));
        let p = Plane::new(
            Vector3::new(1.0, 0.0, 0.0),
            -5.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        if let Some(_) = p.intersect(&r) {
            panic!("Intersection failed.");
        }
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

    #[test]
    fn bounding_box_is_correct_for_yz_plane() {
        let target = Plane::new(
            Vector3::new(1.0, 0.0, 0.0),
            2.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let bb = target.bounding_box();
        assert!(!bb.contains_point(Point3::new(1.0, 2.0, 3.0)));
        assert!(bb.contains_point(Point3::new(2.0, 2.0, 3.0)));
        assert!(bb.contains_point(Point3::new(2.0, 2000.0, 3.0)));
        assert!(bb.contains_point(Point3::new(2.0, 0.0, 3.0)));
        assert!(bb.contains_point(Point3::new(2.0, -2000.0, 3.0)));
        assert!(bb.contains_point(Point3::new(2.0, 2.0, 3000.0)));
        assert!(bb.contains_point(Point3::new(2.0, 2.0, 0.0)));
        assert!(bb.contains_point(Point3::new(2.0, 2.0, -3000.0)));
        assert!(!bb.contains_point(Point3::new(3.0, 2.0, 3.0)));
    }

    #[test]
    fn bounding_box_is_correct_for_yz_plane_with_negative_normal() {
        let target = Plane::new(
            Vector3::new(-1.0, 0.0, 0.0),
            2.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let bb = target.bounding_box();
        assert!(!bb.contains_point(Point3::new(1.0, 2.0, 3.0)));
        assert!(bb.contains_point(Point3::new(-2.0, 2.0, 3.0)));
        assert!(bb.contains_point(Point3::new(-2.0, 2000.0, 3.0)));
        assert!(bb.contains_point(Point3::new(-2.0, 0.0, 3.0)));
        assert!(bb.contains_point(Point3::new(-2.0, -2000.0, 3.0)));
        assert!(bb.contains_point(Point3::new(-2.0, 2.0, 3000.0)));
        assert!(bb.contains_point(Point3::new(-2.0, 2.0, 0.0)));
        assert!(bb.contains_point(Point3::new(-2.0, 2.0, -3000.0)));
        assert!(!bb.contains_point(Point3::new(-3.0, 2.0, 3.0)));
    }

    #[test]
    fn bounding_box_is_correct_for_xz_plane() {
        let target = Plane::new(
            Vector3::new(0.0, 1.0, 0.0),
            2.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let bb = target.bounding_box();
        assert!(!bb.contains_point(Point3::new(1.0, 1.0, 3.0)));
        assert!(bb.contains_point(Point3::new(1.0, 2.0, 3.0)));
        assert!(bb.contains_point(Point3::new(1000.0, 2.0, 3.0)));
        assert!(bb.contains_point(Point3::new(0.0, 2.0, 3.0)));
        assert!(bb.contains_point(Point3::new(-1000.0, 2.0, 3.0)));
        assert!(bb.contains_point(Point3::new(1.0, 2.0, 3000.0)));
        assert!(bb.contains_point(Point3::new(1.0, 2.0, 0.0)));
        assert!(bb.contains_point(Point3::new(1.0, 2.0, -3000.0)));
        assert!(!bb.contains_point(Point3::new(1.0, 3.0, 3.0)));
    }

    #[test]
    fn bounding_box_is_correct_for_xz_plane_with_negative_normal() {
        let target = Plane::new(
            Vector3::new(0.0, -1.0, 0.0),
            2.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let bb = target.bounding_box();
        assert!(!bb.contains_point(Point3::new(1.0, -1.0, 3.0)));
        assert!(bb.contains_point(Point3::new(1.0, -2.0, 3.0)));
        assert!(bb.contains_point(Point3::new(1000.0, -2.0, 3.0)));
        assert!(bb.contains_point(Point3::new(0.0, -2.0, 3.0)));
        assert!(bb.contains_point(Point3::new(-1000.0, -2.0, 3.0)));
        assert!(bb.contains_point(Point3::new(1.0, -2.0, 3000.0)));
        assert!(bb.contains_point(Point3::new(1.0, -2.0, 0.0)));
        assert!(bb.contains_point(Point3::new(1.0, -2.0, -3000.0)));
        assert!(!bb.contains_point(Point3::new(1.0, 3.0, 3.0)));
    }

    #[test]
    fn bounding_box_is_correct_for_xy_plane() {
        let target = Plane::new(
            Vector3::new(0.0, 0.0, 1.0),
            2.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let bb = target.bounding_box();
        assert!(!bb.contains_point(Point3::new(1.0, 2.0, 1.0)));
        assert!(bb.contains_point(Point3::new(1.0, 2.0, 2.0)));
        assert!(bb.contains_point(Point3::new(1.0, 2000.0, 2.0)));
        assert!(bb.contains_point(Point3::new(1.0, 0.0, 2.0)));
        assert!(bb.contains_point(Point3::new(1.0, -2000.0, 2.0)));
        assert!(bb.contains_point(Point3::new(2000.0, 2.0, 2.0)));
        assert!(bb.contains_point(Point3::new(0.0, 2.0, 2.0)));
        assert!(bb.contains_point(Point3::new(-2000.0, 2.0, 2.0)));
        assert!(!bb.contains_point(Point3::new(3.0, 2.0, 3.0)));
    }

    #[test]
    fn bounding_box_is_correct_for_xy_plane_with_negative_normal() {
        let target = Plane::new(
            Vector3::new(0.0, 0.0, -1.0),
            -2.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let bb = target.bounding_box();
        assert!(!bb.contains_point(Point3::new(1.0, 2.0, 1.0)));
        assert!(bb.contains_point(Point3::new(1.0, 2.0, 2.0)));
        assert!(bb.contains_point(Point3::new(1.0, 2000.0, 2.0)));
        assert!(bb.contains_point(Point3::new(1.0, 0.0, 2.0)));
        assert!(bb.contains_point(Point3::new(1.0, -2000.0, 2.0)));
        assert!(bb.contains_point(Point3::new(2000.0, 2.0, 2.0)));
        assert!(bb.contains_point(Point3::new(0.0, 2.0, 2.0)));
        assert!(bb.contains_point(Point3::new(-2000.0, 2.0, 2.0)));
        assert!(!bb.contains_point(Point3::new(3.0, 2.0, 3.0)));
    }

    #[test]
    fn bounding_box_is_infinite_when_normal_is_not_aligned_with_axis() {
        let target = Plane::new(
            Vector3::new(0.1, 0.0, -1.0),
            -2.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let bb = target.bounding_box();
        assert!(bb.contains_point(Point3::new(1.0, 2.0, 1.0)));
        assert!(bb.contains_point(Point3::new(1.0, 2.0, 2.0)));
        assert!(bb.contains_point(Point3::new(1.0, 2000.0, 2.0)));
        assert!(bb.contains_point(Point3::new(1.0, 0.0, 2.0)));
        assert!(bb.contains_point(Point3::new(1.0, -2000.0, 2.0)));
        assert!(bb.contains_point(Point3::new(2000.0, 2.0, 2.0)));
        assert!(bb.contains_point(Point3::new(0.0, 2.0, 2.0)));
        assert!(bb.contains_point(Point3::new(-2000.0, 2.0, 2.0)));
        assert!(bb.contains_point(Point3::new(3.0, 2.0, 3.0)));
    }
}

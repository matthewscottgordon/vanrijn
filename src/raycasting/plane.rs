use crate::materials::Material;
use crate::math::Vec3;

use super::{BoundingBox, HasBoundingBox, Intersect, IntersectionInfo, Primitive, Ray};

use std::sync::Arc;

#[derive(Clone)]
pub struct Plane {
    normal: Vec3,
    tangent: Vec3,
    cotangent: Vec3,
    distance_from_origin: f64,
    material: Arc<dyn Material>,
}

impl Plane {
    pub fn new(normal: Vec3, distance_from_origin: f64, material: Arc<dyn Material>) -> Plane {
        let normal = normal.normalize();
        let mut axis_closest_to_tangent = Vec3::zeros();
        axis_closest_to_tangent[normal.smallest_coord()] = 1.0;
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

/*impl Transform for Plane {
    fn transform(&self, transformation: &Affine3<f64>) -> Self {
        Plane {
            normal: transformation.transform_vector(&self.normal).normalize(),
            cotangent: transformation.transform_vector(&self.cotangent).normalize(),
            tangent: self.normal.cross(&self.cotangent),
            distance_from_origin: transformation
                .transform_vector(&(self.normal * self.distance_from_origin))
                .norm(),
            material: Arc::clone(&self.material),
        }
    }
}*/

impl Intersect for Plane {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo> {
        let ray_direction_dot_plane_normal = ray.direction.dot(&self.normal);
        let point_on_plane = self.normal * self.distance_from_origin;
        let point_on_plane_minus_ray_origin_dot_normal =
            (point_on_plane - ray.origin).dot(&self.normal);
        if ray_direction_dot_plane_normal == 0.0 {
            //Ray is parallel to plane
            if point_on_plane_minus_ray_origin_dot_normal != 0.0 {
                //Ray is not in plane
                return None;
            }
        }
        let t = point_on_plane_minus_ray_origin_dot_normal / ray_direction_dot_plane_normal;
        if t < 0.0 {
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

impl HasBoundingBox for Plane {
    fn bounding_box(&self) -> BoundingBox {
        let p0 = self.normal * self.distance_from_origin;
        let f = |v: Vec3| {
            Vec3::new(
                if v.x() == 0.0 {
                    0.0
                } else {
                    std::f64::INFINITY
                },
                if v.y() == 0.0 {
                    0.0
                } else {
                    std::f64::INFINITY
                },
                if v.z() == 0.0 {
                    0.0
                } else {
                    std::f64::INFINITY
                },
            )
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

impl Primitive for Plane {}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::materials::LambertianMaterial;
    use crate::math::Vec3;

    #[test]
    fn ray_intersects_plane() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, 0.0, 1.0));
        let p = Plane::new(
            Vec3::new(1.0, 0.0, 0.0),
            -5.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        if let None = p.intersect(&r) {
            panic!("Intersection failed.");
        }
    }

    #[test]
    fn ray_does_not_intersect_plane() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 0.0, 1.0));
        let p = Plane::new(
            Vec3::new(1.0, 0.0, 0.0),
            -5.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        if let Some(_) = p.intersect(&r) {
            panic!("Intersection failed.");
        }
    }

    #[test]
    fn intersection_point_is_on_plane() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, 0.0, 1.0));
        let p = Plane::new(
            Vec3::new(1.0, 0.0, 0.0),
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
            }) => assert!((location.x() - (-5.0f64)).abs() < 0.0000000001),
            None => panic!(),
        }
    }

    #[test]
    fn bounding_box_is_correct_for_yz_plane() {
        let target = Plane::new(
            Vec3::new(1.0, 0.0, 0.0),
            2.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let bb = target.bounding_box();
        assert!(!bb.contains_point(Vec3::new(1.0, 2.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(2.0, 2.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(2.0, 2000.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(2.0, 0.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(2.0, -2000.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(2.0, 2.0, 3000.0)));
        assert!(bb.contains_point(Vec3::new(2.0, 2.0, 0.0)));
        assert!(bb.contains_point(Vec3::new(2.0, 2.0, -3000.0)));
        assert!(!bb.contains_point(Vec3::new(3.0, 2.0, 3.0)));
    }

    #[test]
    fn bounding_box_is_correct_for_yz_plane_with_negative_normal() {
        let target = Plane::new(
            Vec3::new(-1.0, 0.0, 0.0),
            2.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let bb = target.bounding_box();
        assert!(!bb.contains_point(Vec3::new(1.0, 2.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(-2.0, 2.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(-2.0, 2000.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(-2.0, 0.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(-2.0, -2000.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(-2.0, 2.0, 3000.0)));
        assert!(bb.contains_point(Vec3::new(-2.0, 2.0, 0.0)));
        assert!(bb.contains_point(Vec3::new(-2.0, 2.0, -3000.0)));
        assert!(!bb.contains_point(Vec3::new(-3.0, 2.0, 3.0)));
    }

    #[test]
    fn bounding_box_is_correct_for_xz_plane() {
        let target = Plane::new(
            Vec3::new(0.0, 1.0, 0.0),
            2.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let bb = target.bounding_box();
        assert!(!bb.contains_point(Vec3::new(1.0, 1.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(1.0, 2.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(1000.0, 2.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(0.0, 2.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(-1000.0, 2.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(1.0, 2.0, 3000.0)));
        assert!(bb.contains_point(Vec3::new(1.0, 2.0, 0.0)));
        assert!(bb.contains_point(Vec3::new(1.0, 2.0, -3000.0)));
        assert!(!bb.contains_point(Vec3::new(1.0, 3.0, 3.0)));
    }

    #[test]
    fn bounding_box_is_correct_for_xz_plane_with_negative_normal() {
        let target = Plane::new(
            Vec3::new(0.0, -1.0, 0.0),
            2.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let bb = target.bounding_box();
        assert!(!bb.contains_point(Vec3::new(1.0, -1.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(1.0, -2.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(1000.0, -2.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(0.0, -2.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(-1000.0, -2.0, 3.0)));
        assert!(bb.contains_point(Vec3::new(1.0, -2.0, 3000.0)));
        assert!(bb.contains_point(Vec3::new(1.0, -2.0, 0.0)));
        assert!(bb.contains_point(Vec3::new(1.0, -2.0, -3000.0)));
        assert!(!bb.contains_point(Vec3::new(1.0, 3.0, 3.0)));
    }

    #[test]
    fn bounding_box_is_correct_for_xy_plane() {
        let target = Plane::new(
            Vec3::new(0.0, 0.0, 1.0),
            2.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let bb = target.bounding_box();
        assert!(!bb.contains_point(Vec3::new(1.0, 2.0, 1.0)));
        assert!(bb.contains_point(Vec3::new(1.0, 2.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(1.0, 2000.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(1.0, 0.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(1.0, -2000.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(2000.0, 2.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(0.0, 2.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(-2000.0, 2.0, 2.0)));
        assert!(!bb.contains_point(Vec3::new(3.0, 2.0, 3.0)));
    }

    #[test]
    fn bounding_box_is_correct_for_xy_plane_with_negative_normal() {
        let target = Plane::new(
            Vec3::new(0.0, 0.0, -1.0),
            -2.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let bb = target.bounding_box();
        assert!(!bb.contains_point(Vec3::new(1.0, 2.0, 1.0)));
        assert!(bb.contains_point(Vec3::new(1.0, 2.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(1.0, 2000.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(1.0, 0.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(1.0, -2000.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(2000.0, 2.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(0.0, 2.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(-2000.0, 2.0, 2.0)));
        assert!(!bb.contains_point(Vec3::new(3.0, 2.0, 3.0)));
    }

    #[test]
    fn bounding_box_is_infinite_when_normal_is_not_aligned_with_axis() {
        let target = Plane::new(
            Vec3::new(0.1, 0.0, -1.0),
            -2.0,
            Arc::new(LambertianMaterial::new_dummy()),
        );
        let bb = target.bounding_box();
        assert!(bb.contains_point(Vec3::new(1.0, 2.0, 1.0)));
        assert!(bb.contains_point(Vec3::new(1.0, 2.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(1.0, 2000.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(1.0, 0.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(1.0, -2000.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(2000.0, 2.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(0.0, 2.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(-2000.0, 2.0, 2.0)));
        assert!(bb.contains_point(Vec3::new(3.0, 2.0, 3.0)));
    }
}

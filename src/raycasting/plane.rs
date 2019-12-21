use nalgebra::{convert, RealField, Vector3};

use crate::materials::Material;

use super::{Intersect, IntersectionInfo, Ray};

use std::sync::Arc;

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
}

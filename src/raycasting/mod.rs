use crate::math::Vec3;

use super::materials::Material;

use std::sync::Arc;

pub mod sphere;
pub use sphere::Sphere;

pub mod plane;
pub use plane::Plane;

pub mod triangle;
pub use triangle::Triangle;

pub mod axis_aligned_bounding_box;
pub use axis_aligned_bounding_box::BoundingBox;

pub mod bounding_volume_hierarchy;
pub use bounding_volume_hierarchy::BoundingVolumeHierarchy;

pub mod vec_aggregate;

/// A ray, consisting or a start point and direction
///
/// This is the basic ray struct used to define things like a line-of-sight
/// going out from the camera of a reflection from a surface.
#[derive(Clone, Debug)]
pub struct Ray {
    /// The start point of the ray
    pub origin: Vec3,

    /// The direction the ray goes in.
    ///
    /// This vector should always be kept normalized
    pub direction: Vec3,
}

impl Ray {
    /// Create a new ray
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Return the point on the ray that is `t` units from the start
    pub fn point_at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    /// Create a new ray by moving the original ray along it's direction by `amount`
    ///
    /// `amount` is normally a very small number. This function is useful for ensuring
    /// that rounding-errors don;t cause a reflection ray doesn't intersect with the point
    /// it's reflected from.
    pub fn bias(&self, amount: f64) -> Ray {
        Ray::new(self.origin + self.direction * amount, self.direction)
    }
}

/// Information about a ray-primitive intersection.
///
/// This struct is returned by [intersect()](Intersect::intersect) and contatins all the
/// information needed to evaluate the rendering function for that intersection.
#[derive(Debug)]
pub struct IntersectionInfo {
    /// The distance between the ray origin and the intersection point
    pub distance: f64,

    /// The intersection point
    pub location: Vec3,

    /// The surface normal at the intersection point
    pub normal: Vec3,

    /// The surface tangent at the intersection point
    ///
    /// Which surface tangent direction returned is dependent on the [Primitive](Primitive)
    /// but should generally be smooth over any given surface
    pub tangent: Vec3,

    /// Another surface tangent, perpendicular to `tangent`
    ///
    /// The cross product or `normal` and `tangent`
    pub cotangent: Vec3,

    /// The direction from the intersection point back towards the ray
    ///
    /// Equal to `-ray.direction`
    pub retro: Vec3,

    /// The [Material](crate::materials::Material) which describes the optical
    /// properties of the intersected surface
    pub material: Arc<dyn Material>,
}

/// A geometric object that has a [Material](crate::materials::Material) and can be
/// intersected with a [Ray](Ray)
pub trait Intersect: Send + Sync {
    /// Test if the ray intersects the object, and return information about the object and intersection.
    fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo>;
}

/// A geometric object that can be intersected with a ray
///
/// This is useful for objects that don't have materials (such as [BoundingBox](BoundingBox))
/// and as a (possibly) faster alternative to [Intersect](Intersect) when only a simple
/// intersection test is needed.
pub trait IntersectP: Send + Sync {
    /// Test if the ray intersects the object, without calculating any extra information.
    fn intersect(&self, ray: &Ray) -> bool;
}

/// Any geometric object for which a [BoundingBox](BoundingBox) can be calculated
pub trait HasBoundingBox: Send + Sync {
    /// The axis-aligned bounding box of the object
    ///
    /// The object must fit entirely inside this box.
    fn bounding_box(&self) -> BoundingBox;
}

/// Any geometric object which can have an affine transformation applied to it
///
/// Used for moving, rotating or scaling primitives
/*pub trait Transform {
    /// Create a new object by applying the transformation to this object.
    fn transform(&self, transformation: &Affine3<f64>) -> Self;
}*/

/// A basic geometric primitive such as a sphere or a triangle
pub trait Primitive: Intersect + HasBoundingBox {
    // / Create a new object by applying the transformation to this object.
    //fn transform(&self, transformation: &Affine3) -> dyn Primitive;
}

/// Either a primitive or a collection of primitives
pub trait Aggregate: Intersect + HasBoundingBox {}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;
    use quickcheck::{Arbitrary, Gen};
    impl Arbitrary for Ray {
        fn arbitrary<G: Gen>(g: &mut G) -> Ray {
            let origin = <Vec3 as Arbitrary>::arbitrary(g);
            let direction = <Vec3 as Arbitrary>::arbitrary(g);
            return Ray::new(origin, direction);
        }
    }

    #[quickcheck]
    fn t0_is_origin(ray: Ray) -> bool {
        ray.point_at(0.0) == ray.origin
    }

    #[quickcheck]
    fn t1_is_origin_plus_direction(ray: Ray) -> bool {
        ray.point_at(1.0) == ray.origin + ray.direction
    }

    #[quickcheck]
    fn points_are_colinear(ray: Ray, t1: f64, t2: f64, t3: f64) -> bool {
        let p1 = ray.point_at(t1);
        let p2 = ray.point_at(t2);
        let p3 = ray.point_at(t3);
        let epsilon = [t1, t2, t3, ray.origin[0], ray.origin[1], ray.origin[2]]
            .iter()
            .fold(0.0f64, |a, &b| a.max(b.abs()))
            * std::f64::EPSILON
            * 256.0f64;
        (p2 - p1).cross(&(p3 - p2)).norm() < epsilon
    }

    #[quickcheck]
    fn t_is_distance(ray: Ray, t: f64) -> bool {
        (ray.point_at(t) - ray.origin).norm() - t.abs() < 0.0000000001
    }
}

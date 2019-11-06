use nalgebra::{RealField, Vector3};

#[derive(Clone, Debug)]
struct Ray<T: RealField> {
    origin: Vector3<T>,
    direction: Vector3<T>,
}

impl<T: RealField> Ray<T> {
    fn new(origin: Vector3<T>, direction: Vector3<T>) -> Ray<T> {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }
}

impl<T: RealField> Ray<T> {
    fn point_at(&self, t: T) -> Vector3<T> {
        return self.origin + self.direction * t;
    }
}

#[cfg(test)]
extern crate quickcheck;
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
mod tests {
    use super::*;
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
            .fold(0.0, |a, &b| a.max(b.abs())) * std::f64::EPSILON * 128.0;
        (p2 - p1).cross(&(p3 - p2)).norm() < epsilon
    }
}

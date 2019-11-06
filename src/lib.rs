use nalgebra::{RealField, Vector3};

#[derive(Clone,Debug)]
struct Ray<T: RealField> {
    origin: Vector3<T>,
    direction: Vector3<T>,
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
    use quickcheck::{Arbitrary,Gen};
    impl<T:Arbitrary+RealField> Arbitrary for Ray<T> {
        fn arbitrary<G:Gen>(g: &mut G) -> Ray<T> {
            let origin = <Vector3<T> as Arbitrary>::arbitrary(g);
            let direction = <Vector3<T> as Arbitrary>::arbitrary(g);
            return Ray{origin, direction}
        }
    }

    #[quickcheck]
    fn test_t0_is_origin(ray:Ray<f64>) -> bool {
        ray.point_at(0.0) == ray.origin
    }
}

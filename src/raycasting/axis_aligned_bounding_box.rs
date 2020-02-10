use crate::Real;

use crate::util::Interval;

use super::{IntersectP, Ray};

use itertools::izip;

pub use crate::util::axis_aligned_bounding_box::BoundingBox;

impl<T: Real> IntersectP<T> for BoundingBox<T> {
    fn intersect(&self, ray: &Ray<T>) -> bool {
        let mut t_interval_in_bounds = Interval::infinite();
        for (&ray_origin, &ray_direction, bounds) in
            izip!(ray.origin.iter(), ray.direction.iter(), self.bounds.iter())
        {
            t_interval_in_bounds = t_interval_in_bounds.intersection(Interval::new(
                (bounds.get_min() - ray_origin) / ray_direction,
                (bounds.get_max() - ray_origin) / ray_direction,
            ));
            if t_interval_in_bounds.is_empty() {
                return false;
            };
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    use nalgebra::{Point3, Vector3};

    fn wrap_value_in_interval(value: f64, interval: Interval<f64>) -> f64 {
        let distance_from_start = (value - interval.get_min()).abs();
        let range = interval.get_max() - interval.get_min();
        let multiple_of_range = distance_from_start / range;
        return interval.get_min() + multiple_of_range.fract() * range;
    }

    #[quickcheck]
    fn wrap_value_in_interval_produces_values_in_interval(v: f64, a: f64, b: f64) -> bool {
        let interval = Interval::new(a, b);
        interval.contains_value(wrap_value_in_interval(v, interval))
    }

    fn wrap_point_into_bounding_box(point: Point3<f64>, bounds: &BoundingBox<f64>) -> Point3<f64> {
        Point3::from(Vector3::from_iterator(
            point
                .iter()
                .zip(bounds.bounds.iter())
                .map(|(&value, &interval)| wrap_value_in_interval(value, interval)),
        ))
    }

    #[quickcheck]
    fn correctly_detects_intersections(
        ray_origin: Point3<f64>,
        corner1: Point3<f64>,
        corner2: Point3<f64>,
        random_point: Point3<f64>,
    ) -> bool {
        let bounds = BoundingBox::from_corners(corner1, corner2);
        let point_in_bounds = wrap_point_into_bounding_box(random_point, &bounds);
        let ray = Ray::new(ray_origin, point_in_bounds - ray_origin);
        bounds.intersect(&ray)
    }

    #[quickcheck]
    fn intersect_always_true_when_ray_origin_is_inside_bounds(
        ray_origin: Point3<f64>,
        corner1: Point3<f64>,
        corner2: Point3<f64>,
        random_point: Point3<f64>,
    ) -> TestResult {
        let bounds = BoundingBox::from_corners(corner1, corner2);
        let ray_origin = wrap_point_into_bounding_box(ray_origin, &bounds);
        let ray = Ray::new(ray_origin, ray_origin - random_point);
        TestResult::from_bool(bounds.intersect(&ray))
    }

    #[quickcheck]
    fn no_intersection_when_behind_ray(
        ray_origin: Point3<f64>,
        corner1: Point3<f64>,
        corner2: Point3<f64>,
        random_point: Point3<f64>,
    ) -> TestResult {
        let bounds = BoundingBox::from_corners(corner1, corner2);
        if bounds.contains_point(ray_origin) {
            return TestResult::discard();
        }
        let point_in_bounds = wrap_point_into_bounding_box(random_point, &bounds);
        let ray = Ray::new(ray_origin, ray_origin - point_in_bounds);
        TestResult::from_bool(bounds.intersect(&ray))
    }

    #[test]
    fn intersection_detected_when_ray_parallel_to_axis() {
        let target =
            BoundingBox::from_corners(Point3::new(1.0f64, 2.0, 3.0), Point3::new(4.0, 5.0, 6.0));
        let x_ray = Ray::new(Point3::new(0.0, 3.0, 4.0), Vector3::new(1.0, 0.0, 0.0));
        assert!(target.intersect(&x_ray));
        let y_ray = Ray::new(Point3::new(2.0, 0.0, 4.0), Vector3::new(0.0, 1.0, 0.0));
        assert!(target.intersect(&y_ray));
        let z_ray = Ray::new(Point3::new(2.0, 3.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        assert!(target.intersect(&z_ray));
    }

    #[test]
    fn intersection_missed_when_ray_parallel_to_axis() {
        let target =
            BoundingBox::from_corners(Point3::new(1.0f64, 2.0, 3.0), Point3::new(4.0, 5.0, 6.0));
        let x_ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        assert!(!target.intersect(&x_ray));
        let y_ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        assert!(!target.intersect(&y_ray));
        let z_ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        assert!(!target.intersect(&z_ray));
    }
}

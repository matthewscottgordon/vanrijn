use nalgebra::{convert, Point3, RealField};

use super::{IntersectP, Ray};

use itertools::izip;

#[derive(Debug, Clone, Copy)]
pub struct Interval<T: RealField> {
    pub min: T,
    pub max: T,
}

impl<T: RealField> Interval<T> {
    pub fn new(a: T, b: T) -> Self {
        if a > b {
            Interval { min: b, max: a }
        } else {
            Interval { min: a, max: b }
        }
    }

    pub fn infinite() -> Self {
        Interval {
            min: convert(std::f64::NEG_INFINITY),
            max: convert(std::f64::INFINITY),
        }
    }

    pub fn intersection(self, b: Self) -> Self {
        Interval {
            min: self.min.max(b.min),
            max: self.max.min(b.max),
        }
    }

    pub fn is_degenerate(self) -> bool {
        self.min == self.max
    }

    pub fn is_empty(self) -> bool {
        self.min > self.max
    }

    pub fn contains_value(&self, value: T) -> bool {
        value >= self.min && value <= self.max
    }
}

pub struct BoundingBox<T: RealField> {
    bounds: [Interval<T>; 3],
}

impl<T: RealField> BoundingBox<T> {
    pub fn from_corners(a: Point3<T>, b: Point3<T>) -> Self {
        let mut result = BoundingBox {
            bounds: [Interval::infinite(); 3],
        };
        for (bounds_elem, a_elem, b_elem) in izip!(result.bounds.iter_mut(), a.iter(), b.iter()) {
            *bounds_elem = Interval::new(*a_elem, *b_elem);
        }
        result
    }

    pub fn contains_point(&self, p: Point3<T>) -> bool {
        self.bounds
            .iter()
            .zip(p.iter())
            .all(|(interval, &value)| interval.contains_value(value))
    }
}

impl<T: RealField> IntersectP<T> for BoundingBox<T> {
    fn intersect(&self, ray: &Ray<T>) -> bool {
        let mut t_interval_in_bounds = Interval::infinite();
        for (&ray_origin, &ray_direction, bounds) in
            izip!(ray.origin.iter(), ray.direction.iter(), self.bounds.iter())
        {
            t_interval_in_bounds = t_interval_in_bounds.intersection(Interval::new(
                (bounds.min - ray_origin) / ray_direction,
                (bounds.max - ray_origin) / ray_direction,
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

    mod interval {
        use super::*;

        #[test]
        fn never_constructed_empty() {
            let target1 = Interval::new(5f64, 10f64);
            assert!(!target1.is_empty());
            let target2 = Interval::new(10f64, 5f64);
            assert!(!target2.is_empty());
            let target1 = Interval::new(5f64, -10f64);
            assert!(!target1.is_empty());
            let target2 = Interval::new(10f64, -5f64);
            assert!(!target2.is_empty());
            let target1 = Interval::new(-5f64, 10f64);
            assert!(!target1.is_empty());
            let target2 = Interval::new(-10f64, 5f64);
            assert!(!target2.is_empty());
            let target1 = Interval::new(-5f64, -10f64);
            assert!(!target1.is_empty());
            let target2 = Interval::new(-10f64, -5f64);
            assert!(!target2.is_empty());
        }

        #[test]
        fn empty_is_empty() {
            let target = Interval {
                min: 10f64,
                max: 5f64,
            };
            assert!(target.is_empty());
        }

        #[test]
        fn empty_is_empty_with_negative_values() {
            let target = Interval {
                min: -5f64,
                max: -10f64,
            };
            assert!(target.is_empty());
        }

        #[test]
        fn empty_is_empty_with_mixed_signs() {
            let target = Interval {
                min: 5f64,
                max: -10f64,
            };
            assert!(target.is_empty());
        }

        #[test]
        fn empty_is_not_degenerate() {
            let target = Interval {
                min: 10f64,
                max: 5f64,
            };
            assert!(!target.is_degenerate());
        }

        #[test]
        fn empty_is_not_degenerate_with_negative_values() {
            let target = Interval {
                min: -5f64,
                max: -10f64,
            };
            assert!(!target.is_degenerate());
        }

        #[test]
        fn empty_is_not_degenerate_with_mixed_signs() {
            let target = Interval {
                min: -5f64,
                max: 10f64,
            };
            assert!(!target.is_degenerate());
        }

        #[test]
        fn degenerate_is_degenerate() {
            let target = Interval {
                min: 5f64,
                max: 5f64,
            };
            assert!(target.is_degenerate());
        }

        #[test]
        fn degenerate_is_not_empty() {
            let target = Interval {
                min: 5f64,
                max: 5f64,
            };
            assert!(target.is_degenerate());
        }

        #[test]
        fn degenerate_is_degenerate_with_negative_value() {
            let target = Interval {
                min: -5f64,
                max: -5f64,
            };
            assert!(target.is_degenerate());
        }

        #[test]
        fn degenerate_is_not_empty_with_negative_value() {
            let target = Interval {
                min: -5f64,
                max: -5f64,
            };
            assert!(target.is_degenerate());
        }

        #[test]
        fn intersection_with_infinite_is_self() {
            let target = Interval::new(5f32, 10f32);
            let result = target.intersection(Interval::infinite());
            assert!(target.min == result.min);
            assert!(target.max == result.max);
        }
    }

    mod bounding_box {
        use super::*;

        use nalgebra::Vector3;

        use quickcheck::TestResult;
        use quickcheck_macros::quickcheck;

        #[test]
        fn from_corners_with_same_point_yields_degenerate_intervals() {
            let test_point = Point3::new(0f64, 1f64, 2f64);
            let target = BoundingBox::from_corners(test_point, test_point);
            assert!(target.bounds.iter().all(|e| e.is_degenerate()));
        }

        #[test]
        fn from_corners_yields_same_result_with_any_oposite_corners() {
            let corner_000 = Point3::new(0.0, 0.0, 0.0);
            let corner_001 = Point3::new(0.0, 0.0, 1.0);
            let corner_010 = Point3::new(0.0, 1.0, 0.0);
            let corner_011 = Point3::new(0.0, 1.0, 1.0);
            let corner_100 = Point3::new(1.0, 0.0, 0.0);
            let corner_101 = Point3::new(1.0, 0.0, 1.0);
            let corner_110 = Point3::new(1.0, 1.0, 0.0);
            let corner_111 = Point3::new(1.0, 1.0, 1.0);

            let test_inputs: Vec<(Point3<f64>, Point3<f64>)> = vec![
                (corner_000, corner_111),
                (corner_001, corner_110),
                (corner_010, corner_101),
                (corner_011, corner_100),
                (corner_100, corner_011),
                (corner_101, corner_010),
                (corner_110, corner_001),
                (corner_111, corner_000),
            ];
            for (a, b) in test_inputs {
                let target = BoundingBox::from_corners(a, b);
                assert!(target
                    .bounds
                    .iter()
                    .all(|bounds| bounds.min == 0.0 && bounds.max == 1.0));
            }
        }

        fn wrap_value_in_interval(value: f64, interval: Interval<f64>) -> f64 {
            let distance_from_start = (value - interval.min).abs();
            let range = interval.max - interval.min;
            let multiple_of_range = distance_from_start / range;
            return interval.min + multiple_of_range.fract() * range;
        }

        #[quickcheck]
        fn wrap_value_in_interval_produces_values_in_interval(v: f64, a: f64, b: f64) -> bool {
            let interval = Interval::new(a, b);
            interval.contains_value(wrap_value_in_interval(v, interval))
        }

        fn wrap_point_into_bounding_box(
            point: Point3<f64>,
            bounds: &BoundingBox<f64>,
        ) -> Point3<f64> {
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
            let target = BoundingBox::from_corners(
                Point3::new(1.0f64, 2.0, 3.0),
                Point3::new(4.0, 5.0, 6.0),
            );
            let x_ray = Ray::new(Point3::new(0.0, 3.0, 4.0), Vector3::new(1.0, 0.0, 0.0));
            assert!(target.intersect(&x_ray));
            let y_ray = Ray::new(Point3::new(2.0, 0.0, 4.0), Vector3::new(0.0, 1.0, 0.0));
            assert!(target.intersect(&y_ray));
            let z_ray = Ray::new(Point3::new(2.0, 3.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
            assert!(target.intersect(&z_ray));
        }

        #[test]
        fn intersection_missed_when_ray_parallel_to_axis() {
            let target = BoundingBox::from_corners(
                Point3::new(1.0f64, 2.0, 3.0),
                Point3::new(4.0, 5.0, 6.0),
            );
            let x_ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
            assert!(!target.intersect(&x_ray));
            let y_ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
            assert!(!target.intersect(&y_ray));
            let z_ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
            assert!(!target.intersect(&z_ray));
        }
    }
}

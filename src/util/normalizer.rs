use super::axis_aligned_bounding_box::BoundingBox;
use super::Interval;

use nalgebra::{clamp, Point3};

use itertools::izip;

#[derive(Debug, Copy, Clone)]
pub struct RealNormalizer {
    min: f64,
    range: f64,
}

impl RealNormalizer {
    pub fn new(interval: Interval) -> Self {
        let min = interval.get_min();
        let range = interval.get_max() - min;
        Self { min, range }
    }

    pub fn normalize(&self, value: f64) -> f64 {
        (value - self.min) / self.range
    }

    pub fn normalize_and_clamp(&self, value: f64) -> f64 {
        clamp((value - self.min) / self.range, 0.0, 1.0)
    }
}

#[derive(Debug)]
pub struct Point3Normalizer {
    dimension_normalizers: [RealNormalizer; 3],
}

impl Point3Normalizer {
    pub fn new(bounds: BoundingBox) -> Self {
        let mut normalizer = Point3Normalizer {
            dimension_normalizers: [RealNormalizer::new(Interval::empty()); 3],
        };
        for (normalizer, &bounds) in normalizer
            .dimension_normalizers
            .iter_mut()
            .zip(bounds.bounds.iter())
        {
            *normalizer = RealNormalizer::new(bounds);
        }
        normalizer
    }

    pub fn normalize(&self, point: Point3<f64>) -> Point3<f64> {
        let mut result = Point3::new(0.0, 0.0, 0.0);
        for (value_out, &value_in, normalizer) in izip!(
            result.iter_mut(),
            point.iter(),
            self.dimension_normalizers.iter()
        ) {
            *value_out = normalizer.normalize(value_in);
        }
        result
    }

    pub fn normalize_and_clamp(&self, point: Point3<f64>) -> Point3<f64> {
        let mut result = Point3::new(0.0, 0.0, 0.0);
        for (value_out, &value_in, normalizer) in izip!(
            result.iter_mut(),
            point.iter(),
            self.dimension_normalizers.iter()
        ) {
            *value_out = normalizer.normalize_and_clamp(value_in);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn normalize_zero_to_one_yields_input(value: f64) -> bool {
        let target = RealNormalizer::new(Interval::new(0.0, 1.0));
        target.normalize(value) == value
    }

    #[quickcheck]
    fn normalize_two_to_three_yields_input_minus_two(value: f64) -> bool {
        let target = RealNormalizer::new(Interval::new(2.0, 3.0));
        target.normalize(value) == value - 2.0
    }

    #[quickcheck]
    fn normalize_negative_three_to_negative_two_yields_input_plus_three(value: f64) -> bool {
        let target = RealNormalizer::new(Interval::new(-3.0, -2.0));
        target.normalize(value) == value + 3.0
    }

    #[quickcheck]
    fn normalize_zero_to_two_yields_input_divided_by_two(value: f64) -> bool {
        let target = RealNormalizer::new(Interval::new(0.0, 2.0));
        target.normalize(value) == value / 2.0
    }

    #[test]
    fn normalize_two_to_four_yields_zero_when_input_is_two() {
        let target = RealNormalizer::new(Interval::new(2.0, 4.0));
        assert!(target.normalize(2.0) == 0.0)
    }

    #[test]
    fn normalize_two_to_four_yields_one_when_input_is_four() {
        let target = RealNormalizer::new(Interval::new(2.0, 4.0));
        assert!(target.normalize(4.0) == 1.0)
    }

    #[quickcheck]
    fn normalize_two_to_four_yields_input_divided_by_two_minus_one(value: f64) -> bool {
        let target = RealNormalizer::new(Interval::new(2.0, 4.0));
        target.normalize(value) == (value - 2.0) / 2.0
    }

    #[quickcheck]
    fn normalize_and_clamp_two_to_four_yields_zero_when_input_less_than_or_equal_two(
        value: f64,
    ) -> bool {
        let target = RealNormalizer::new(Interval::new(2.0, 4.0));
        target.normalize_and_clamp(value) == 0.0 || value > 2.0
    }

    #[quickcheck]
    fn normalize_and_clamp_two_to_four_yields_one_when_input_greater_than_or_equal_four(
        value: f64,
    ) -> bool {
        let target = RealNormalizer::new(Interval::new(2.0, 4.0));
        target.normalize_and_clamp(value) == 1.0 || value < 4.0
    }

    #[quickcheck]
    fn normalize_and_clamp_two_to_four_yields_same_value_as_normalize_when_in_range(
        value: f64,
    ) -> bool {
        let target = RealNormalizer::new(Interval::new(2.0, 4.0));
        target.normalize_and_clamp(value) == target.normalize(value) || value < 2.0 || value > 4.0
    }

    #[quickcheck]
    fn normalize_point3_is_the_same_as_normalize_each_dimension(
        a: Point3<f64>,
        b: Point3<f64>,
        c: Point3<f64>,
    ) -> bool {
        let x_normalizer = RealNormalizer::new(Interval::new(a.x.min(b.x), a.x.max(b.x)));
        let y_normalizer = RealNormalizer::new(Interval::new(a.y.min(b.y), a.y.max(b.y)));
        let z_normalizer = RealNormalizer::new(Interval::new(a.z.min(b.z), a.z.max(b.z)));
        let xyz_normalizer = Point3Normalizer::new(BoundingBox::from_corners(a, b));
        let normalized_point = xyz_normalizer.normalize(c);
        x_normalizer.normalize(c.x) == normalized_point.x
            && y_normalizer.normalize(c.y) == normalized_point.y
            && z_normalizer.normalize(c.z) == normalized_point.z
    }

    #[quickcheck]
    fn normalize_and_clamp_point3_is_the_same_as_normalize_and_clamp_each_dimension(
        a: Point3<f64>,
        b: Point3<f64>,
        c: Point3<f64>,
    ) -> bool {
        let x_normalizer = RealNormalizer::new(Interval::new(a.x.min(b.x), a.x.max(b.x)));
        let y_normalizer = RealNormalizer::new(Interval::new(a.y.min(b.y), a.y.max(b.y)));
        let z_normalizer = RealNormalizer::new(Interval::new(a.z.min(b.z), a.z.max(b.z)));
        let xyz_normalizer = dbg!(Point3Normalizer::new(BoundingBox::from_corners(a, b)));
        let normalized_point = xyz_normalizer.normalize_and_clamp(c);
        x_normalizer.normalize_and_clamp(c.x) == normalized_point.x
            && y_normalizer.normalize_and_clamp(c.y) == normalized_point.y
            && z_normalizer.normalize_and_clamp(c.z) == normalized_point.z
    }
}

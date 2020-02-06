use super::Interval;

use nalgebra::{clamp, RealField};

pub struct RealFieldNormalizer<T: RealField> {
    min: T,
    range: T,
}

impl<T: RealField> RealFieldNormalizer<T> {
    pub fn new(interval: Interval<T>) -> Self {
        let min = interval.get_min();
        let range = interval.get_max() - min;
        Self { min, range }
    }

    pub fn normalize(&self, value: T) -> T {
        (value - self.min) / self.range
    }

    pub fn normalize_and_clamp(&self, value: T) -> T {
        clamp((value - self.min) / self.range, T::zero(), T::one())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn normalize_zero_to_one_yields_input(value: f64) -> bool {
        let target = RealFieldNormalizer::new(Interval::new(0.0, 1.0));
        target.normalize(value) == value
    }

    #[quickcheck]
    fn normalize_two_to_three_yields_input_minus_two(value: f64) -> bool {
        let target = RealFieldNormalizer::new(Interval::new(2.0, 3.0));
        target.normalize(value) == value - 2.0
    }

    #[quickcheck]
    fn normalize_negative_three_to_negative_two_yields_input_plus_three(value: f64) -> bool {
        let target = RealFieldNormalizer::new(Interval::new(-3.0, -2.0));
        target.normalize(value) == value + 3.0
    }

    #[quickcheck]
    fn normalize_zero_to_two_yields_input_divided_by_two(value: f64) -> bool {
        let target = RealFieldNormalizer::new(Interval::new(0.0, 2.0));
        target.normalize(value) == value / 2.0
    }

    #[test]
    fn normalize_two_to_four_yields_zero_when_input_is_two() {
        let target = RealFieldNormalizer::new(Interval::new(2.0, 4.0));
        assert!(target.normalize(2.0) == 0.0)
    }

    #[test]
    fn normalize_two_to_four_yields_one_when_input_is_four() {
        let target = RealFieldNormalizer::new(Interval::new(2.0, 4.0));
        assert!(target.normalize(4.0) == 1.0)
    }

    #[quickcheck]
    fn normalize_two_to_four_yields_input_divided_by_two_minus_one(value: f64) -> bool {
        let target = RealFieldNormalizer::new(Interval::new(2.0, 4.0));
        target.normalize(value) == (value - 2.0) / 2.0
    }

    #[quickcheck]
    fn normalize_and_clamp_two_to_four_yields_zero_when_input_less_than_or_equal_two(
        value: f64,
    ) -> bool {
        let target = RealFieldNormalizer::new(Interval::new(2.0, 4.0));
        target.normalize_and_clamp(value) == 0.0 || value > 2.0
    }

    #[quickcheck]
    fn normalize_and_clamp_two_to_four_yields_one_when_input_greater_than_or_equal_four(
        value: f64,
    ) -> bool {
        let target = RealFieldNormalizer::new(Interval::new(2.0, 4.0));
        target.normalize_and_clamp(value) == 1.0 || value < 4.0
    }

    #[quickcheck]
    fn normalize_and_clamp_two_to_four_yields_same_value_as_normalize_when_in_range(
        value: f64,
    ) -> bool {
        let target = RealFieldNormalizer::new(Interval::new(2.0, 4.0));
        target.normalize_and_clamp(value) == target.normalize(value) || value < 2.0 || value > 4.0
    }
}

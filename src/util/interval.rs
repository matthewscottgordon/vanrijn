use nalgebra::convert;

use crate::Real;

#[derive(Debug, Clone, Copy)]
pub struct Interval<T: Real> {
    min: T,
    max: T,
}

impl<T: Real> Interval<T> {
    pub fn new(a: T, b: T) -> Self {
        if a > b {
            Interval { min: b, max: a }
        } else {
            Interval { min: a, max: b }
        }
    }

    pub fn empty() -> Self {
        Interval {
            min: convert(std::f64::INFINITY),
            max: convert(std::f64::NEG_INFINITY),
        }
    }

    pub fn infinite() -> Self {
        Interval {
            min: convert(std::f64::NEG_INFINITY),
            max: convert(std::f64::INFINITY),
        }
    }

    pub fn degenerate(value: T) -> Self {
        Interval {
            min: value,
            max: value,
        }
    }

    pub fn get_min(&self) -> T {
        self.min
    }

    pub fn get_max(&self) -> T {
        self.max
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

    pub fn intersection(self, b: Self) -> Self {
        Interval {
            min: self.min.max(b.min),
            max: self.max.min(b.max),
        }
    }

    pub fn union(self, b: Self) -> Self {
        if self.is_empty() {
            b
        } else if b.is_empty() {
            self
        } else {
            Interval {
                min: self.min.min(b.min),
                max: self.max.max(b.max),
            }
        }
    }

    pub fn expand_to_value(self, v: T) -> Self {
        if self.is_empty() {
            Interval::degenerate(v)
        } else {
            Interval {
                min: self.min.min(v),
                max: self.max.max(v),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use itertools::{Itertools, MinMaxResult};

    use quickcheck_macros::quickcheck;

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
        let target: Interval<f64> = Interval::empty();
        assert!(target.is_empty());
    }

    #[test]
    fn empty_when_min_greater_than_max() {
        let target = Interval {
            min: 10f64,
            max: 5f64,
        };
        assert!(target.is_empty());
    }

    #[test]
    fn empty_when_min_greater_than_max_with_negative_values() {
        let target = Interval {
            min: -5f64,
            max: -10f64,
        };
        assert!(target.is_empty());
    }

    #[test]
    fn empty_when_min_greater_than_max_with_mixed_signs() {
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

    #[quickcheck]
    fn no_value_is_in_interval_returned_by_emtpy(value: f64) -> bool {
        !Interval::empty().contains_value(value)
    }

    #[test]
    fn identical_min_max_yields_degenerate() {
        let target = Interval {
            min: 5f64,
            max: 5f64,
        };
        assert!(target.is_degenerate());
    }

    #[test]
    fn degenerate_is_degenerate() {
        let target = Interval::degenerate(5f64);
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
    fn degenerate_contains_expected_value() {
        let target = Interval::degenerate(5f64);
        assert!(target.contains_value(5.0));
    }

    #[quickcheck]
    fn degenerate_does_not_contain_any_values_othter_than_expected_value(value: f64) -> bool {
        let target_value = if value == 5f64 { 5.5 } else { 5f64 };
        let target = Interval::degenerate(target_value);
        !target.contains_value(value)
    }

    #[test]
    fn intersection_with_infinite_is_self() {
        let target = Interval::new(5f32, 10f32);
        let result = target.intersection(Interval::infinite());
        assert!(target.min == result.min);
        assert!(target.max == result.max);
    }

    #[quickcheck]
    fn union_with_self_yields_self(a: f64, b: f64) -> bool {
        let target = Interval::new(a, b);
        let result = target.union(target);
        result.min == target.min && result.max == target.max
    }

    #[quickcheck]
    fn union_yields_min_and_max(a: f64, b: f64, c: f64, d: f64) -> bool {
        let values = vec![a, b, c, d];
        if let MinMaxResult::MinMax(&min, &max) =
            values.iter().minmax_by(|a, b| a.partial_cmp(b).unwrap())
        {
            let target1 = Interval::new(a, b);
            let target2 = Interval::new(c, d);
            let result = target1.union(target2);
            result.min == min && result.max == max
        } else {
            false
        }
    }

    #[test]
    fn union_with_empty_interval_is_correct() {
        let empty = Interval {
            min: 1f64,
            max: -1f64,
        };
        let not_empty = Interval {
            min: 5f64,
            max: 10f64,
        };
        let union1 = not_empty.union(empty);
        assert!(union1.min == 5.0);
        assert!(union1.max == 10.0);
        let union2 = empty.union(not_empty);
        assert!(union2.min == 5.0);
        assert!(union2.max == 10.0);
    }

    #[test]
    fn union_with_empty_interval_is_correct_when_empty_interval_produced_by_intersection() {
        let empty = Interval {
            min: 1f64,
            max: -1f64,
        };
        let not_empty = Interval {
            min: 5f64,
            max: 10f64,
        };
        let union1 = not_empty.union(empty);
        assert!(union1.min == 5.0);
        assert!(union1.max == 10.0);
        let union2 = empty.union(not_empty);
        assert!(union2.min == 5.0);
        assert!(union2.max == 10.0);
    }

    #[quickcheck]
    pub fn expand_to_value_creates_interval_that_includes_value(
        min: f64,
        max: f64,
        value: f64,
    ) -> bool {
        // Don't check if min <= max, we want to test empty intervals too
        let interval1 = Interval { min, max };
        let interval2 = interval1.expand_to_value(value);
        interval2.contains_value(value)
    }

    #[quickcheck]
    pub fn expand_to_value_creates_interval_that_includes_original_interval(
        b: f64,
        a: f64,
        value: f64,
    ) -> bool {
        let interval1 = Interval::new(a, b);
        let interval2 = interval1.expand_to_value(value);
        let interval3 = interval2.intersection(interval1);
        // If interval2 contains interval1, that the intersection of the two will
        // be equal to interval1
        interval1.min == interval3.min && interval1.max == interval3.max
    }
}

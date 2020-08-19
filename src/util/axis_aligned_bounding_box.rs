use nalgebra::Point3;

use crate::util::Interval;

use itertools::izip;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub bounds: [Interval; 3],
}

impl BoundingBox {
    pub fn from_corners(a: Point3<f64>, b: Point3<f64>) -> Self {
        let mut result = BoundingBox {
            bounds: [Interval::infinite(); 3],
        };
        for (bounds_elem, a_elem, b_elem) in izip!(result.bounds.iter_mut(), a.iter(), b.iter()) {
            *bounds_elem = Interval::new(*a_elem, *b_elem);
        }
        result
    }

    pub fn empty() -> Self {
        BoundingBox {
            bounds: [Interval::empty(), Interval::empty(), Interval::empty()],
        }
    }

    pub fn from_point(p: Point3<f64>) -> Self {
        BoundingBox {
            bounds: [
                Interval::degenerate(p.x),
                Interval::degenerate(p.y),
                Interval::degenerate(p.z),
            ],
        }
    }

    pub fn from_points<'a, I>(points: I) -> Self
    where
        I: IntoIterator<Item = &'a Point3<f64>>,
    {
        points
            .into_iter()
            .fold(BoundingBox::empty(), |acc, p| acc.expand_to_point(p))
    }

    pub fn expand_to_point(&self, p: &Point3<f64>) -> Self {
        BoundingBox {
            bounds: [
                self.bounds[0].expand_to_value(p.x),
                self.bounds[1].expand_to_value(p.y),
                self.bounds[2].expand_to_value(p.z),
            ],
        }
    }

    pub fn contains_point(&self, p: Point3<f64>) -> bool {
        self.bounds
            .iter()
            .zip(p.iter())
            .all(|(interval, &value)| interval.contains_value(value))
    }

    pub fn union(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            bounds: [
                self.bounds[0].union(other.bounds[0]),
                self.bounds[1].union(other.bounds[1]),
                self.bounds[2].union(other.bounds[2]),
            ],
        }
    }

    pub fn largest_dimension(&self) -> usize {
        let (dimension, _) = self
            .bounds
            .iter()
            .enumerate()
            .map(|(index, elem)| {
                (
                    index,
                    if elem.is_degenerate() {
                        -1.0
                    } else {
                        elem.get_max() - elem.get_min()
                    },
                )
            })
            .fold((0, 0.0), |(acc, acc_size), (elem, elem_size)| {
                if elem_size > acc_size {
                    (elem, elem_size)
                } else {
                    (acc, acc_size)
                }
            });
        dimension
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                .all(|bounds| bounds.get_min() == 0.0 && bounds.get_max() == 1.0));
        }
    }

    #[quickcheck]
    fn union_with_self_yields_self(a: Point3<f64>, b: Point3<f64>) -> bool {
        let target = BoundingBox::from_corners(a, b);
        let result = target.union(&target);
        target
            .bounds
            .iter()
            .zip(result.bounds.iter())
            .all(|(a, b)| a.get_min() == b.get_min() && a.get_max() == b.get_max())
    }

    #[quickcheck]
    fn union_yields_full_ranges(
        a: Point3<f64>,
        b: Point3<f64>,
        c: Point3<f64>,
        d: Point3<f64>,
    ) -> bool {
        let target1 = BoundingBox::from_corners(a, b);
        let target2 = BoundingBox::from_corners(c, d);
        let result = target1.union(&target2);
        izip!(
            result.bounds.iter(),
            target1.bounds.iter(),
            target2.bounds.iter()
        )
        .all(|(r, t1, t2)| {
            r.get_min() <= t1.get_min()
                && r.get_min() <= t2.get_min()
                && r.get_max() >= t1.get_max()
                && r.get_max() >= t2.get_max()
        })
    }

    #[quickcheck]
    fn empty_box_contains_no_points(p: Point3<f64>) -> bool {
        let target = BoundingBox::empty();
        !target.contains_point(p)
    }

    #[quickcheck]
    fn from_points_produces_box_that_contains_only_points_bounded_by_inputs_on_all_axes(
        p: Point3<f64>,
        a: Point3<f64>,
        b: Point3<f64>,
        c: Point3<f64>,
        d: Point3<f64>,
        e: Point3<f64>,
    ) -> bool {
        let points = vec![a, b, c, d, e];
        let target = BoundingBox::from_points(&points);
        let is_in_bounds = points.iter().any(|elem| elem.x >= p.x)
            && points.iter().any(|elem| elem.x <= p.x)
            && points.iter().any(|elem| elem.y >= p.y)
            && points.iter().any(|elem| elem.y <= p.y)
            && points.iter().any(|elem| elem.z >= p.z)
            && points.iter().any(|elem| elem.z <= p.z);
        target.contains_point(p) == is_in_bounds
    }

    #[quickcheck]
    fn no_dimension_is_larger_than_largest_dimension(
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> bool {
        let target = BoundingBox {
            bounds: [
                if a > b {
                    Interval::empty()
                } else {
                    Interval::new(a, b)
                },
                if c > d {
                    Interval::empty()
                } else {
                    Interval::new(c, d)
                },
                if e > f {
                    Interval::empty()
                } else {
                    Interval::new(e, f)
                },
            ],
        };
        let largest_dimension = target.largest_dimension();
        let largest_bounds = target.bounds[largest_dimension];
        if largest_bounds.is_empty() {
            target.bounds.iter().all(|elem| elem.is_empty())
        } else {
            let largest_size = largest_bounds.get_max() - largest_bounds.get_min();
            target
                .bounds
                .iter()
                .all(|elem| elem.is_empty() || !(largest_size < elem.get_max() - elem.get_min()))
        }
    }
}

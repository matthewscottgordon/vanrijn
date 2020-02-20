use nalgebra::{Point2, Point3};

use crate::Real;

//use std::cmp::Ordering;

fn spread_bits(v: u32) -> u32 {
    let mut result = 0;
    for power in 0..9 {
        result |= ((1 << power) & v) << power * 2;
    }
    dbg!(format!("{:b}", result));
    result
}

pub fn morton_order_value_3d<T: Real>(p: Point3<T>) -> u32 {
    let x = p.x.normalized_to_u32(10);
    let y = p.y.normalized_to_u32(10);
    let z = p.z.normalized_to_u32(10);
    (spread_bits(x) << 2) | (spread_bits(y) << 1) | spread_bits(z)
}

#[cfg(test)]
mod tests {
    use super::*;
    mod spread_bits {
        use super::*;

        #[test]
        fn zero_yields_zero() {
            assert!(spread_bits(0) == 0);
        }

        #[test]
        fn one_yields_one() {
            assert!(spread_bits(1) == 1);
        }

        #[test]
        fn b1111_yields_b1001001001() {
            assert!(spread_bits(0b1111) == 0b1001001001);
        }

        #[test]
        fn b1010_yields_b1000001000() {
            assert!(spread_bits(0b1010) == 0b1000001000);
        }
    }
}

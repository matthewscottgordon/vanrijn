use super::Mat3;

use itertools::izip;

use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Vec3 {
    pub coords: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { coords: [x, y, z] }
    }

    pub fn from_slice(v: &[f64]) -> Self {
        let mut coords = [0.0; 3];
        coords.clone_from_slice(v);
        Vec3 { coords }
    }

    /*pub fn from_iterator<I>(values: I) -> Vec3
    where
        I: Iterator<Item = f64>,
    {
        Vec3 {
            coords: [
                values.next().unwrap(),
                values.next().unwrap(),
                values.next().unwrap(),
            ],
        }
    }*/

    pub fn zeros() -> Vec3 {
        Vec3 {
            coords: [0.0, 0.0, 0.0],
        }
    }

    pub fn unit_x() -> Vec3 {
        Vec3 {
            coords: [1.0, 0.0, 0.0],
        }
    }

    pub fn unit_y() -> Vec3 {
        Vec3 {
            coords: [0.0, 1.0, 0.0],
        }
    }

    pub fn unit_z() -> Vec3 {
        Vec3 {
            coords: [0.0, 0.0, 1.0],
        }
    }

    pub fn x(&self) -> f64 {
        self.coords[0]
    }

    pub fn y(&self) -> f64 {
        self.coords[1]
    }

    pub fn z(&self) -> f64 {
        self.coords[2]
    }

    pub fn as_slice(&self) -> &[f64] {
        &self.coords
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.coords
            .iter()
            .zip(rhs.coords.iter())
            .map(|(a_elem, b_elem)| a_elem * b_elem)
            .sum()
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        let x = self.y() * rhs.z() - self.z() * rhs.y();
        let y = self.z() * rhs.x() - self.x() * rhs.z();
        let z = self.x() * rhs.y() - self.y() * rhs.x();
        Vec3 { coords: [x, y, z] }
    }

    pub fn abs(&self) -> Self {
        Vec3::new(self.x().abs(), self.y().abs(), self.z().abs())
    }

    pub fn norm_squared(&self) -> f64 {
        self.dot(&self)
    }

    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mut coords = [0.0; 3];
        let inverse_norm = 1.0 / self.norm();
        for (r, a) in coords.iter_mut().zip(self.coords.iter()) {
            *r = a * inverse_norm;
        }
        Vec3 { coords }
    }

    pub fn smallest_coord(&self) -> usize {
        let x = self.x().abs();
        let y = self.y().abs();
        let z = self.z().abs();
        if x < y {
            if x < z {
                0
            } else {
                2
            }
        } else {
            if y < z {
                1
            } else {
                2
            }
        }
    }

    pub fn component_mul(&self, rhs: &Self) -> Self {
        let mut coords = [0.0; 3];
        for (elem, lhs_elem, rhs_elem) in
            izip!(coords.iter_mut(), self.coords.iter(), rhs.coords.iter())
        {
            *elem = lhs_elem * rhs_elem;
        }
        Vec3 { coords }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.coords[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.coords[i]
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        let mut coords = [0.0; 3];
        for (r, a, b) in izip!(coords.iter_mut(), self.coords.iter(), rhs.coords.iter()) {
            *r = a + b;
        }
        Vec3 { coords }
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        let mut coords = [0.0; 3];
        for (r, a, b) in izip!(coords.iter_mut(), self.coords.iter(), rhs.coords.iter()) {
            *r = a + b;
        }
        Vec3 { coords }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut coords = [0.0; 3];
        for (r, a, b) in izip!(coords.iter_mut(), self.coords.iter(), rhs.coords.iter()) {
            *r = a + b;
        }
        Vec3 { coords }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        for (a, b) in self.coords.iter_mut().zip(rhs.coords.iter()) {
            *a += b;
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        let mut coords = [0.0; 3];
        for (r, a, b) in izip!(coords.iter_mut(), self.coords.iter(), rhs.coords.iter()) {
            *r = a - b;
        }
        Vec3 { coords }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        let mut coords = [0.0; 3];
        for (r, a, b) in izip!(coords.iter_mut(), self.coords.iter(), rhs.coords.iter()) {
            *r = a - b;
        }
        Vec3 { coords }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        for (a, b) in self.coords.iter_mut().zip(rhs.coords.iter()) {
            *a -= b;
        }
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        let mut coords = [0.0; 3];
        for (r, a) in coords.iter_mut().zip(self.coords.iter()) {
            *r = a * rhs;
        }
        Vec3 { coords }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        let mut coords = [0.0; 3];
        for (r, a) in coords.iter_mut().zip(self.coords.iter()) {
            *r = a * rhs;
        }
        Vec3 { coords }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        for a in self.coords.iter_mut() {
            *a *= rhs;
        }
    }
}

impl Mul<Mat3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Mat3) -> Vec3 {
        let mut coords = [0.0; 3];
        for i in 0..3 {
            coords[i] = self.dot(&rhs.get_column(i));
        }
        Vec3 { coords }
    }
}

impl Mul<Mat3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Mat3) -> Self {
        let mut coords = [0.0; 3];
        for i in 0..3 {
            coords[i] = self.dot(&rhs.get_column(i));
        }
        Vec3 { coords }
    }
}

impl MulAssign<Mat3> for Vec3 {
    fn mul_assign(&mut self, rhs: Mat3) {
        let mut coords = [0.0; 3];
        for i in 0..3 {
            coords[i] = self.dot(&rhs.get_column(i));
        }
        self.coords = coords;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        rhs * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for Vec3 {
        fn arbitrary<G: Gen>(g: &mut G) -> Vec3 {
            Vec3::new(f64::arbitrary(g), f64::arbitrary(g), f64::arbitrary(g))
        }
    }

    #[test]
    fn x_returns_first_element() {
        let target = Vec3::new(1.0, 2.0, 3.0);
        assert!(target.x() == 1.0);
    }

    #[test]
    fn y_returns_second_element() {
        let target = Vec3::new(1.0, 2.0, 3.0);
        assert!(target.y() == 2.0);
    }

    #[test]
    fn z_returns_third_element() {
        let target = Vec3::new(1.0, 2.0, 3.0);
        assert!(target.z() == 3.0);
    }

    /*#[test]
    fn from_iterator_takes_first_three_elements() {
        let target = Vec3::from_iterator([1.0, 2.0, 3.0].iter());
        assert!(target = Vec3::new(1.0, 2.0, 3.0));
    }*/

    #[test]
    fn dot_product_returns_correct_result() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert!(a.dot(&b) == 32.0);
    }

    #[test]
    fn cross_product_returns_correct_result() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let c = Vec3::new(-3.0, 6.0, -3.0);
        assert!(a.cross(&b) == c);
    }

    #[test]
    fn norm_returns_expected_value() {
        let target = Vec3::new(2.0, 3.0, 6.0);
        assert!(target.norm() == 7.0);
    }

    #[test]
    fn normalized_vector_times_norm_yields_original() {
        let mut target = Vec3::new(2.0, 3.0, 6.0);
        let norm = target.norm();
        target = target.normalize();
        target *= norm;
        assert!(target == Vec3::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn smallest_coord_works_for_x_when_positive() {
        let target = Vec3::new(1.0, 2.0, 3.0);
        assert!(target.smallest_coord() == 0);
    }

    #[test]
    fn smallest_coord_works_for_x_when_negative() {
        let target = Vec3::new(-2.0, -3.0, 3.0);
        assert!(target.smallest_coord() == 0);
    }

    #[test]
    fn smallest_coord_works_for_y_when_positive() {
        let target = Vec3::new(2.0, 1.0, 3.0);
        assert!(target.smallest_coord() == 1);
    }

    #[test]
    fn smallest_coord_works_for_y_when_negative() {
        let target = Vec3::new(-3.0, -2.0, 3.0);
        assert!(target.smallest_coord() == 1);
    }

    #[test]
    fn smallest_coord_works_for_z_when_positive() {
        let target = Vec3::new(3.0, 2.0, 1.0);
        assert!(target.smallest_coord() == 2);
    }

    #[test]
    fn smallest_coord_works_for_z_when_negative() {
        let target = Vec3::new(3.0, -3.0, -2.0);
        assert!(target.smallest_coord() == 2);
    }

    #[test]
    fn add_returns_correct_result() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let c = Vec3::new(5.0, 7.0, 9.0);
        assert!(a + b == c);
    }

    #[test]
    fn add_assign_returns_correct_result() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let c = Vec3::new(5.0, 7.0, 9.0);
        a += b;
        assert!(a == c);
    }

    #[test]
    fn sub_returns_correct_result() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 6.0, 8.0);
        let c = Vec3::new(-3.0, -4.0, -5.0);
        assert!(a - b == c);
    }

    #[test]
    fn sub_assign_returns_correct_result() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 6.0, 8.0);
        let c = Vec3::new(-3.0, -4.0, -5.0);
        a -= b;
        assert!(a == c);
    }

    #[test]
    fn mul_by_scalar_returns_correct_result() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 0.5;
        let c = Vec3::new(0.5, 1.0, 1.5);
        assert!(a * b == c);
    }

    #[test]
    fn mul_assign_by_scalar_returns_correct_result() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 0.5;
        let c = Vec3::new(0.5, 1.0, 1.5);
        a *= b;
        assert!(a == c);
    }

    #[test]
    fn mul_with_mat3_returns_expected_result() {
        let a = Mat3::from_rows(
            &Vec3::new(1.0, 2.0, 3.0),
            &Vec3::new(4.0, 5.0, 6.0),
            &Vec3::new(7.0, 8.0, 9.0),
        );
        let b = Vec3::new(10.0, 11.0, 12.0);
        let c = Vec3::new(138.0, 171.0, 204.0);
        assert!(b * a == c);
    }

    #[test]
    fn mul_assign_with_mat3_returns_expected_result() {
        let a = Mat3::from_rows(
            &Vec3::new(1.0, 2.0, 3.0),
            &Vec3::new(4.0, 5.0, 6.0),
            &Vec3::new(7.0, 8.0, 9.0),
        );
        let mut b = Vec3::new(10.0, 11.0, 12.0);
        let c = Vec3::new(138.0, 171.0, 204.0);
        b *= a;
        assert!(b == c);
    }
}

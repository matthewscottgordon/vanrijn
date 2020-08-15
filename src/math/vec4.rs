use itertools::izip;

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(PartialEq, Debug)]
pub struct Vec4 {
    coords: [f64; 4],
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Vec4 {
            coords: [x, y, z, w],
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

    pub fn w(&self) -> f64 {
        self.coords[3]
    }

    pub fn dot(&self, rhs: &Vec4) -> f64 {
        self.coords
            .iter()
            .zip(rhs.coords.iter())
            .map(|(a_elem, b_elem)| a_elem * b_elem)
            .sum()
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut coords = [0.0; 4];
        for (r, a, b) in izip!(coords.iter_mut(), self.coords.iter(), rhs.coords.iter()) {
            *r = a + b;
        }
        Vec4 { coords }
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        for (a, b) in self.coords.iter_mut().zip(rhs.coords.iter()) {
            *a += b;
        }
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut coords = [0.0; 4];
        for (r, a, b) in izip!(coords.iter_mut(), self.coords.iter(), rhs.coords.iter()) {
            *r = a - b;
        }
        Vec4 { coords }
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        for (a, b) in self.coords.iter_mut().zip(rhs.coords.iter()) {
            *a -= b;
        }
    }
}

impl Mul<f64> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Vec4 {
        let mut coords = [0.0; 4];
        for (r, a) in coords.iter_mut().zip(self.coords.iter()) {
            *r = a * rhs;
        }
        Vec4 { coords }
    }
}

impl MulAssign<f64> for Vec4 {
    fn mul_assign(&mut self, rhs: f64) {
        for a in self.coords.iter_mut() {
            *a *= rhs;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_returns_first_element() {
        let target = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert!(target.x() == 1.0);
    }

    #[test]
    fn y_returns_second_element() {
        let target = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert!(target.y() == 2.0);
    }

    #[test]
    fn z_returns_third_element() {
        let target = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert!(target.z() == 3.0);
    }

    #[test]
    fn w_returns_third_element() {
        let target = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert!(target.w() == 4.0);
    }

    #[test]
    fn dot_product_returns_correct_result() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(4.0, 5.0, 6.0, 7.0);
        assert!(a.dot(&b) == 60.0);
    }

    #[test]
    fn add_returns_correct_result() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(4.0, 5.0, 6.0, 7.0);
        let c = Vec4::new(5.0, 7.0, 9.0, 11.0);
        assert!(a + b == c);
    }

    #[test]
    fn add_assign_returns_correct_result() {
        let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(4.0, 5.0, 6.0, 7.0);
        let c = Vec4::new(5.0, 7.0, 9.0, 11.0);
        a += b;
        assert!(a == c);
    }

    #[test]
    fn sub_returns_correct_result() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(4.0, 6.0, 8.0, 10.0);
        let c = Vec4::new(-3.0, -4.0, -5.0, -6.0);
        assert!(a - b == c);
    }

    #[test]
    fn sub_assign_returns_correct_result() {
        let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(4.0, 6.0, 8.0, 10.0);
        let c = Vec4::new(-3.0, -4.0, -5.0, -6.0);
        a -= b;
        assert!(a == c);
    }

    #[test]
    fn mul_by_scalar_returns_correct_result() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = 0.5;
        let c = Vec4::new(0.5, 1.0, 1.5, 2.0);
        assert!(a * b == c);
    }

    #[test]
    fn mul_assign_by_scalar_returns_correct_result() {
        let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = 0.5;
        let c = Vec4::new(0.5, 1.0, 1.5, 2.0);
        a *= b;
        assert!(a == c);
    }
}

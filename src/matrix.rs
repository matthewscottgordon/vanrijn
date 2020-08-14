use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(PartialEq, Debug)]
pub struct Vec3 {
    coords: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { coords: [x, y, z] }
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
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x() + other.x();
        let y = self.y() + other.y();
        let z = self.z() + other.z();
        Vec3 { coords: [x, y, z] }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        for (a, b) in self.coords.iter_mut().zip(other.coords.iter()) {
            *a += b;
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        let z = self.z() - other.z();
        Vec3 { coords: [x, y, z] }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        for (a, b) in self.coords.iter_mut().zip(other.coords.iter()) {
            *a -= b;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod vec3 {
        use super::*;

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
    }
}

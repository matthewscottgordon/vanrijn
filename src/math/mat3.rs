use super::Vec3;

use std::ops::{Mul, MulAssign};

#[derive(PartialEq, Debug)]
pub struct Mat3 {
    elements: [[f64; 3]; 3],
}

impl Mat3 {
    pub fn new(
        m00: f64,
        m01: f64,
        m02: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m20: f64,
        m21: f64,
        m22: f64,
    ) -> Mat3 {
        Mat3 {
            elements: [[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]],
        }
    }

    pub fn from_rows(r0: &Vec3, r1: &Vec3, r2: &Vec3) -> Mat3 {
        let mut elements = [[0.0; 3]; 3];
        for (row, v) in elements.iter_mut().zip([r0, r1, r2].iter()) {
            for (it, val) in row.iter_mut().zip(v.coords.iter()) {
                *it = *val;
            }
        }
        Mat3 { elements }
    }

    pub fn get_element(&self, row: usize, column: usize) -> f64 {
        self.elements[row][column]
    }

    pub fn get_row(&self, row: usize) -> Vec3 {
        Vec3 {
            coords: self.elements[row],
        }
    }

    pub fn get_column(&self, column: usize) -> Vec3 {
        let mut coords = [0.0; 3];
        for (coord, row) in coords.iter_mut().zip(self.elements.iter()) {
            *coord = row[column];
        }
        Vec3 { coords }
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut elements = [[0.0; 3]; 3];
        for row in 0..3 {
            for column in 0..3 {
                elements[row][column] = self.get_row(row).dot(&rhs.get_column(column));
            }
        }
        Mat3 { elements }
    }
}

impl MulAssign<Mat3> for Mat3 {
    fn mul_assign(&mut self, rhs: Self) {
        for row in 0..3 {
            let mut new_row = [0.0; 3];
            for column in 0..3 {
                new_row[column] = self.get_row(row).dot(&rhs.get_column(column));
            }
            self.elements[row] = new_row;
        }
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let mut coords = [0.0; 3];
        for (coord, row) in coords.iter_mut().zip(self.elements.iter()) {
            *coord = Vec3 { coords: *row }.dot(&rhs);
        }
        Vec3 { coords }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elements_are_in_expected_locations() {
        let target = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert!(target.get_element(0, 0) == 1.0);
        assert!(target.get_element(0, 1) == 2.0);
        assert!(target.get_element(0, 2) == 3.0);
        assert!(target.get_element(1, 0) == 4.0);
        assert!(target.get_element(1, 1) == 5.0);
        assert!(target.get_element(1, 2) == 6.0);
        assert!(target.get_element(2, 0) == 7.0);
        assert!(target.get_element(2, 1) == 8.0);
        assert!(target.get_element(2, 2) == 9.0);
    }

    #[test]
    fn from_rows_places_values_in_rows() {
        let target = Mat3::from_rows(
            &Vec3::new(1.0, 2.0, 3.0),
            &Vec3::new(4.0, 5.0, 6.0),
            &Vec3::new(7.0, 8.0, 9.0),
        );
        assert!(target.get_element(0, 0) == 1.0);
        assert!(target.get_element(0, 1) == 2.0);
        assert!(target.get_element(0, 2) == 3.0);
        assert!(target.get_element(1, 0) == 4.0);
        assert!(target.get_element(1, 1) == 5.0);
        assert!(target.get_element(1, 2) == 6.0);
        assert!(target.get_element(2, 0) == 7.0);
        assert!(target.get_element(2, 1) == 8.0);
        assert!(target.get_element(2, 2) == 9.0);
    }

    #[test]
    fn get_column_returns_expected_value() {
        let target = Mat3::from_rows(
            &Vec3::new(1.0, 2.0, 3.0),
            &Vec3::new(4.0, 5.0, 6.0),
            &Vec3::new(7.0, 8.0, 9.0),
        );
        assert!(target.get_column(0) == Vec3::new(1.0, 4.0, 7.0));
        assert!(target.get_column(1) == Vec3::new(2.0, 5.0, 8.0));
        assert!(target.get_column(2) == Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn mul_with_mat3_returns_expected_result() {
        let a = Mat3::from_rows(
            &Vec3::new(1.0, 2.0, 3.0),
            &Vec3::new(4.0, 5.0, 6.0),
            &Vec3::new(7.0, 8.0, 9.0),
        );
        let b = Mat3::from_rows(
            &Vec3::new(10.0, 11.0, 12.0),
            &Vec3::new(13.0, 14.0, 15.0),
            &Vec3::new(16.0, 17.0, 18.0),
        );
        let c = Mat3::from_rows(
            &Vec3::new(84.0, 90.0, 96.0),
            &Vec3::new(201.0, 216.0, 231.0),
            &Vec3::new(318.0, 342.0, 366.0),
        );
        assert!(a * b == c);
    }

    #[test]
    fn mul_assign_returns_expected_result() {
        let mut a = Mat3::from_rows(
            &Vec3::new(1.0, 2.0, 3.0),
            &Vec3::new(4.0, 5.0, 6.0),
            &Vec3::new(7.0, 8.0, 9.0),
        );
        let b = Mat3::from_rows(
            &Vec3::new(10.0, 11.0, 12.0),
            &Vec3::new(13.0, 14.0, 15.0),
            &Vec3::new(16.0, 17.0, 18.0),
        );
        let c = Mat3::from_rows(
            &Vec3::new(84.0, 90.0, 96.0),
            &Vec3::new(201.0, 216.0, 231.0),
            &Vec3::new(318.0, 342.0, 366.0),
        );

        a *= b;
        assert!(a == c);
    }

    #[test]
    fn mul_with_vec3_returns_expected_result() {
        let a = Mat3::from_rows(
            &Vec3::new(1.0, 2.0, 3.0),
            &Vec3::new(4.0, 5.0, 6.0),
            &Vec3::new(7.0, 8.0, 9.0),
        );
        let b = Vec3::new(10.0, 11.0, 12.0);
        let c = Vec3::new(68.0, 167.0, 266.0);
        assert!(a * b == c);
    }
}

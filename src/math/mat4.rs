use super::Vec4;

use std::ops::{Mul, MulAssign};

#[derive(PartialEq, Debug)]
pub struct Mat4 {
    elements: [[f64; 4]; 4],
}

impl Mat4 {
    pub fn new(
        m00: f64,
        m01: f64,
        m02: f64,
        m03: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m13: f64,
        m20: f64,
        m21: f64,
        m22: f64,
        m23: f64,
        m30: f64,
        m31: f64,
        m32: f64,
        m33: f64,
    ) -> Mat4 {
        Mat4 {
            elements: [
                [m00, m01, m02, m03],
                [m10, m11, m12, m13],
                [m20, m21, m22, m23],
                [m30, m31, m32, m33],
            ],
        }
    }

    pub fn from_rows(r0: &Vec4, r1: &Vec4, r2: &Vec4, r3: &Vec4) -> Mat4 {
        let mut elements = [[0.0; 4]; 4];
        for (row, v) in elements.iter_mut().zip([r0, r1, r2, r3].iter()) {
            for (it, val) in row.iter_mut().zip(v.coords.iter()) {
                *it = *val;
            }
        }
        Mat4 { elements }
    }

    pub fn get_element(&self, row: usize, column: usize) -> f64 {
        self.elements[row][column]
    }

    pub fn get_row(&self, row: usize) -> Vec4 {
        Vec4 {
            coords: self.elements[row],
        }
    }

    pub fn get_column(&self, column: usize) -> Vec4 {
        let mut coords = [0.0; 4];
        for (coord, row) in coords.iter_mut().zip(self.elements.iter()) {
            *coord = row[column];
        }
        Vec4 { coords }
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut elements = [[0.0; 4]; 4];
        for row in 0..4 {
            for column in 0..4 {
                elements[row][column] = self.get_row(row).dot(&rhs.get_column(column));
            }
        }
        Mat4 { elements }
    }
}

impl MulAssign<Mat4> for Mat4 {
    fn mul_assign(&mut self, rhs: Self) {
        for row in 0..4 {
            let mut new_row = [0.0; 4];
            for column in 0..4 {
                new_row[column] = self.get_row(row).dot(&rhs.get_column(column));
            }
            self.elements[row] = new_row;
        }
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        let mut coords = [0.0; 4];
        for (coord, row) in coords.iter_mut().zip(self.elements.iter()) {
            *coord = Vec4 { coords: *row }.dot(&rhs);
        }
        Vec4 { coords }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elements_are_in_expected_locations() {
        let target = Mat4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        );
        assert!(target.get_element(0, 0) == 1.0);
        assert!(target.get_element(0, 1) == 2.0);
        assert!(target.get_element(0, 2) == 3.0);
        assert!(target.get_element(0, 3) == 4.0);
        assert!(target.get_element(1, 0) == 5.0);
        assert!(target.get_element(1, 1) == 6.0);
        assert!(target.get_element(1, 2) == 7.0);
        assert!(target.get_element(1, 3) == 8.0);
        assert!(target.get_element(2, 0) == 9.0);
        assert!(target.get_element(2, 1) == 10.0);
        assert!(target.get_element(2, 2) == 11.0);
        assert!(target.get_element(2, 3) == 12.0);
        assert!(target.get_element(3, 0) == 13.0);
        assert!(target.get_element(3, 1) == 14.0);
        assert!(target.get_element(3, 2) == 15.0);
        assert!(target.get_element(3, 3) == 16.0);
    }

    #[test]
    fn from_rows_places_values_in_rows() {
        let target = Mat4::from_rows(
            &Vec4::new(1.0, 2.0, 3.0, 4.0),
            &Vec4::new(5.0, 6.0, 7.0, 8.0),
            &Vec4::new(9.0, 10.0, 11.0, 12.0),
            &Vec4::new(13.0, 14.0, 15.0, 16.0),
        );
        assert!(target.get_element(0, 0) == 1.0);
        assert!(target.get_element(0, 1) == 2.0);
        assert!(target.get_element(0, 2) == 3.0);
        assert!(target.get_element(0, 3) == 4.0);
        assert!(target.get_element(1, 0) == 5.0);
        assert!(target.get_element(1, 1) == 6.0);
        assert!(target.get_element(1, 2) == 7.0);
        assert!(target.get_element(1, 3) == 8.0);
        assert!(target.get_element(2, 0) == 9.0);
        assert!(target.get_element(2, 1) == 10.0);
        assert!(target.get_element(2, 2) == 11.0);
        assert!(target.get_element(2, 3) == 12.0);
        assert!(target.get_element(3, 0) == 13.0);
        assert!(target.get_element(3, 1) == 14.0);
        assert!(target.get_element(3, 2) == 15.0);
        assert!(target.get_element(3, 3) == 16.0);
    }

    #[test]
    fn get_column_returns_expected_value() {
        let target = Mat4::from_rows(
            &Vec4::new(1.0, 2.0, 3.0, 4.0),
            &Vec4::new(5.0, 6.0, 7.0, 8.0),
            &Vec4::new(9.0, 10.0, 11.0, 12.0),
            &Vec4::new(13.0, 14.0, 15.0, 16.0),
        );
        assert!(target.get_column(0) == Vec4::new(1.0, 5.0, 9.0, 13.0));
        assert!(target.get_column(1) == Vec4::new(2.0, 6.0, 10.0, 14.0));
        assert!(target.get_column(2) == Vec4::new(3.0, 7.0, 11.0, 15.0));
        assert!(target.get_column(3) == Vec4::new(4.0, 8.0, 12.0, 16.0));
    }

    #[test]
    fn mul_with_mat4_returns_expected_result() {
        let a = Mat4::from_rows(
            &Vec4::new(1.0, 2.0, 3.0, 4.0),
            &Vec4::new(5.0, 6.0, 7.0, 8.0),
            &Vec4::new(9.0, 10.0, 11.0, 12.0),
            &Vec4::new(13.0, 14.0, 15.0, 16.0),
        );
        let b = Mat4::from_rows(
            &Vec4::new(17.0, 18.0, 19.0, 20.0),
            &Vec4::new(21.0, 22.0, 23.0, 24.0),
            &Vec4::new(25.0, 26.0, 27.0, 28.0),
            &Vec4::new(29.0, 30.0, 31.0, 32.0),
        );
        let c = Mat4::from_rows(
            &Vec4::new(250.0, 260.0, 270.0, 280.0),
            &Vec4::new(618.0, 644.0, 670.0, 696.0),
            &Vec4::new(986.0, 1028.0, 1070.0, 1112.0),
            &Vec4::new(1354.0, 1412.0, 1470.0, 1528.0),
        );
        assert!(a * b == c);
    }

    #[test]
    fn mul_assign_returns_expected_result() {
        let mut a = Mat4::from_rows(
            &Vec4::new(1.0, 2.0, 3.0, 4.0),
            &Vec4::new(5.0, 6.0, 7.0, 8.0),
            &Vec4::new(9.0, 10.0, 11.0, 12.0),
            &Vec4::new(13.0, 14.0, 15.0, 16.0),
        );
        let b = Mat4::from_rows(
            &Vec4::new(17.0, 18.0, 19.0, 20.0),
            &Vec4::new(21.0, 22.0, 23.0, 24.0),
            &Vec4::new(25.0, 26.0, 27.0, 28.0),
            &Vec4::new(29.0, 30.0, 31.0, 32.0),
        );
        let c = Mat4::from_rows(
            &Vec4::new(250.0, 260.0, 270.0, 280.0),
            &Vec4::new(618.0, 644.0, 670.0, 696.0),
            &Vec4::new(986.0, 1028.0, 1070.0, 1112.0),
            &Vec4::new(1354.0, 1412.0, 1470.0, 1528.0),
        );

        a *= b;
        assert!(a == c);
    }

    #[test]
    fn mul_with_vec4_returns_expected_result() {
        let a = Mat4::from_rows(
            &Vec4::new(1.0, 2.0, 3.0, 4.0),
            &Vec4::new(5.0, 6.0, 7.0, 8.0),
            &Vec4::new(9.0, 10.0, 11.0, 12.0),
            &Vec4::new(13.0, 14.0, 15.0, 16.0),
        );
        let b = Vec4::new(17.0, 18.0, 19.0, 20.0);
        let c = Vec4::new(190.0, 486.0, 782.0, 1078.0);
        assert!(a * b == c);
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Mat2 {
    pub elements: [[f64; 2]; 2],
}

impl Mat2 {
    pub fn new(m00: f64, m01: f64, m10: f64, m11: f64) -> Mat2 {
        Mat2 {
            elements: [[m00, m01], [m10, m11]],
        }
    }

    pub fn determinant(&self) -> f64 {
        self.elements[0][0] * self.elements[1][1] - self.elements[0][1] * self.elements[1][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determinant_returns_expected_value() {
        let target1 = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let target2 = Mat2::new(1.0, -2.0, 3.0, 4.0);
        let target3 = Mat2::new(1.0, 1.0, 1.0, 1.0);
        let target4 = Mat2::new(21.0, 45.0, -16.0, 0.0);
        assert!(target1.determinant() == -2.0);
        assert!(target2.determinant() == 10.0);
        assert!(target3.determinant() == 0.0);
        assert!(target4.determinant() == 720.0);
    }
}

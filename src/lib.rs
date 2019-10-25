use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Vector2D<T>(T, T);

impl<T: Add> Add for Vector2D<T> {
    type Output = Vector2D<T::Output>;

    fn add(self, other: Vector2D<T>) -> Vector2D<T::Output> {
        Vector2D(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: Sub> Sub for Vector2D<T> {
    type Output = Vector2D<T::Output>;

    fn sub(self, other: Vector2D<T>) -> Vector2D<T::Output> {
        Vector2D(self.0 - other.0, self.1 - other.1)
    }
}

impl<T: Mul + Copy> Mul<T> for Vector2D<T> {
    type Output = Vector2D<T::Output>;

    fn mul(self, other: T) -> Vector2D<T::Output> {
        Vector2D(self.0 * other, self.1 * other)
    }
}

impl Mul<Vector2D<f64>> for f64 {
    type Output = Vector2D<f64>;

    fn mul(self, other: Vector2D<f64>) -> Vector2D<f64> {
        Vector2D(self * other.0, self * other.1)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Vector3D<T>(T, T, T);

impl<T: Add> Add for Vector3D<T> {
    type Output = Vector3D<T::Output>;

    fn add(self, other: Vector3D<T>) -> Vector3D<T::Output> {
        Vector3D(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl<T: Sub> Sub for Vector3D<T> {
    type Output = Vector3D<T::Output>;

    fn sub(self, other: Vector3D<T>) -> Vector3D<T::Output> {
        Vector3D(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl<T: Mul + Copy> Mul<T> for Vector3D<T> {
    type Output = Vector3D<T::Output>;

    fn mul(self, other: T) -> Vector3D<T::Output> {
        Vector3D(self.0 * other, self.1 * other, self.2 * other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2_add_zeroes_yields_zeroes() {
        let a = Vector2D(0.0, 0.0);
        let b = Vector2D(0.0, 0.0);
        let c = a + b;
        assert!(c.0 == 0.0);
        assert!(c.1 == 0.0);
    }

    #[test]
    fn test_vector2d_addition_identity() {
        let id = Vector2D(0.0, 0.0);
        let a = Vector2D(1.0, 2.0);
        {
            let c = a + id;
            assert!(c.0 == a.0);
            assert!(c.1 == a.1);
        }
        {
            let d = id + a;
            assert!(d.0 == a.0);
            assert!(d.1 == a.1);
        }
    }

    #[test]
    fn test_vector2d_addition_float() {
        let a = Vector2D(1.0, 2.0);
        let b = Vector2D(4.0, 3.0);
        let c = Vector2D(-1.0, -5.5);
        let d = Vector2D(1.0, -2.0);
        {
            let r = a + b;
            assert!(r.0 == a.0 + b.0);
            assert!(r.1 == a.1 + b.1);
        }
        {
            let r = b + a;
            assert!(r.0 == a.0 + b.0);
            assert!(r.1 == a.1 + b.1);
        }
        {
            let r = b + c;
            assert!(r.0 == b.0 + c.0);
            assert!(r.1 == b.1 + c.1);
        }
        {
            let r = c + b;
            assert!(r.0 == b.0 + c.0);
            assert!(r.1 == b.1 + c.1);
        }
        {
            let r = c + d;
            assert!(r.0 == c.0 + d.0);
            assert!(r.1 == c.1 + d.1);
        }
        {
            let r = d + c;
            assert!(r.0 == c.0 + d.0);
            assert!(r.1 == c.1 + d.1);
        }
    }

    #[test]
    fn test_vector2d_addition_int() {
        let a = Vector2D(1, 2);
        let b = Vector2D(4, 3);
        let c = Vector2D(-1, -5);
        let d = Vector2D(1, -2);
        {
            let r = a + b;
            assert!(r.0 == a.0 + b.0);
            assert!(r.1 == a.1 + b.1);
        }
        {
            let r = b + a;
            assert!(r.0 == a.0 + b.0);
            assert!(r.1 == a.1 + b.1);
        }
        {
            let r = b + c;
            assert!(r.0 == b.0 + c.0);
            assert!(r.1 == b.1 + c.1);
        }
        {
            let r = c + b;
            assert!(r.0 == b.0 + c.0);
            assert!(r.1 == b.1 + c.1);
        }
        {
            let r = c + d;
            assert!(r.0 == c.0 + d.0);
            assert!(r.1 == c.1 + d.1);
        }
        {
            let r = d + c;
            assert!(r.0 == c.0 + d.0);
            assert!(r.1 == c.1 + d.1);
        }
    }

    #[test]
    fn test_vector2_subtrace_zeroes_yields_zeroes() {
        let a = Vector2D(0.0, 0.0);
        let b = Vector2D(0.0, 0.0);
        let c = a - b;
        assert!(c.0 == 0.0);
        assert!(c.1 == 0.0);
    }

    #[test]
    fn test_vector2d_subtraction_identity() {
        let id = Vector2D(0.0, 0.0);
        let a = Vector2D(1.0, 2.0);
        let c = a - id;
        assert!(c.0 == a.0);
        assert!(c.1 == a.1);
    }

    #[test]
    fn test_vector2d_subtraction_float() {
        let a = Vector2D(1.0, 2.0);
        let b = Vector2D(4.0, 3.0);
        let c = Vector2D(-1.0, -5.5);
        let d = Vector2D(1.0, -2.0);
        {
            let r = a - b;
            assert!(r.0 == a.0 - b.0);
            assert!(r.1 == a.1 - b.1);
        }
        {
            let r = b - a;
            assert!(r.0 == b.0 - a.0);
            assert!(r.1 == b.1 - a.1);
        }
        {
            let r = b - c;
            assert!(r.0 == b.0 - c.0);
            assert!(r.1 == b.1 - c.1);
        }
        {
            let r = c - b;
            assert!(r.0 == c.0 - b.0);
            assert!(r.1 == c.1 - b.1);
        }
        {
            let r = c - d;
            assert!(r.0 == c.0 - d.0);
            assert!(r.1 == c.1 - d.1);
        }
        {
            let r = d - c;
            assert!(r.0 == d.0 - c.0);
            assert!(r.1 == d.1 - c.1);
        }
    }

    #[test]
    fn test_vector2d_subrtaction_int() {
        let a = Vector2D(1, 2);
        let b = Vector2D(4, 3);
        let c = Vector2D(-1, -5);
        let d = Vector2D(1, -2);
        {
            let r = a - b;
            assert!(r.0 == a.0 - b.0);
            assert!(r.1 == a.1 - b.1);
        }
        {
            let r = b - a;
            assert!(r.0 == b.0 - a.0);
            assert!(r.1 == b.1 - a.1);
        }
        {
            let r = b - c;
            assert!(r.0 == b.0 - c.0);
            assert!(r.1 == b.1 - c.1);
        }
        {
            let r = c - b;
            assert!(r.0 == c.0 - b.0);
            assert!(r.1 == c.1 - b.1);
        }
        {
            let r = c - d;
            assert!(r.0 == c.0 - d.0);
            assert!(r.1 == c.1 - d.1);
        }
        {
            let r = d - c;
            assert!(r.0 == d.0 - c.0);
            assert!(r.1 == d.1 - c.1);
        }
    }

    #[test]
    fn test_vector2_multiply_zeroes_yields_zeroes() {
        let a = Vector2D(0.0, 0.0);
        let b = 0.0;
        let c = a * b;
        assert!(c.0 == 0.0);
        assert!(c.1 == 0.0);
    }

    #[test]
    fn test_vector2_multiplication_by_zero_yields_zeroes() {
        let a = Vector2D(1.0, 2.0);
        let b = 0.0;
        let c = a * b;
        assert!(c.0 == 0.0);
        assert!(c.1 == 0.0);
    }

    #[test]
    fn test_vector2_multiplication_identity() {
        let a = Vector2D(1.0, 2.0);
        let b = 1.0;
        let c = a * b;
        assert!(c.0 == a.0);
        assert!(c.1 == a.1);
    }

    #[test]
    fn test_vector2_multiplication_by_positive_float() {
        let a = Vector2D(1.0, 2.0);
        let b = 2.0;
        let c = a * b;
        assert!(c.0 == a.0 * 2.0);
        assert!(c.1 == a.1 * 2.0);
    }

    #[test]
    fn test_vector2_multiplication_by_negative_float() {
        let a = Vector2D(1.0, 2.0);
        let b = -2.0;
        let c = a * b;
        assert!(c.0 == a.0 * -2.0);
        assert!(c.1 == a.1 * -2.0);
    }

    #[test]
    fn test_vector2_multiplication_by_positive_int() {
        let a = Vector2D(1, 2);
        let b = 2;
        let c = a * b;
        assert!(c.0 == a.0 * 2);
        assert!(c.1 == a.1 * 2);
    }

    #[test]
    fn test_vector2_multiplication_by_negative_int() {
        let a = Vector2D(1, 2);
        let b = -2;
        let c = a * b;
        assert!(c.0 == a.0 * -2);
        assert!(c.1 == a.1 * -2);
    }

    #[test]
    fn test_vector2_multiply_zeroes_yields_zeroes_scalar_first() {
        let a = Vector2D(0.0, 0.0);
        let b = 0.0;
        let c = b * a;
        assert!(c.0 == 0.0);
        assert!(c.1 == 0.0);
    }

    #[test]
    fn test_vector2_multiply_zero_byvector2d_yields_zeroes() {
        let a = Vector2D(1.0, 2.0);
        let b = 0.0;
        let c = b * a;
        assert!(c.0 == 0.0);
        assert!(c.1 == 0.0);
    }

    #[test]
    fn test_vector2_multiplication_identity_scalar_first() {
        let a = Vector2D(1.0, 2.0);
        let b = 1.0;
        let c = b * a;
        assert!(c.0 == a.0);
        assert!(c.1 == a.1);
    }

    #[test]
    fn test_vector2_multiply_positive_float_by_vector2d() {
        let a = Vector2D(1.0, 2.0);
        let b = 2.0;
        let c = b * a;
        assert!(c.0 == a.0 * 2.0);
        assert!(c.1 == a.1 * 2.0);
    }

    #[test]
    fn test_vector2_multiply_negative_float_by_vector2d() {
        let a = Vector2D(1.0, 2.0);
        let b = -2.0;
        let c = b * a;
        assert!(c.0 == a.0 * -2.0);
        assert!(c.1 == a.1 * -2.0);
    }

    #[test]
    fn test_vector3d_add_zeroes_yields_zeroes() {
        let a = Vector3D(0.0, 0.0, 0.0);
        let b = Vector3D(0.0, 0.0, 0.0);
        let c = a + b;
        assert!(c.0 == 0.0);
        assert!(c.1 == 0.0);
        assert!(c.2 == 0.0);
    }

    #[test]
    fn test_vector3d_addition_identity() {
        let id = Vector3D(0.0, 0.0, 0.0);
        let a = Vector3D(1.0, 2.0, 3.0);
        {
            let c = a + id;
            assert!(c.0 == a.0);
            assert!(c.1 == a.1);
            assert!(c.2 == a.2);
        }
        {
            let d = id + a;
            assert!(d.0 == a.0);
            assert!(d.1 == a.1);
            assert!(d.2 == a.2);
        }
    }

    #[test]
    fn test_vector3d_addition_float() {
        let a = Vector3D(1.0, 2.0, 1.5);
        let b = Vector3D(4.0, 3.0, 2.0);
        let c = Vector3D(-1.0, -5.5, 3.0);
        let d = Vector3D(1.0, -2.0, 1.0);
        {
            let r = a + b;
            assert!(r.0 == a.0 + b.0);
            assert!(r.1 == a.1 + b.1);
            assert!(r.2 == a.2 + b.2);
        }
        {
            let r = b + a;
            assert!(r.0 == a.0 + b.0);
            assert!(r.1 == a.1 + b.1);
            assert!(r.2 == a.2 + b.2);
        }
        {
            let r = b + c;
            assert!(r.0 == b.0 + c.0);
            assert!(r.1 == b.1 + c.1);
            assert!(r.2 == b.2 + c.2);
        }
        {
            let r = c + b;
            assert!(r.0 == b.0 + c.0);
            assert!(r.1 == b.1 + c.1);
            assert!(r.2 == b.2 + c.2);
        }
        {
            let r = c + d;
            assert!(r.0 == c.0 + d.0);
            assert!(r.1 == c.1 + d.1);
            assert!(r.2 == c.2 + d.2);
        }
        {
            let r = d + c;
            assert!(r.0 == c.0 + d.0);
            assert!(r.1 == c.1 + d.1);
            assert!(r.2 == c.2 + d.2);
        }
    }

    #[test]
    fn test_vector3d_addition_int() {
        let a = Vector3D(1, 2, 1);
        let b = Vector3D(4, 3, 2);
        let c = Vector3D(-1, -5, 3);
        let d = Vector3D(1, -2, 1);
        {
            let r = a + b;
            assert!(r.0 == a.0 + b.0);
            assert!(r.1 == a.1 + b.1);
            assert!(r.2 == a.2 + b.2);
        }
        {
            let r = b + a;
            assert!(r.0 == a.0 + b.0);
            assert!(r.1 == a.1 + b.1);
            assert!(r.2 == a.2 + b.2);
        }
        {
            let r = b + c;
            assert!(r.0 == b.0 + c.0);
            assert!(r.1 == b.1 + c.1);
            assert!(r.2 == b.2 + c.2);
        }
        {
            let r = c + b;
            assert!(r.0 == b.0 + c.0);
            assert!(r.1 == b.1 + c.1);
            assert!(r.2 == b.2 + c.2);
        }
        {
            let r = c + d;
            assert!(r.0 == c.0 + d.0);
            assert!(r.1 == c.1 + d.1);
            assert!(r.2 == c.2 + d.2);
        }
        {
            let r = d + c;
            assert!(r.0 == c.0 + d.0);
            assert!(r.1 == c.1 + d.1);
            assert!(r.2 == c.2 + d.2);
        }
    }

    #[test]
    fn test_vector3d_subtract_zeroes_yields_zeroes() {
        let a = Vector3D(0.0, 0.0, 0.0);
        let b = Vector3D(0.0, 0.0, 0.0);
        let c = a - b;
        assert!(c.0 == 0.0);
        assert!(c.1 == 0.0);
        assert!(c.2 == 0.0);
    }

    #[test]
    fn test_vector3d_subtraction_identity() {
        let id = Vector3D(0.0, 0.0, 0.0);
        let a = Vector3D(1.0, 2.0, 3.0);
        {
            let c = a - id;
            assert!(c.0 == a.0);
            assert!(c.1 == a.1);
            assert!(c.2 == a.2);
        }
    }

    #[test]
    fn test_vector3d_subtraction_float() {
        let a = Vector3D(1.0, 2.0, 1.5);
        let b = Vector3D(4.0, 3.0, 2.0);
        let c = Vector3D(-1.0, -5.5, 3.0);
        let d = Vector3D(1.0, -2.0, 1.0);
        {
            let r = a - b;
            assert!(r.0 == a.0 - b.0);
            assert!(r.1 == a.1 - b.1);
            assert!(r.2 == a.2 - b.2);
        }
        {
            let r = b - a;
            assert!(r.0 == b.0 - a.0);
            assert!(r.1 == b.1 - a.1);
            assert!(r.2 == b.2 - a.2);
        }
        {
            let r = b - c;
            assert!(r.0 == b.0 - c.0);
            assert!(r.1 == b.1 - c.1);
            assert!(r.2 == b.2 - c.2);
        }
        {
            let r = c - b;
            assert!(r.0 == c.0 - b.0);
            assert!(r.1 == c.1 - b.1);
            assert!(r.2 == c.2 - b.2);
        }
        {
            let r = c - d;
            assert!(r.0 == c.0 - d.0);
            assert!(r.1 == c.1 - d.1);
            assert!(r.2 == c.2 - d.2);
        }
        {
            let r = d - c;
            assert!(r.0 == d.0 - c.0);
            assert!(r.1 == d.1 - c.1);
            assert!(r.2 == d.2 - c.2);
        }
    }

    #[test]
    fn test_vector3d_subtraction_int() {
        let a = Vector3D(1, 2, 1);
        let b = Vector3D(4, 3, 2);
        let c = Vector3D(-1, -5, 3);
        let d = Vector3D(1, -2, 1);
        {
            let r = a - b;
            assert!(r.0 == a.0 - b.0);
            assert!(r.1 == a.1 - b.1);
            assert!(r.2 == a.2 - b.2);
        }
        {
            let r = b - a;
            assert!(r.0 == b.0 - a.0);
            assert!(r.1 == b.1 - a.1);
            assert!(r.2 == b.2 - a.2);
        }
        {
            let r = b - c;
            assert!(r.0 == b.0 - c.0);
            assert!(r.1 == b.1 - c.1);
            assert!(r.2 == b.2 - c.2);
        }
        {
            let r = c - b;
            assert!(r.0 == c.0 - b.0);
            assert!(r.1 == c.1 - b.1);
            assert!(r.2 == c.2 - b.2);
        }
        {
            let r = c - d;
            assert!(r.0 == c.0 - d.0);
            assert!(r.1 == c.1 - d.1);
            assert!(r.2 == c.2 - d.2);
        }
        {
            let r = d - c;
            assert!(r.0 == d.0 - c.0);
            assert!(r.1 == d.1 - c.1);
            assert!(r.2 == d.2 - c.2);
        }
    }

    #[test]
    fn test_vector3_multiply_zeroes_yields_zeroes() {
        let a = Vector3D(0.0, 0.0, 0.0);
        let b = 0.0;
        let c = a * b;
        assert!(c.0 == 0.0);
        assert!(c.1 == 0.0);
        assert!(c.2 == 0.0);
    }

    #[test]
    fn test_vector3_multiplication_by_zero_yields_zeroes() {
        let a = Vector3D(1.0, 2.0, 3.0);
        let b = 0.0;
        let c = a * b;
        assert!(c.0 == 0.0);
        assert!(c.1 == 0.0);
        assert!(c.2 == 0.0);
    }

    #[test]
    fn test_vector3_multiplication_identity() {
        let a = Vector3D(1.0, 2.0, 3.0);
        let b = 1.0;
        let c = a * b;
        assert!(c == a);
    }

    #[test]
    fn test_vector3_multiplication_by_positive_float() {
        let a = Vector3D(1.0, 2.0, 3.0);
        let b = 2.0;
        let c = a * b;
        assert!(c.0 == a.0 * 2.0);
        assert!(c.1 == a.1 * 2.0);
        assert!(c.2 == a.2 * 2.0);
    }

    #[test]
    fn test_vector3_multiplication_by_negative_float() {
        let a = Vector3D(1.0, 2.0, 3.0);
        let b = -2.0;
        let c = a * b;
        assert!(c.0 == a.0 * -2.0);
        assert!(c.1 == a.1 * -2.0);
        assert!(c.2 == a.2 * -2.0);
    }

    #[test]
    fn test_vector3_multiplication_by_positive_int() {
        let a = Vector3D(1, 2, 3);
        let b = 2;
        let c = a * b;
        assert!(c.0 == a.0 * 2);
        assert!(c.1 == a.1 * 2);
        assert!(c.2 == a.2 * 2);
    }

    #[test]
    fn test_vector3_multiplication_by_negative_int() {
        let a = Vector3D(1, 2, 3);
        let b = -2;
        let c = a * b;
        assert!(c.0 == a.0 * -2);
        assert!(c.1 == a.1 * -2);
        assert!(c.2 == a.2 * -2);
    }
}

use std::ops::Add;

#[derive(Clone, Copy)]
struct Vector2D<T>(T,T);

impl<T: Add> Add for Vector2D<T> {
    type Output = Vector2D<T::Output>;

    fn add(self, other: Vector2D<T>) -> Vector2D<T::Output> {
        Vector2D(
            self.0 + other.0,
            self.1 + other.1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_zeroes_yields_zeroes() {
        let a = Vector2D(0.0, 0.0);
        let b = Vector2D(0.0, 0.0);
        let c = a + b;
        assert!(c.0 == 0.0);
        assert!(c.0 == 0.0);
    }

    #[test]
    fn test_addition_identity() {
        let id = Vector2D(0.0, 0.0 );
        let a = Vector2D(1.0, 2.0 );
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
}

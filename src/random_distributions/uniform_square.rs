use rand::distributions::Open01;
use rand::{thread_rng, Rng};

use crate::math::Vec2;

use super::RandomDistribution;

#[derive(Debug)]
pub struct UniformSquare {
    corner: Vec2,
    size: f64,
}

impl UniformSquare {
    pub fn new(corner: Vec2, size: f64) -> UniformSquare {
        UniformSquare { corner, size }
    }
}

impl RandomDistribution<Vec2> for UniformSquare {
    fn value(&self) -> Vec2 {
        let mut rng = thread_rng();
        self.corner
            + Vec2::new(rng.sample::<f64, _>(Open01), rng.sample::<f64, _>(Open01)) * self.size
    }

    fn pdf(&self, _value: Vec2) -> f64 {
        1.0 / (self.size * self.size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn print_values() {
        let target = UniformSquare {
            corner: Vec2::new(1.5, -2.5),
            size: 3.0,
        };
        for _ in 0..1000 {
            let value = target.value();
            println!("{}, {}", value.x(), value.y());
        }
    }

    #[test]
    #[ignore]
    fn integral_is_near_area() {
        let target = UniformSquare {
            corner: Vec2::new(1.5, -2.5),
            size: 3.0,
        };
        let integral = (0..1000)
            .map(|_| target.value())
            .map(|value| 1.0 / target.pdf(value))
            .sum::<f64>()
            / 1000.0;
        println!("Area: {}\nIntegral: {}", 3.0 * 3.0, integral);
    }
}

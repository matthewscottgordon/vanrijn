use std::f64::consts::PI;

use crate::math::Vec2;

use super::{RandomDistribution, UniformSquare};

#[derive(Debug)]
pub struct UnitDisc {
    square_distribution: UniformSquare,
}

impl UnitDisc {
    pub fn new() -> UnitDisc {
        let square_distribution = UniformSquare::new(Vec2::new(-1.0, -1.0), 2.0);
        UnitDisc {
            square_distribution,
        }
    }
}

impl RandomDistribution<Vec2> for UnitDisc {
    fn value(&self) -> Vec2 {
        let offset = self.square_distribution.value();
        if offset.x() == 0.0 && offset.y() == 0.0 {
            offset
        } else {
            let (radius, angle) = if offset.x().abs() > offset.y().abs() {
                (offset.x(), (PI / 4.0) * offset.y() / offset.x())
            } else {
                (offset.y(), PI / 2.0 - (PI / 4.0) * offset.x() / offset.y())
            };
            Vec2::new(angle.cos(), angle.sin()) * radius
        }
    }

    fn pdf(&self, _: Vec2) -> f64 {
        1.0 / PI
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn print_values() {
        let target = UnitDisc::new();
        for _ in 0..1000 {
            let value = target.value();
            println!("{}, {}", value.x(), value.y());
        }
    }

    #[test]
    #[ignore]
    fn integral_is_near_area() {
        let target = UnitDisc::new();
        let integral = (0..1000)
            .map(|_| target.value())
            .map(|value| 1.0 / target.pdf(value))
            .sum::<f64>()
            / 1000.0;
        println!("Area: {}\nIntegral: {}", PI, integral);
    }
}

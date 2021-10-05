use std::f64::consts::PI;

use crate::math::Vec3;

use super::{RandomDistribution, UnitDisc};

pub struct CosineWeightedHemisphere {
    unit_disc: UnitDisc,
}

impl CosineWeightedHemisphere {
    pub fn new() -> CosineWeightedHemisphere {
        let unit_disc = UnitDisc::new();
        CosineWeightedHemisphere { unit_disc }
    }
}

impl RandomDistribution<Vec3> for CosineWeightedHemisphere {
    fn value(&self) -> Vec3 {
        let point_on_disc = self.unit_disc.value();
        let z = 0.0f64
            .max(
                1.0 - point_on_disc.x() * point_on_disc.x() - point_on_disc.y() * point_on_disc.y(),
            )
            .sqrt();
        Vec3::new(point_on_disc.x(), point_on_disc.y(), z)
    }

    fn pdf(&self, v: Vec3) -> f64 {
        (v.x() * v.x() + v.y() * v.y()).sqrt() / PI
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn print_values() {
        let target = CosineWeightedHemisphere::new();
        for _ in 0..1000 {
            let value = target.value();
            println!("{}, {}, {}", value.x(), value.y(), value.z());
        }
    }

    #[test]
    #[ignore]
    fn integral_is_near_area() {
        let target = CosineWeightedHemisphere::new();
        let integral = (0..100000)
            .map(|_| target.value())
            .map(|value| 1.0 / target.pdf(value))
            .sum::<f64>()
            / 100000.0;
        println!("Area: {}\nIntegral: {}", 2.0 * PI, integral);
    }
}

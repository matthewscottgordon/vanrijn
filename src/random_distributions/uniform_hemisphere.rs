use std::f64::consts::PI;

use rand::distributions::{Open01, OpenClosed01};
use rand::{thread_rng, Rng};

use crate::math::Vec3;

use super::RandomDistribution;

#[derive(Default)]
pub struct UniformHemisphere {}

impl UniformHemisphere {
    pub fn new() -> UniformHemisphere {
        UniformHemisphere {}
    }
}

impl RandomDistribution<Vec3> for UniformHemisphere {
    fn value(&self) -> Vec3 {
        let mut rng = thread_rng();
        let mut result = Vec3::new(
            2.0 * rng.sample::<f64, _>(Open01) - 1.0,
            2.0 * rng.sample::<f64, _>(Open01) - 1.0,
            rng.sample::<f64, _>(OpenClosed01),
        );
        while result.norm_squared() > 1.0 {
            result = Vec3::new(
                2.0 * rng.sample::<f64, _>(Open01) - 1.0,
                2.0 * rng.sample::<f64, _>(Open01) - 1.0,
                rng.sample::<f64, _>(OpenClosed01),
            );
        }
        result.normalize()
    }

    fn pdf(&self, _: Vec3) -> f64 {
        1.0 / (2.0 * PI)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn print_values() {
        let target = UniformHemisphere::new();
        for _ in 0..1000 {
            let value = target.value();
            println!("{}, {}, {}", value.x(), value.y(), value.z());
        }
    }

    #[test]
    #[ignore]
    fn integral_is_near_area() {
        let target = UniformHemisphere::new();
        let integral = (0..1000)
            .map(|_| target.value())
            .map(|value| 1.0 / target.pdf(value))
            .sum::<f64>()
            / 1000.0;
        println!("Area: {}\nIntegral: {}", 2.0 * PI, integral);
    }
}

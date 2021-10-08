use std::f64::consts::PI;

use rand::distributions::Open01;
use rand::{thread_rng, Rng};

use crate::math::Vec3;

use super::{LinearWeighted, RandomDistribution};

pub struct SkyLightPdf {
    z_distribution: LinearWeighted,
}

impl SkyLightPdf {
    pub fn new() -> SkyLightPdf {
        let z_distribution = LinearWeighted::new(1.0);
        SkyLightPdf { z_distribution }
    }
}

impl RandomDistribution<Vec3> for SkyLightPdf {
    fn value(&self) -> Vec3 {
        let mut rng = thread_rng();
        let phi = rng.sample::<f64, _>(Open01) * 2.0 * PI;
        let z = self.z_distribution.value();
        let r = (1.0 - z * z).sqrt();
        Vec3::new(r * phi.cos(), r * phi.sin(), z)
    }

    fn pdf(&self, value: Vec3) -> f64 {
        let z = value.z();
        if z < 0.0 {
            0.0
        } else {
            z / PI
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn print_values() {
        let target = SkyLightPdf::new();
        for _ in 0..1000 {
            let value = target.value();
            println!("{}, {}, {}", value.x(), value.y(), value.z());
        }
    }

    #[test]
    #[ignore]
    fn integral_is_near_area() {
        let target = SkyLightPdf::new();
        let integral = (0..100000)
            .map(|_| target.value())
            .map(|value| 1.0 / target.pdf(value))
            .sum::<f64>()
            / 100000.0;
        println!("Area: {}\nIntegral: {}", 2.0 * PI, integral);
    }
}

use rand::distributions::Open01;
use rand::{thread_rng, Rng};

use super::RandomDistribution;

pub struct LinearWeighted {
    max_value: f64,
}

impl LinearWeighted {
    pub fn new(max_value: f64) -> LinearWeighted {
        LinearWeighted { max_value }
    }
}

impl RandomDistribution<f64> for LinearWeighted {
    fn value(&self) -> f64 {
        let mut rng = thread_rng();
        rng.sample::<f64, _>(Open01).sqrt() * self.max_value
    }

    fn pdf(&self, value: f64) -> f64 {
        2.0 * value / (self.max_value * self.max_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn print_values() {
        let target = LinearWeighted::new(2.0);
        for _ in 0..1000 {
            let value = target.value();
            println!("{}", value);
        }
    }

    #[test]
    #[ignore]
    fn print_buckets() {
        let mut buckets = [0; 20];
        let target = LinearWeighted::new(20.0);
        for _ in 0..10000 {
            let value = target.value();
            let i = value as usize;
            buckets[i] += 1;
        }
        for count in buckets {
            println!("{}", count);
        }
    }

    #[test]
    #[ignore]
    fn integral_is_near_area() {
        let target = LinearWeighted::new(2.0);
        let integral = (0..100000)
            .map(|_| target.value())
            .map(|value| 1.0 / target.pdf(value))
            .sum::<f64>()
            / 100000.0;
        println!("Area: {}\nIntegral: {}", 2.0, integral);
    }
}

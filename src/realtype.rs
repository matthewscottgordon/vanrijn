use simba::scalar::{SubsetOf,SupersetOf};
use nalgebra::RealField;

pub trait NormalizedToU32 {
    fn normalized_to_u32(self, num_bits: usize) -> u32;
}

pub trait Real: RealField + SupersetOf<f32> + SubsetOf<f32> + NormalizedToU32 {}

impl NormalizedToU32 for f32 {
    fn normalized_to_u32(self, num_bits: usize) -> u32 {
        let scale = (num_bits as f32).exp2() - 1.0;
        (self * scale) as u32
    }
}

impl NormalizedToU32 for f64 {
    fn normalized_to_u32(self, num_bits: usize) -> u32 {
        let scale = (num_bits as f64).exp2() - 1.0;
        (self * scale) as u32
    }
}

impl Real for f32 {}
impl Real for f64 {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero_f32_normalizes_to_zero() {
        let target = 0.0f32;
        assert!(target.normalized_to_u32(10) == 0);
    }

    #[test]
    fn one_f32_normalizes_to_all_ones() {
        let target = 1.0f32;
        assert!(target.normalized_to_u32(10) == 0b1111111111);
    }

    #[test]
    fn half_f32_normalizes_to_half_value() {
        let target = 0.5f32;
        assert!(target.normalized_to_u32(10) == 511);
    }

    #[test]
    fn zero_f64_normalizes_to_zero() {
        let target = 0.0f64;
        assert!(target.normalized_to_u32(10) == 0);
    }

    #[test]
    fn one_f64_normalizes_to_all_ones() {
        let target = 1.0f64;
        assert!(target.normalized_to_u32(10) == 0b1111111111);
    }

    #[test]
    fn half_f64_normalizes_to_half_value() {
        let target = 0.5f64;
        assert!(target.normalized_to_u32(10) == 511);
    }
}

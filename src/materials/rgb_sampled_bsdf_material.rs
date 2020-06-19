use nalgebra::{convert, Vector3};

use super::{Bsdf, Material};
use crate::colour::ColourRgbF;
use crate::Real;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use std::f64::consts::{FRAC_PI_2, PI};

#[derive(Debug)]
pub struct RgbSampledBsdfMaterial<T: Real> {
    lut: Arc<Vec<Vec<Vec<Vec<Vector3<T>>>>>>,
}

fn expand_and_index<T: Clone>(v: &mut Vec<T>, i: usize, default: T) -> &mut T {
    if v.len() < i + 1 {
        v.resize(i + 1, default);
    }
    &mut v[i]
}

impl<T: Real> RgbSampledBsdfMaterial<T> {
    pub fn from_csv_file(filename: &str) -> Result<RgbSampledBsdfMaterial<T>, Box<dyn Error>> {
        let csv_file = File::open(filename)?;
        let mut reader = csv::Reader::from_reader(BufReader::new(&csv_file));
        let mut lut = Vec::new();
        for row_result in reader.records() {
            let row = row_result?;
            let theta_in_index = row[0].trim().parse::<usize>()?;
            let phi_in_index = row[1].trim().parse::<usize>()?;
            let theta_out_index = row[2].trim().parse::<usize>()?;
            let phi_out_index = row[3].trim().parse::<usize>()?;
            let red = row[4].trim().parse::<f64>()?;
            let green = row[5].trim().parse::<f64>()?;
            let blue = row[6].trim().parse::<f64>()?;
            *expand_and_index(
                expand_and_index(
                    expand_and_index(
                        expand_and_index(&mut lut, theta_in_index, Vec::new()),
                        phi_in_index,
                        Vec::new(),
                    ),
                    theta_out_index,
                    Vec::new(),
                ),
                phi_out_index,
                Vector3::zeros(),
            ) = Vector3::new(convert(red), convert(green), convert(blue));
        }
        let lut = Arc::new(lut);
        Ok(RgbSampledBsdfMaterial { lut })
    }
}

impl<'a, T: Real> Material<T> for RgbSampledBsdfMaterial<T> {
    fn bsdf(&self) -> Bsdf<T> {
        let lut = Arc::clone(&self.lut);
        Box::new(move |w_in, w_out, colour_in| {
            if w_in.z < T::zero() || w_out.z < T::zero() {
                return ColourRgbF::new(T::zero(), T::zero(), T::zero());
            }
            let theta_in = w_in.z.acos();
            let theta_in_index = (theta_in / convert(FRAC_PI_2)).normalized_to_u32(4) as usize;
            let phi_in = w_in.y.atan2(w_in.x) + convert(PI);
            let phi_in_index = (phi_in / convert(2.0 * PI)).normalized_to_u32(6) as usize;
            let theta_out = w_out.z.acos();
            let theta_out_index = (theta_out / convert(FRAC_PI_2)).normalized_to_u32(4) as usize;
            let phi_out = w_out.y.atan2(w_out.x) + convert(PI);
            let phi_out_index = (phi_out / convert(2.0 * PI)).normalized_to_u32(6) as usize;
            ColourRgbF::from_vector3(
                &colour_in.as_vector3().component_mul(
                    &lut[theta_in_index][phi_in_index][theta_out_index][phi_out_index],
                ),
            )
        })
    }

    fn sample(&self, w_o: &Vector3<T>) -> Vec<Vector3<T>> {
        vec![Vector3::new(-w_o.x, -w_o.y, w_o.z)]
    }
}

fn find_before_and_after<'a, T: Real, C>(
    value: T,
    vec: &'a Vec<(T, C)>,
) -> (&'a (T, C), &'a (T, C)) {
    let first = vec.first().unwrap();
    let (lowest, _) = first;
    if value < *lowest {
        (first, first)
    } else {
        vec.iter()
            .zip(vec.iter().skip(1))
            .find(|((value1, _), (value2, _))| value1 <= &value && value2 > &value)
            .unwrap_or((vec.last().unwrap(), vec.last().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod find_before_and_after {
        use super::*;

        #[test]
        fn returns_element_before_value() {
            let test_data = vec![(10.0, 1), (15.0, 2), (20.0, 3), (25.0, 4), (30.0, 5)];
            let ((low_first, low_second), (_, _)) = find_before_and_after(23.0, &test_data);
            assert!(*low_first == 20.0);
            assert!(*low_second == 3);
        }

        #[test]
        fn returns_element_after_value() {
            let test_data = vec![(10.0, 1), (15.0, 2), (20.0, 3), (25.0, 4), (30.0, 5)];
            let ((_, _), (high_first, high_second)) = find_before_and_after(23.0, &test_data);
            assert!(*high_first == 25.0);
            assert!(*high_second == 4);
        }

        #[test]
        fn returns_element_equal_to_value_in_first_position() {
            let test_data = vec![(10.0, 1), (15.0, 2), (20.0, 3), (25.0, 4), (30.0, 5)];
            let ((low_first, low_second), (high_first, high_second)) =
                find_before_and_after(20.0, &test_data);
            assert!(*low_first == 20.0);
            assert!(*low_second == 3);
        }

        #[test]
        fn returns_first_element_twice_when_value_less_than_first() {
            let test_data = vec![(10.0, 1), (15.0, 2), (20.0, 3), (25.0, 4), (30.0, 5)];
            let ((low_first, low_second), (high_first, high_second)) =
                find_before_and_after(5.0, &test_data);
            assert!(*low_first == 10.0);
            assert!(*low_second == 1);
            assert!(high_first == low_first);
            assert!(high_second == high_second);
        }

        #[test]
        fn returns_last_element_twice_when_value_greater_than_last() {
            let test_data = vec![(10.0, 1), (15.0, 2), (20.0, 3), (25.0, 4), (30.0, 5)];
            let ((low_first, low_second), (high_first, high_second)) =
                find_before_and_after(35.0, &test_data);
            assert!(*low_first == 30.0);
            assert!(*low_second == 5);
            assert!(high_first == low_first);
            assert!(high_second == high_second);
        }

        #[test]
        fn returns_first_two_elements_when_value_is_between_them() {
            let test_data = vec![(10.0, 1), (15.0, 2), (20.0, 3), (25.0, 4), (30.0, 5)];
            let ((low_first, low_second), (high_first, high_second)) =
                find_before_and_after(12.0, &test_data);
            assert!(*low_first == 10.0);
            assert!(*low_second == 1);
            assert!(*high_first == 15.0);
            assert!(*high_second == 2);
        }

        #[test]
        fn returns_last_two_elements_when_value_is_between_them() {
            let test_data = vec![(10.0, 1), (15.0, 2), (20.0, 3), (25.0, 4), (30.0, 5)];
            let ((low_first, low_second), (high_first, high_second)) =
                find_before_and_after(27.0, &test_data);
            assert!(*low_first == 25.0);
            assert!(*low_second == 4);
            assert!(*high_first == 30.0);
            assert!(*high_second == 5);
        }
    }
}

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

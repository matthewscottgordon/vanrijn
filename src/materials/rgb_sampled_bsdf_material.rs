use super::{Bsdf, Material};
use crate::colour::ColourRgbF;
use crate::math::Vec3;
use crate::realtype::NormalizedToU32;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use std::f64::consts::{FRAC_PI_2, PI};

#[derive(Debug)]
pub struct RgbSampledBsdfMaterial {
    lut: Arc<Vec<Vec<Vec<Vec<Vec3>>>>>,
}

fn expand_and_index<T: Clone>(v: &mut Vec<T>, i: usize, default: T) -> &mut T {
    if v.len() < i + 1 {
        v.resize(i + 1, default);
    }
    &mut v[i]
}

impl RgbSampledBsdfMaterial {
    pub fn from_csv_file(filename: &str) -> Result<RgbSampledBsdfMaterial, Box<dyn Error>> {
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
                Vec3::zeros(),
            ) = Vec3::new(red, green, blue);
        }
        let lut = Arc::new(lut);
        Ok(RgbSampledBsdfMaterial { lut })
    }
}

impl<'a> Material for RgbSampledBsdfMaterial {
    fn bsdf(&self) -> Bsdf {
        let lut = Arc::clone(&self.lut);
        Box::new(move |w_in, w_out, colour_in| {
            if w_in.z() < 0.0 || w_out.z() < 0.0 {
                return ColourRgbF::new(0.0, 0.0, 0.0);
            }
            let theta_in = w_in.z().acos();
            let theta_in_index = (theta_in / FRAC_PI_2).normalized_to_u32(4) as usize;
            let phi_in = w_in.y().atan2(w_in.x()) + PI;
            let phi_in_index = (phi_in / (2.0 * PI)).normalized_to_u32(6) as usize;
            let theta_out = w_out.z().acos();
            let theta_out_index = (theta_out / FRAC_PI_2).normalized_to_u32(4) as usize;
            let phi_out = w_out.y().atan2(w_out.x()) + PI;
            let phi_out_index = (phi_out / (2.0 * PI)).normalized_to_u32(6) as usize;
            ColourRgbF::from_vec3(
                &colour_in.as_vec3().component_mul(
                    &lut[theta_in_index][phi_in_index][theta_out_index][phi_out_index],
                ),
            )
        })
    }

    fn sample(&self, w_o: &Vec3) -> Vec<Vec3> {
        vec![Vec3::new(-w_o.x(), -w_o.y(), w_o.z())]
    }
}

use crate::colour::{ColourRgbF, Photon, LONGEST_VISIBLE_WAVELENGTH, SHORTEST_VISIBLE_WAVELENGTH};

use itertools::izip;

#[derive(Debug)]
pub struct Spectrum {
    shortest_wavelength: f64,
    longest_wavelength: f64,
    samples: Vec<f64>,
}

impl Spectrum {
    pub fn black() -> Spectrum {
        Spectrum {
            shortest_wavelength: SHORTEST_VISIBLE_WAVELENGTH,
            longest_wavelength: LONGEST_VISIBLE_WAVELENGTH,
            samples: vec![0.0, 0.0],
        }
    }

    pub fn grey(brightness: f64) -> Spectrum {
        Spectrum {
            shortest_wavelength: SHORTEST_VISIBLE_WAVELENGTH,
            longest_wavelength: LONGEST_VISIBLE_WAVELENGTH,
            samples: vec![brightness; 2],
        }
    }

    pub fn diamond_index_of_refraction() -> Spectrum {
        Spectrum {
            shortest_wavelength: 326.27,
            longest_wavelength: 774.9,
            samples: vec![
                2.505813241,
                2.487866556,
                2.473323675,
                2.464986815,
                2.455051934,
                2.441251728,
                2.431478974,
                2.427076431,
                2.420857286,
                2.411429037,
                2.406543164,
                2.406202402,
            ],
        }
    }

    fn wavelength_range(&self) -> f64 {
        self.longest_wavelength - self.shortest_wavelength
    }

    fn index_at_or_before_wavelength(&self, wavelength: f64) -> usize {
        ((self.samples.len() - 1) as f64
            * ((wavelength - self.shortest_wavelength) / self.wavelength_range())) as usize
    }

    fn wavelength_at_index(&self, index: usize) -> f64 {
        (index as f64) / ((self.samples.len() - 1) as f64) * self.wavelength_range()
            + self.shortest_wavelength
    }

    pub fn intensity_at_wavelength(&self, wavelength: f64) -> f64 {
        if wavelength < self.shortest_wavelength || wavelength > self.longest_wavelength {
            0.0
        } else {
            let index_before = self.index_at_or_before_wavelength(wavelength);
            let wavelength_before = self.wavelength_at_index(index_before);
            if index_before == self.samples.len() - 1 {
                self.samples[index_before]
            } else {
                let wavelength_after = self.wavelength_at_index(index_before + 1);
                let delta = wavelength_after - wavelength_before;
                let ratio = (wavelength - wavelength_before) / delta;
                self.samples[index_before] * (1.0 - ratio) + self.samples[index_before + 1] * ratio
            }
        }
    }

    pub fn reflection_from_linear_rgb(colour: &ColourRgbF) -> Spectrum {
        Spectrum {
            shortest_wavelength: rgb_reference_spectrum::SHORTEST_WAVELENGTH,
            longest_wavelength: rgb_reference_spectrum::LONGEST_WAVELENGTH,
            samples: if colour.red() <= colour.green() && colour.red() <= colour.blue() {
                if colour.green() <= colour.blue() {
                    izip![
                        rgb_reference_spectrum::reflection::WHITE.iter(),
                        rgb_reference_spectrum::reflection::CYAN.iter(),
                        rgb_reference_spectrum::reflection::BLUE.iter()
                    ]
                    .map(|(white, cyan, blue)| {
                        colour.red() * white
                            + (colour.green() - colour.red()) * cyan
                            + (colour.blue() - colour.green()) * blue
                    })
                    .collect()
                } else {
                    izip![
                        rgb_reference_spectrum::reflection::WHITE.iter(),
                        rgb_reference_spectrum::reflection::CYAN.iter(),
                        rgb_reference_spectrum::reflection::GREEN.iter()
                    ]
                    .map(|(white, cyan, green)| {
                        colour.red() * white
                            + (colour.blue() - colour.red()) * cyan
                            + (colour.green() - colour.blue()) * green
                    })
                    .collect()
                }
            } else if colour.green() <= colour.red() && colour.green() < colour.blue() {
                if colour.red() <= colour.blue() {
                    izip![
                        rgb_reference_spectrum::reflection::WHITE.iter(),
                        rgb_reference_spectrum::reflection::MAGENTA.iter(),
                        rgb_reference_spectrum::reflection::BLUE.iter()
                    ]
                    .map(|(white, magenta, blue)| {
                        colour.green() * white
                            + (colour.red() - colour.green()) * magenta
                            + (colour.blue() - colour.red()) * blue
                    })
                    .collect()
                } else {
                    izip![
                        rgb_reference_spectrum::reflection::WHITE.iter(),
                        rgb_reference_spectrum::reflection::MAGENTA.iter(),
                        rgb_reference_spectrum::reflection::RED.iter()
                    ]
                    .map(|(white, magenta, red)| {
                        colour.green() * white
                            + (colour.blue() - colour.green()) * magenta
                            + (colour.red() - colour.blue()) * red
                    })
                    .collect()
                }
            } else {
                if colour.red() <= colour.green() {
                    izip![
                        rgb_reference_spectrum::reflection::WHITE.iter(),
                        rgb_reference_spectrum::reflection::YELLOW.iter(),
                        rgb_reference_spectrum::reflection::GREEN.iter()
                    ]
                    .map(|(white, yellow, green)| {
                        colour.blue() * white
                            + (colour.red() - colour.blue()) * yellow
                            + (colour.green() - colour.red()) * green
                    })
                    .collect()
                } else {
                    izip![
                        rgb_reference_spectrum::reflection::WHITE.iter(),
                        rgb_reference_spectrum::reflection::YELLOW.iter(),
                        rgb_reference_spectrum::reflection::RED.iter()
                    ]
                    .map(|(white, yellow, red)| {
                        colour.blue() * white
                            + (colour.green() - colour.blue()) * yellow
                            + (colour.red() - colour.green()) * red
                    })
                    .collect()
                }
            },
        }
    }

    pub fn scale_photon(&self, photon: &Photon) -> Photon {
        let wavelength = photon.wavelength;
        photon.scale_intensity(self.intensity_at_wavelength(wavelength))
    }

    pub fn emit_photon(&self, photon: &Photon) -> Photon {
        let wavelength = photon.wavelength;
        photon.set_intensity(self.intensity_at_wavelength(wavelength))
    }
}

mod rgb_reference_spectrum {
    pub const SHORTEST_WAVELENGTH: f64 = 380.0;
    pub const LONGEST_WAVELENGTH: f64 = 720.0;
    pub mod reflection {
        pub const WHITE: [f64; 32] = [
            1.0618958571272863e+00,
            1.0615019980348779e+00,
            1.0614335379927147e+00,
            1.0622711654692485e+00,
            1.0622036218416742e+00,
            1.0625059965187085e+00,
            1.0623938486985884e+00,
            1.0624706448043137e+00,
            1.0625048144827762e+00,
            1.0624366131308856e+00,
            1.0620694238892607e+00,
            1.0613167586932164e+00,
            1.0610334029377020e+00,
            1.0613868564828413e+00,
            1.0614215366116762e+00,
            1.0620336151299086e+00,
            1.0625497454805051e+00,
            1.0624317487992085e+00,
            1.0625249140554480e+00,
            1.0624277664486914e+00,
            1.0624749854090769e+00,
            1.0625538581025402e+00,
            1.0625326910104864e+00,
            1.0623922312225325e+00,
            1.0623650980354129e+00,
            1.0625256476715284e+00,
            1.0612277619533155e+00,
            1.0594262608698046e+00,
            1.0599810758292072e+00,
            1.0602547314449409e+00,
            1.0601263046243634e+00,
            1.0606565756823634e+00,
        ];
        pub const CYAN: [f64; 32] = [
            1.0414628021426751e+00,
            1.0328661533771188e+00,
            1.0126146228964314e+00,
            1.0350460524836209e+00,
            1.0078661447098567e+00,
            1.0422280385081280e+00,
            1.0442596738499825e+00,
            1.0535238290294409e+00,
            1.0180776226938120e+00,
            1.0442729908727713e+00,
            1.0529362541920750e+00,
            1.0537034271160244e+00,
            1.0533901869215969e+00,
            1.0537782700979574e+00,
            1.0527093770467102e+00,
            1.0530449040446797e+00,
            1.0550554640191208e+00,
            1.0553673610724821e+00,
            1.0454306634683976e+00,
            6.2348950639230805e-01,
            1.8038071613188977e-01,
            -7.6303759201984539e-03,
            -1.5217847035781367e-04,
            -7.5102257347258311e-03,
            -2.1708639328491472e-03,
            6.5919466602369636e-04,
            1.2278815318539780e-02,
            -4.4669775637208031e-03,
            1.7119799082865147e-02,
            4.9211089759759801e-03,
            5.8762925143334985e-03,
            2.5259399415550079e-02,
        ];
        pub const MAGENTA: [f64; 32] = [
            9.9422138151236850e-01,
            9.8986937122975682e-01,
            9.8293658286116958e-01,
            9.9627868399859310e-01,
            1.0198955019000133e+00,
            1.0166395501210359e+00,
            1.0220913178757398e+00,
            9.9651666040682441e-01,
            1.0097766178917882e+00,
            1.0215422470827016e+00,
            6.4031953387790963e-01,
            2.5012379477078184e-03,
            6.5339939555769944e-03,
            2.8334080462675826e-03,
            -5.1209675389074505e-11,
            -9.0592291646646381e-03,
            3.3936718323331200e-03,
            -3.0638741121828406e-03,
            2.2203936168286292e-01,
            6.3141140024811970e-01,
            9.7480985576500956e-01,
            9.7209562333590571e-01,
            1.0173770302868150e+00,
            9.9875194322734129e-01,
            9.4701725739602238e-01,
            8.5258623154354796e-01,
            9.4897798581660842e-01,
            9.4751876096521492e-01,
            9.9598944191059791e-01,
            8.6301351503809076e-01,
            8.9150987853523145e-01,
            8.4866492652845082e-01,
        ];
        pub const YELLOW: [f64; 32] = [
            5.5740622924920873e-03,
            -4.7982831631446787e-03,
            -5.2536564298613798e-03,
            -6.4571480044499710e-03,
            -5.9693514658007013e-03,
            -2.1836716037686721e-03,
            1.6781120601055327e-02,
            9.6096355429062641e-02,
            2.1217357081986446e-01,
            3.6169133290685068e-01,
            5.3961011543232529e-01,
            7.4408810492171507e-01,
            9.2209571148394054e-01,
            1.0460304298411225e+00,
            1.0513824989063714e+00,
            1.0511991822135085e+00,
            1.0510530911991052e+00,
            1.0517397230360510e+00,
            1.0516043086790485e+00,
            1.0511944032061460e+00,
            1.0511590325868068e+00,
            1.0516612465483031e+00,
            1.0514038526836869e+00,
            1.0515941029228475e+00,
            1.0511460436960840e+00,
            1.0515123758830476e+00,
            1.0508871369510702e+00,
            1.0508923708102380e+00,
            1.0477492815668303e+00,
            1.0493272144017338e+00,
            1.0435963333422726e+00,
            1.0392280772051465e+00,
        ];
        pub const RED: [f64; 32] = [
            1.6575604867086180e-01,
            1.1846442802747797e-01,
            1.2408293329637447e-01,
            1.1371272058349924e-01,
            7.8992434518899132e-02,
            3.2205603593106549e-02,
            -1.0798365407877875e-02,
            1.8051975516730392e-02,
            5.3407196598730527e-03,
            1.3654918729501336e-02,
            -5.9564213545642841e-03,
            -1.8444365067353252e-03,
            -1.0571884361529504e-02,
            -2.9375521078000011e-03,
            -1.0790476271835936e-02,
            -8.0224306697503633e-03,
            -2.2669167702495940e-03,
            7.0200240494706634e-03,
            -8.1528469000299308e-03,
            6.0772866969252792e-01,
            9.8831560865432400e-01,
            9.9391691044078823e-01,
            1.0039338994753197e+00,
            9.9234499861167125e-01,
            9.9926530858855522e-01,
            1.0084621557617270e+00,
            9.8358296827441216e-01,
            1.0085023660099048e+00,
            9.7451138326568698e-01,
            9.8543269570059944e-01,
            9.3495763980962043e-01,
            9.8713907792319400e-01,
        ];
        pub const GREEN: [f64; 32] = [
            2.6494153587602255e-03,
            -5.0175013429732242e-03,
            -1.2547236272489583e-02,
            -9.4554964308388671e-03,
            -1.2526086181600525e-02,
            -7.9170697760437767e-03,
            -7.9955735204175690e-03,
            -9.3559433444469070e-03,
            6.5468611982999303e-02,
            3.9572875517634137e-01,
            7.5244022299886659e-01,
            9.6376478690218559e-01,
            9.9854433855162328e-01,
            9.9992977025287921e-01,
            9.9939086751140449e-01,
            9.9994372267071396e-01,
            9.9939121813418674e-01,
            9.9911237310424483e-01,
            9.6019584878271580e-01,
            6.3186279338432438e-01,
            2.5797401028763473e-01,
            9.4014888527335638e-03,
            -3.0798345608649747e-03,
            -4.5230367033685034e-03,
            -6.8933410388274038e-03,
            -9.0352195539015398e-03,
            -8.5913667165340209e-03,
            -8.3690869120289398e-03,
            -7.8685832338754313e-03,
            -8.3657578711085132e-06,
            5.4301225442817177e-03,
            -2.7745589759259194e-03,
        ];
        pub const BLUE: [f64; 32] = [
            9.9209771469720676e-01,
            9.8876426059369127e-01,
            9.9539040744505636e-01,
            9.9529317353008218e-01,
            9.9181447411633950e-01,
            1.0002584039673432e+00,
            9.9968478437342512e-01,
            9.9988120766657174e-01,
            9.8504012146370434e-01,
            7.9029849053031276e-01,
            5.6082198617463974e-01,
            3.3133458513996528e-01,
            1.3692410840839175e-01,
            1.8914906559664151e-02,
            -5.1129770932550889e-06,
            -4.2395493167891873e-04,
            -4.1934593101534273e-04,
            1.7473028136486615e-03,
            3.7999160177631316e-03,
            -5.5101474906588642e-04,
            -4.3716662898480967e-05,
            7.5874501748732798e-03,
            2.5795650780554021e-02,
            3.8168376532500548e-02,
            4.9489586408030833e-02,
            4.9595992290102905e-02,
            4.9814819505812249e-02,
            3.9840911064978023e-02,
            3.0501024937233868e-02,
            2.1243054765241080e-02,
            6.9596532104356399e-03,
            4.1733649330980525e-03,
        ];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intensity_at_wavelength_returns_expected_value_at_minimum_wavelength() {
        let target = Spectrum {
            shortest_wavelength: 400.5,
            longest_wavelength: 700.25,
            samples: vec![0.5, 1.0, 0.75, 1.5],
        };
        assert!(target.intensity_at_wavelength(400.5) == 0.5)
    }

    #[test]
    fn intensity_at_wavelength_returns_expected_value_at_max_wavelength() {
        let target = Spectrum {
            shortest_wavelength: 400.5,
            longest_wavelength: 700.25,
            samples: vec![0.5, 1.0, 0.75, 1.5],
        };
        assert!(target.intensity_at_wavelength(700.25) == 1.5)
    }

    #[test]
    fn intensity_at_wavelength_returns_expected_value_at_interior_sample_wavelength() {
        let target = Spectrum {
            shortest_wavelength: 400.0,
            longest_wavelength: 700.0,
            samples: vec![0.5, 1.0, 0.75, 1.5],
        };
        assert!(target.intensity_at_wavelength(500.0) == 1.0);
        assert!(target.intensity_at_wavelength(600.0) == 0.75);
    }

    #[test]
    fn intensity_at_wavelength_returns_expected_value_at_halfway_between_sample_wavelength() {
        let target = Spectrum {
            shortest_wavelength: 400.0,
            longest_wavelength: 700.0,
            samples: vec![0.5, 1.0, 0.75, 1.5],
        };
        assert!(target.intensity_at_wavelength(450.0) == 0.75);
        assert!(target.intensity_at_wavelength(550.0) == 0.875);
        assert!(target.intensity_at_wavelength(650.0) == 1.125);
    }

    #[test]
    fn intensity_below_minimum_wavelength_is_zero() {
        let target = Spectrum {
            shortest_wavelength: 400.0,
            longest_wavelength: 700.0,
            samples: vec![0.5, 1.0, 0.75, 1.5],
        };
        assert!(target.intensity_at_wavelength(399.9999) == 0.0);
    }

    #[test]
    fn intensity_above_maximum_wavelength_is_zero() {
        let target = Spectrum {
            shortest_wavelength: 400.0,
            longest_wavelength: 700.0,
            samples: vec![0.5, 1.0, 0.75, 1.5],
        };
        assert!(target.intensity_at_wavelength(700.0001) == 0.0);
    }
}

use nalgebra::RealField;

pub trait Real: RealField {}

impl Real for f32 {}
impl Real for f64 {}

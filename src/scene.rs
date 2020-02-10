use nalgebra::{Point3};

use crate::raycasting::Primitive;
use crate::Real;

use std::sync::Arc;

pub struct Scene<T: Real> {
    pub camera_location: Point3<T>,
    pub objects: Vec<Arc<dyn Primitive<T>>>,
}

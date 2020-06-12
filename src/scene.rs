use nalgebra::{Point3};

use crate::raycasting::Aggregate;
use crate::Real;

pub struct Scene<T: Real> {
    pub camera_location: Point3<T>,
    pub objects: Vec<Box<dyn Aggregate<T>>>,
}

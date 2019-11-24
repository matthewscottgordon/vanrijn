use nalgebra::{RealField, Vector3};

use crate::raycasting::Intersect;

pub struct Scene<T: RealField> {
    pub camera_location: Vector3<T>,
    pub objects: Vec<Box<dyn Intersect<T>>>,
}

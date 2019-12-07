use nalgebra::{Point3, RealField};

use crate::raycasting::Intersect;

pub struct Scene<T: RealField> {
    pub camera_location: Point3<T>,
    pub objects: Vec<Box<dyn Intersect<T>>>,
}

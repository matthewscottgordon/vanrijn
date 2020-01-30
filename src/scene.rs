use nalgebra::{Point3, RealField};

use crate::raycasting::Primitive;

use std::sync::Arc;

pub struct Scene<T: RealField> {
    pub camera_location: Point3<T>,
    pub objects: Vec<Arc<dyn Primitive<T>>>,
}

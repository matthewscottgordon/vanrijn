use nalgebra::Point3;

use crate::raycasting::Aggregate;

pub struct Scene {
    pub camera_location: Point3<f64>,
    pub objects: Vec<Box<dyn Aggregate>>,
}

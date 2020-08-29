use crate::math::Vec3;

use crate::raycasting::Aggregate;

pub struct Scene {
    pub camera_location: Vec3,
    pub objects: Vec<Box<dyn Aggregate>>,
}

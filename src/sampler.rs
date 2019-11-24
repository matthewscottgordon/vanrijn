use super::raycasting::{IntersectionInfo, Ray};
use super::scene::Scene;

use nalgebra::RealField;

pub struct Sampler<'a, T: RealField> {
    pub scene: &'a Scene<T>,
}

impl<'a, T: RealField> Sampler<'a, T> {
    pub fn sample(&self, ray: &Ray<T>) -> Option<IntersectionInfo<T>> {
        self.scene
            .objects
            .iter()
            .flat_map(|object| object.intersect(&ray))
            .min_by(
                |a, b| match PartialOrd::partial_cmp(&a.distance, &b.distance) {
                    None => std::cmp::Ordering::Less,
                    Some(ordering) => ordering,
                },
            )
    }
}

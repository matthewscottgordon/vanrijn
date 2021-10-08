use super::raycasting::{IntersectionInfo, Ray};
use super::scene::Scene;

pub struct Sampler<'a> {
    pub scene: &'a Scene,
}

impl<'a> Sampler<'a> {
    pub fn sample(&self, ray: &Ray) -> Option<IntersectionInfo> {
        self.scene
            .objects
            .iter()
            .flat_map(|object| object.intersect(ray))
            .min_by(
                |a, b| match PartialOrd::partial_cmp(&a.distance, &b.distance) {
                    None => std::cmp::Ordering::Less,
                    Some(ordering) => ordering,
                },
            )
    }
}

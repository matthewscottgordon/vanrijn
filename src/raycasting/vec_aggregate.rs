use super::{Aggregate, BoundingBox, HasBoundingBox, Intersect, IntersectionInfo, Primitive, Ray};
use crate::Real;

impl<T: Real> HasBoundingBox<T> for Vec<Box<dyn Primitive<T>>> {
    fn bounding_box(&self) -> BoundingBox<T> {
        self.iter().fold(BoundingBox::empty(), |acc, elem| {
            acc.union(&elem.bounding_box())
        })
    }
}

impl<T: Real> Intersect<T> for Vec<Box<dyn Primitive<T>>> {
    fn intersect<'a>(&'a self, ray: &Ray<T>) -> Option<IntersectionInfo<T>> {
        self.iter()
            .flat_map(|primitive| primitive.intersect(&ray))
            .min_by(
                |a, b| match PartialOrd::partial_cmp(&a.distance, &b.distance) {
                    None => std::cmp::Ordering::Less,
                    Some(ordering) => ordering,
                },
            )
    }
}

impl<T: Real> Aggregate<T> for Vec<Box<dyn Primitive<T>>> {}


impl<T: Real> HasBoundingBox<T> for Vec<Box<dyn Aggregate<T>>> {
    fn bounding_box(&self) -> BoundingBox<T> {
        self.iter().fold(BoundingBox::empty(), |acc, elem| {
            acc.union(&elem.bounding_box())
        })
    }
}

impl<T: Real> Intersect<T> for Vec<Box<dyn Aggregate<T>>> {
    fn intersect<'a>(&'a self, ray: &Ray<T>) -> Option<IntersectionInfo<T>> {
        self.iter()
            .flat_map(|aggregate| aggregate.intersect(&ray))
            .min_by(
                |a, b| match PartialOrd::partial_cmp(&a.distance, &b.distance) {
                    None => std::cmp::Ordering::Less,
                    Some(ordering) => ordering,
                },
            )
    }
}

impl<T: Real> Aggregate<T> for Vec<Box<dyn Aggregate<T>>> {}

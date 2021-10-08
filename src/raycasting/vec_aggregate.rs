use super::{Aggregate, BoundingBox, HasBoundingBox, Intersect, IntersectionInfo, Primitive, Ray};

impl HasBoundingBox for Vec<Box<dyn Primitive>> {
    fn bounding_box(&self) -> BoundingBox {
        self.iter().fold(BoundingBox::empty(), |acc, elem| {
            acc.union(&elem.bounding_box())
        })
    }
}

impl Intersect for Vec<Box<dyn Primitive>> {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo> {
        self.iter()
            .flat_map(|primitive| primitive.intersect(ray))
            .min_by(
                |a, b| match PartialOrd::partial_cmp(&a.distance, &b.distance) {
                    None => std::cmp::Ordering::Less,
                    Some(ordering) => ordering,
                },
            )
    }
}

impl Aggregate for Vec<Box<dyn Primitive>> {}

impl HasBoundingBox for Vec<Box<dyn Aggregate>> {
    fn bounding_box(&self) -> BoundingBox {
        self.iter().fold(BoundingBox::empty(), |acc, elem| {
            acc.union(&elem.bounding_box())
        })
    }
}

impl Intersect for Vec<Box<dyn Aggregate>> {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo> {
        self.iter()
            .flat_map(|aggregate| aggregate.intersect(ray))
            .min_by(
                |a, b| match PartialOrd::partial_cmp(&a.distance, &b.distance) {
                    None => std::cmp::Ordering::Less,
                    Some(ordering) => ordering,
                },
            )
    }
}

impl Aggregate for Vec<Box<dyn Aggregate>> {}

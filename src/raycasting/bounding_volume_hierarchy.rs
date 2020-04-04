use super::{BoundingBox, HasBoundingBox, Intersect, IntersectP, IntersectionInfo, Primitive, Ray};

use crate::util::morton::morton_order_value_3d;
use crate::util::normalizer::Point3Normalizer;
use crate::Real;

use nalgebra::{convert, Point3};

use std::sync::Arc;

#[derive(Clone)]
pub enum BoundingVolumeHierarchy<T: Real> {
    Node {
        bounds: BoundingBox<T>,
        left: Box<BoundingVolumeHierarchy<T>>,
        right: Box<BoundingVolumeHierarchy<T>>,
    },
    Leaf {
        bounds: BoundingBox<T>,
        primitive: Arc<dyn Primitive<T>>,
    },
    None,
}

fn centre<T: Real>(bounds: &BoundingBox<T>) -> Point3<T> {
    let two = convert(2.0);
    Point3::new(
        (bounds.bounds[0].get_min() + bounds.bounds[0].get_max()) / two,
        (bounds.bounds[1].get_min() + bounds.bounds[1].get_max()) / two,
        (bounds.bounds[2].get_min() + bounds.bounds[2].get_max()) / two,
    )
}

impl<T: Real> BoundingVolumeHierarchy<T> {
    pub fn build<'a, I>(primitives: I) -> Self
    where
        I: IntoIterator<Item = &'a Arc<dyn Primitive<T>>>,
    {
        Self::from_node_vec(
            primitives
                .into_iter()
                .map(|primitive| (primitive.bounding_box(), Arc::clone(primitive)))
                .collect(),
        )
    }

    fn from_node_vec(nodes: Vec<(BoundingBox<T>, Arc<dyn Primitive<T>>)>) -> Self {
        let overall_bounds = nodes
            .iter()
            .fold(BoundingBox::empty(), |a, (b, _)| a.union(b));
        let normalizer = Point3Normalizer::new(overall_bounds);
        let mut nodes = nodes;
        nodes.sort_by(|(a, _), (b, _)| {
            morton_order_value_3d(normalizer.normalize(centre(a)))
                .cmp(&morton_order_value_3d(normalizer.normalize(centre(b))))
        });
        Self::from_sorted_nodes(nodes.as_slice())
    }

    fn from_sorted_nodes(nodes: &[(BoundingBox<T>, Arc<dyn Primitive<T>>)]) -> Self {
        if nodes.len() >= 2 {
            let midpoint = nodes.len() / 2;
            let left = Box::new(Self::from_sorted_nodes(&nodes[..midpoint]));
            let right = Box::new(Self::from_sorted_nodes(&nodes[midpoint..]));
            let bounds = left.get_bounds().union(&right.get_bounds());
            BoundingVolumeHierarchy::Node {
                bounds,
                left,
                right,
            }
        } else if nodes.len() == 1 {
            let (bounds, ref primitive) = nodes[0];
            BoundingVolumeHierarchy::Leaf {
                bounds,
                primitive: Arc::clone(primitive),
            }
        } else {
            BoundingVolumeHierarchy::None
        }
    }

    pub fn get_bounds(&self) -> BoundingBox<T> {
        match self {
            BoundingVolumeHierarchy::Node {
                bounds,
                left: _,
                right: _,
            } => *bounds,
            BoundingVolumeHierarchy::Leaf {
                bounds,
                primitive: _,
            } => *bounds,
            BoundingVolumeHierarchy::None => BoundingBox::empty(),
        }
    }

    pub fn count_leaves(&self) -> usize {
        match self {
            Self::Node {
                bounds: _,
                left,
                right,
            } => right.count_leaves() + left.count_leaves(),
            Self::Leaf {
                bounds: _,
                primitive: _,
            } => 1,
            Self::None => 0,
        }
    }
}

fn closest_intersection<T: Real>(
    a: Option<IntersectionInfo<T>>,
    b: Option<IntersectionInfo<T>>,
) -> Option<IntersectionInfo<T>> {
    match (a, b) {
        (Some(a), Some(b)) => {
            if a.distance < b.distance {
                Some(a)
            } else {
                Some(b)
            }
        }
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

impl<T: Real> Intersect<T> for BoundingVolumeHierarchy<T> {
    fn intersect<'a>(&'a self, ray: &Ray<T>) -> Option<IntersectionInfo<T>> {
        match self {
            Self::Node {
                bounds,
                left,
                right,
            } => {
                if bounds.intersect(ray) {
                    closest_intersection(left.intersect(ray), right.intersect(ray))
                } else {
                    None
                }
            }
            Self::Leaf {
                bounds: _,
                primitive,
            } => primitive.intersect(ray),
            Self::None => None,
        }
    }
}

impl<T: Real> HasBoundingBox<T> for BoundingVolumeHierarchy<T> {
    fn bounding_box(&self) -> BoundingBox<T> {
        self.get_bounds()
    }
}

impl<T: Real> Primitive<T> for BoundingVolumeHierarchy<T> {}

pub struct FilterIterator<'a, T: Real> {
    unsearched_subtrees: Vec<&'a BoundingVolumeHierarchy<T>>,
    predicate: Box<dyn Fn(&BoundingBox<T>) -> bool>,
}

impl<'a, T: Real> FilterIterator<'a, T> {
    pub fn new(
        root: &'a BoundingVolumeHierarchy<T>,
        predicate: Box<dyn Fn(&BoundingBox<T>) -> bool>,
    ) -> Self {
        FilterIterator {
            unsearched_subtrees: vec![root],
            predicate,
        }
    }
}

impl<T: Real> Iterator for FilterIterator<'_, T> {
    type Item = Arc<dyn Primitive<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        //let mut result = Option::None;
        while let Some(next_subtree) = self.unsearched_subtrees.pop() {
            match next_subtree {
                BoundingVolumeHierarchy::Node {
                    bounds,
                    left,
                    right,
                } => {
                    if (self.predicate)(bounds) {
                        self.unsearched_subtrees.push(right);
                        self.unsearched_subtrees.push(left);
                    }
                }
                BoundingVolumeHierarchy::Leaf { bounds, primitive } => {
                    if (self.predicate)(bounds) {
                        return Some(Arc::clone(primitive));
                    }
                }
                BoundingVolumeHierarchy::None => {}
            }
        }
        Option::None
    }
}

#[cfg(test)]
mod test {

    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    use super::*;
    use crate::materials::LambertianMaterial;
    use crate::raycasting::Sphere;
    use nalgebra::Point3;

    impl<T: Arbitrary + Real> Arbitrary for Sphere<T> {
        fn arbitrary<G: Gen>(g: &mut G) -> Sphere<T> {
            let centre = <Point3<T> as Arbitrary>::arbitrary(g);
            let radius = <T as Arbitrary>::arbitrary(g);
            Sphere::new(centre, radius, Arc::new(LambertianMaterial::new_dummy()))
        }
    }

    fn sphere_vec_to_primitive_arc_vec<T: Real>(
        spheres: &Vec<Sphere<T>>,
    ) -> Vec<Arc<dyn Primitive<T>>> {
        let mut prims: Vec<Arc<dyn Primitive<T>>> = Vec::with_capacity(spheres.len());
        for sphere in spheres {
            prims.push(Arc::new(sphere.clone()));
        }
        prims
    }

    #[quickcheck]
    fn contains_expected_number_of_primitives(spheres: Vec<Sphere<f32>>) -> bool {
        let target =
            BoundingVolumeHierarchy::build(sphere_vec_to_primitive_arc_vec(&spheres).iter());

        target.count_leaves() == spheres.len()
    }

    #[quickcheck]
    fn finds_expected_points(spheres: Vec<Sphere<f32>>, p: Point3<f32>) -> bool {
        let primitives = sphere_vec_to_primitive_arc_vec(&spheres);
        let target = BoundingVolumeHierarchy::build(primitives.iter());
        let expected_hits: Vec<Arc<dyn Primitive<f32>>> = primitives
            .iter()
            .filter(|elem| elem.bounding_box().contains_point(p))
            .cloned()
            .collect();
        let found_hits: Vec<Arc<dyn Primitive<f32>>> = FilterIterator::new(
            &target,
            Box::new(move |elem: &BoundingBox<f32>| elem.contains_point(p)),
        )
        .collect();
        expected_hits.iter().all(|expected_hit| {
            found_hits
                .iter()
                .any(|found_hit| Arc::ptr_eq(found_hit, expected_hit))
        })
    }
}

use super::{
    Aggregate, BoundingBox, HasBoundingBox, Intersect, IntersectP, IntersectionInfo, Primitive, Ray,
};

use crate::Real;

use nalgebra::{convert, Point3};

use std::cmp::Ordering;
use std::sync::Arc;

/// Stores a set of [Primitives](Primitive) and accelerates raycasting
///
/// Organizes the primitives into a binary tree based on their bounds, allowing the
/// closest intersection with a ray to be found efficiently.
///
/// Each node knows the overall bounds of all it's children, which means that a ray that
/// doesn't intersect the [BoundingBox](BoundingBox) of the node doesn't intersect any of
/// the primitives stored in it's children.
pub enum BoundingVolumeHierarchy<T: Real> {
    Node {
        bounds: BoundingBox<T>,
        left: Box<BoundingVolumeHierarchy<T>>,
        right: Box<BoundingVolumeHierarchy<T>>,
    },
    Leaf {
        bounds: BoundingBox<T>,
        primitives: Vec<Arc<dyn Primitive<T>>>,
    },
}

fn centre<T: Real>(bounds: &BoundingBox<T>) -> Point3<T> {
    let two = convert(2.0);
    Point3::new(
        (bounds.bounds[0].get_min() + bounds.bounds[0].get_max()) / two,
        (bounds.bounds[1].get_min() + bounds.bounds[1].get_max()) / two,
        (bounds.bounds[2].get_min() + bounds.bounds[2].get_max()) / two,
    )
}

fn heuristic_split<T: Real>(
    primitives: &mut [Arc<dyn Primitive<T>>],
    bounds: &BoundingBox<T>,
) -> usize {
    let largest_dimension = bounds.largest_dimension();
    primitives.sort_unstable_by(|a, b| {
        centre(&a.bounding_box())[largest_dimension]
            .partial_cmp(&centre(&b.bounding_box())[largest_dimension])
            .unwrap_or(Ordering::Equal)
    });
    primitives.len() / 2
}

impl<T: Real> BoundingVolumeHierarchy<T> {
    pub fn build(primitives: &mut [Arc<dyn Primitive<T>>]) -> Self {
        BoundingVolumeHierarchy::build_from_slice(primitives)
    }

    pub fn build_from_slice(primitives: &mut [Arc<dyn Primitive<T>>]) -> Self {
        let bounds = primitives
            .iter()
            .fold(BoundingBox::empty(), |acc, p| acc.union(&p.bounding_box()));
        if primitives.len() <= 1 {
            let primitives = primitives.iter().cloned().collect();
            BoundingVolumeHierarchy::Leaf { bounds, primitives }
        } else {
            let pivot = heuristic_split(primitives, &bounds);
            let left = Box::new(BoundingVolumeHierarchy::build_from_slice(
                &mut primitives[0..pivot],
            ));
            let right = Box::new(BoundingVolumeHierarchy::build_from_slice(
                &mut primitives[pivot..],
            ));
            BoundingVolumeHierarchy::Node {
                bounds,
                left,
                right,
            }
        }
    }
}

fn closest_intersection<T: Real>(
    a: Option<IntersectionInfo<T>>,
    b: Option<IntersectionInfo<T>>,
) -> Option<IntersectionInfo<T>> {
    match a {
        None => b,
        Some(a_info) => match b {
            None => Some(a_info),
            Some(b_info) => Some(if a_info.distance < b_info.distance {
                a_info
            } else {
                b_info
            }),
        },
    }
}

impl<T: Real> Intersect<T> for BoundingVolumeHierarchy<T> {
    fn intersect<'a>(&'a self, ray: &Ray<T>) -> Option<IntersectionInfo<T>> {
        match self {
            BoundingVolumeHierarchy::Node {
                bounds,
                left,
                right,
            } => {
                if bounds.intersect(&ray) {
                    closest_intersection(left.intersect(&ray), right.intersect(&ray))
                } else {
                    None
                }
            }
            BoundingVolumeHierarchy::Leaf { bounds, primitives } => {
                if bounds.intersect(&ray) {
                    primitives
                        .iter()
                        .map(|elem| elem.intersect(&ray))
                        .fold(None, |acc, elem| closest_intersection(acc, elem))
                } else {
                    None
                }
            }
        }
    }
}

impl<T: Real> HasBoundingBox<T> for BoundingVolumeHierarchy<T> {
    fn bounding_box(&self) -> BoundingBox<T> {
        BoundingBox::empty()
    }
}

impl<T: Real> Aggregate<T> for BoundingVolumeHierarchy<T> {}

#[cfg(test)]
mod test {}

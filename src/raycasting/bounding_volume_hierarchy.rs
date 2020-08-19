use super::{
    Aggregate, BoundingBox, HasBoundingBox, Intersect, IntersectP, IntersectionInfo, Primitive, Ray,
};

use nalgebra::Point3;

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
pub enum BoundingVolumeHierarchy {
    Node {
        bounds: BoundingBox,
        left: Box<BoundingVolumeHierarchy>,
        right: Box<BoundingVolumeHierarchy>,
    },
    Leaf {
        bounds: BoundingBox,
        primitives: Vec<Arc<dyn Primitive>>,
    },
}

fn centre(bounds: &BoundingBox) -> Point3<f64> {
    Point3::new(
        (bounds.bounds[0].get_min() + bounds.bounds[0].get_max()) / 2.00,
        (bounds.bounds[1].get_min() + bounds.bounds[1].get_max()) / 2.0,
        (bounds.bounds[2].get_min() + bounds.bounds[2].get_max()) / 2.0,
    )
}

fn heuristic_split(primitives: &mut [Arc<dyn Primitive>], bounds: &BoundingBox) -> usize {
    let largest_dimension = bounds.largest_dimension();
    primitives.sort_unstable_by(|a, b| {
        centre(&a.bounding_box())[largest_dimension]
            .partial_cmp(&centre(&b.bounding_box())[largest_dimension])
            .unwrap_or(Ordering::Equal)
    });
    primitives.len() / 2
}

impl BoundingVolumeHierarchy {
    pub fn build(primitives: &mut [Arc<dyn Primitive>]) -> Self {
        BoundingVolumeHierarchy::build_from_slice(primitives)
    }

    pub fn build_from_slice(primitives: &mut [Arc<dyn Primitive>]) -> Self {
        let bounds = primitives
            .iter()
            .fold(BoundingBox::empty(), |acc, p| acc.union(&p.bounding_box()));
        if primitives.len() <= 1 {
            let primitives = primitives.to_vec();
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

fn closest_intersection(
    a: Option<IntersectionInfo>,
    b: Option<IntersectionInfo>,
) -> Option<IntersectionInfo> {
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

impl Intersect for BoundingVolumeHierarchy {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<IntersectionInfo> {
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
                        .fold(None, closest_intersection)
                } else {
                    None
                }
            }
        }
    }
}

impl HasBoundingBox for BoundingVolumeHierarchy {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::empty()
    }
}

impl Aggregate for BoundingVolumeHierarchy {}

#[cfg(test)]
mod test {}

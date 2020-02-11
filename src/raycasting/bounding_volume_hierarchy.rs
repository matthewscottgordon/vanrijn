use super::{BoundingBox, Primitive};

use crate::util::morton::morton_order_value;
use crate::util::normalizer::Point3Normalizer;
use crate::Real;

use nalgebra::{convert, Point3};

use std::sync::Arc;

#[derive(Clone)]
pub enum BoundingVolumeHierarchy<T: Real> {
    Node {
        bounds: BoundingBox<T>,
        left: Arc<BoundingVolumeHierarchy<T>>,
        right: Arc<BoundingVolumeHierarchy<T>>,
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
            morton_order_value(normalizer.normalize(centre(a)))
                .cmp(&morton_order_value(normalizer.normalize(centre(b))))
        });
        Self::from_sorted_nodes(nodes.as_slice())
    }

    fn from_sorted_nodes(nodes: &[(BoundingBox<T>, Arc<dyn Primitive<T>>)]) -> Self {
        if nodes.len() >= 2 {
            let midpoint = nodes.len() / 2;
            let left = Arc::new(Self::from_sorted_nodes(&nodes[..midpoint]));
            let right = Arc::new(Self::from_sorted_nodes(&nodes[midpoint..]));
            let bounds = left.get_bounds().union(&right.get_bounds());
            BoundingVolumeHierarchy::Node {
                bounds,
                left,
                right,
            }
        } else if nodes.len() == 1 {
            match nodes[0] {
                (bounds, ref primitive) => BoundingVolumeHierarchy::Leaf {
                    bounds,
                    primitive: Arc::clone(primitive),
                },
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
}

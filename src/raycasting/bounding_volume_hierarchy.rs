use super::{
    Aggregate, BoundingBox, HasBoundingBox, Intersect, IntersectP, IntersectionInfo, Primitive, Ray,
};

use crate::util::binary_tree::BinaryTree;
use crate::util::morton::morton_order_value_3d;
use crate::util::normalizer::Point3Normalizer;
use crate::Real;

use nalgebra::{convert, Point3};

use std::mem::swap;

type Tree<T> = BinaryTree<BoundingBox<T>, Box<dyn Primitive<T>>>;

/// Stores a set of [Primitives](Primitive) and accelerates raycasting
///
/// Organizes the primitives into a binary tree based on their bounds, allowing the
/// closest intersection with a ray to be found efficiently.
///
/// Each node knows the overall bounds of all it's children, which means that a ray that
/// doesn't intersect the [BoundingBox](BoundingBox) of the node doesn't intersect any of
/// the primitives stored in it's children.
pub struct BoundingVolumeHierarchy<T: Real> {
    tree: Tree<T>,
}

fn centre<T: Real>(bounds: &BoundingBox<T>) -> Point3<T> {
    let two = convert(2.0);
    Point3::new(
        (bounds.bounds[0].get_min() + bounds.bounds[0].get_max()) / two,
        (bounds.bounds[1].get_min() + bounds.bounds[1].get_max()) / two,
        (bounds.bounds[2].get_min() + bounds.bounds[2].get_max()) / two,
    )
}

struct PrimitiveInfo<T: Real>(BoundingBox<T>, Option<Box<dyn Primitive<T>>>);

impl<T: Real> BoundingVolumeHierarchy<T> {
    pub fn build<'a, I>(primitives: I) -> Self
    where
        I: IntoIterator<Item = Box<dyn Primitive<T>>>,
    {
        let tree = Self::from_node_vec(
            primitives
                .into_iter()
                .map(|primitive| PrimitiveInfo(primitive.bounding_box(), Some(primitive)))
                .collect(),
        );
        Self { tree }
    }

    fn from_node_vec(nodes: Vec<PrimitiveInfo<T>>) -> Tree<T> {
        let overall_bounds = nodes
            .iter()
            .fold(BoundingBox::empty(), |a, PrimitiveInfo(b, _)| a.union(b));
        let normalizer = Point3Normalizer::new(overall_bounds);
        let mut nodes = nodes;
        nodes.sort_by(|PrimitiveInfo(a, _), PrimitiveInfo(b, _)| {
            morton_order_value_3d(normalizer.normalize(centre(a)))
                .cmp(&morton_order_value_3d(normalizer.normalize(centre(b))))
        });
        Self::from_sorted_nodes(nodes.as_mut_slice())
    }

    fn from_sorted_nodes(nodes: &mut [PrimitiveInfo<T>]) -> Tree<T> {
        if nodes.len() >= 2 {
            let midpoint = nodes.len() / 2;
            let left = Box::new(Self::from_sorted_nodes(&mut nodes[..midpoint]));
            let right = Box::new(Self::from_sorted_nodes(&mut nodes[midpoint..]));
            let bounds = Self::get_bounds(&left).union(&Self::get_bounds(&right));
            Tree::Branch {
                value: bounds,
                left,
                right,
            }
        } else if nodes.len() == 1 {
            let PrimitiveInfo(_, ref mut primitive_src) = nodes[0];
            let mut primitive = None;
            swap(primitive_src, &mut primitive);
            let primitive = primitive.unwrap();
            Tree::Leaf { value: primitive }
        } else {
            Tree::None
        }
    }

    pub fn get_bounds(tree: &Tree<T>) -> BoundingBox<T> {
        match tree {
            Tree::Branch {
                value,
                left: _,
                right: _,
            } => *value,
            Tree::Leaf { value } => value.bounding_box(),
            Tree::None => BoundingBox::empty(),
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

impl<T: Real> Intersect<T> for Tree<T> {
    fn intersect<'a>(&'a self, ray: &Ray<T>) -> Option<IntersectionInfo<T>> {
        match self {
            Tree::Branch {
                value: bounds,
                left,
                right,
            } => {
                if bounds.intersect(ray) {
                    closest_intersection(left.intersect(ray), right.intersect(ray))
                } else {
                    None
                }
            }
            Tree::Leaf { value: primitive } => primitive.intersect(ray),
            Tree::None => None,
        }
    }
}

impl<T: Real> Intersect<T> for BoundingVolumeHierarchy<T> {
    fn intersect<'a>(&'a self, ray: &Ray<T>) -> Option<IntersectionInfo<T>> {
        self.tree.intersect(ray)
    }
}

impl<T: Real> HasBoundingBox<T> for BoundingVolumeHierarchy<T> {
    fn bounding_box(&self) -> BoundingBox<T> {
        Self::get_bounds(&self.tree)
    }
}

impl<T: Real> Aggregate<T> for BoundingVolumeHierarchy<T> {}

#[cfg(test)]
mod test {

    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    use super::*;
    use crate::materials::LambertianMaterial;
    use crate::raycasting::Sphere;
    use nalgebra::Point3;

    use std::sync::Arc;

    impl<T: Arbitrary + Real> Arbitrary for Sphere<T> {
        fn arbitrary<G: Gen>(g: &mut G) -> Sphere<T> {
            let centre = <Point3<T> as Arbitrary>::arbitrary(g);
            let radius = <T as Arbitrary>::arbitrary(g);
            Sphere::new(centre, radius, Arc::new(LambertianMaterial::new_dummy()))
        }
    }

    fn sphere_vec_to_primitive_box_vec<T: Real>(
        spheres: &Vec<Sphere<T>>,
    ) -> Vec<Box<dyn Primitive<T>>> {
        let mut prims: Vec<Box<dyn Primitive<T>>> = Vec::with_capacity(spheres.len());
        for sphere in spheres {
            prims.push(Box::new(sphere.clone()));
        }
        prims
    }

    #[quickcheck]
    fn contains_expected_number_of_primitives(spheres: Vec<Sphere<f32>>) -> bool {
        let target = BoundingVolumeHierarchy::build(sphere_vec_to_primitive_box_vec(&spheres));

        target.tree.count_leaves() == spheres.len()
    }
}

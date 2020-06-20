use itertools::izip;
use nalgebra::{convert, Point3, Vector3};

use crate::materials::Material;
use crate::raycasting::Triangle;
use crate::Real;

use std::sync::Arc;

pub fn triangulate_polygon<T: Real>(
    vertices: &Vec<Point3<T>>,
    normal: &Vector3<T>,
    material: Arc<dyn Material<T>>,
) -> Vec<Triangle<T>> {
    assert!(vertices.len() >= 3);
    let hinge = vertices[0];
    izip!(vertices.iter().skip(1), vertices.iter().skip(2))
        .map(|(a, b)| Triangle {
            vertices: [hinge, *a, *b],
            normals: [*normal, *normal, *normal],
            material: Arc::clone(&material),
        })
        .collect()
}

pub fn generate_dodecahedron<T: Real>(
    centre: Point3<T>,
    size: T,
    material: Arc<dyn Material<T>>,
) -> Vec<Triangle<T>> {
    let phi = convert((1.0 + (5.0_f64).sqrt()) / 2.0);
    let phi_inv = T::one() / phi;
    let one = T::one();
    let zero = T::zero();

    let faces = vec![
        vec![
            Vector3::new(phi_inv, zero, phi),
            Vector3::new(-phi_inv, zero, phi),
            Vector3::new(-one, -one, one),
            Vector3::new(zero, -phi, phi_inv),
            Vector3::new(one, -one, one),
        ],
        vec![
            Vector3::new(phi_inv, zero, phi),
            Vector3::new(-phi_inv, zero, phi),
            Vector3::new(-one, one, one),
            Vector3::new(zero, phi, phi_inv),
            Vector3::new(one, one, one),
        ],
        vec![
            Vector3::new(phi_inv, zero, phi),
            Vector3::new(one, -one, one),
            Vector3::new(phi, -phi_inv, zero),
            Vector3::new(phi, phi_inv, zero),
            Vector3::new(one, one, one),
        ],
        vec![
            Vector3::new(-phi_inv, zero, phi),
            Vector3::new(-one, -one, one),
            Vector3::new(-phi, -phi_inv, zero),
            Vector3::new(-phi, phi_inv, zero),
            Vector3::new(-one, one, one),
        ],
        vec![
            Vector3::new(-one, -one, one),
            Vector3::new(-phi, -phi_inv, zero),
            Vector3::new(-one, -one, -one),
            Vector3::new(zero, -phi, -phi_inv),
            Vector3::new(zero, -phi, phi_inv),
        ],
        vec![
            Vector3::new(zero, -phi, phi_inv),
            Vector3::new(zero, -phi, -phi_inv),
            Vector3::new(one, -one, -one),
            Vector3::new(phi, -phi_inv, zero),
            Vector3::new(one, -one, one),
        ],
        vec![
            Vector3::new(zero, phi, phi_inv),
            Vector3::new(zero, phi, -phi_inv),
            Vector3::new(-one, one, -one),
            Vector3::new(-phi, phi_inv, zero),
            Vector3::new(-one, one, one),
        ],
        vec![
            Vector3::new(one, one, one),
            Vector3::new(phi, phi_inv, zero),
            Vector3::new(one, one, -one),
            Vector3::new(zero, phi, -phi_inv),
            Vector3::new(zero, phi, phi_inv),
        ],
        vec![
            Vector3::new(one, -one, -one),
            Vector3::new(zero, -phi, -phi_inv),
            Vector3::new(-one, -one, -one),
            Vector3::new(-phi_inv, zero, -phi),
            Vector3::new(phi_inv, zero, -phi),
        ],
        vec![
            Vector3::new(one, one, -one),
            Vector3::new(zero, phi, -phi_inv),
            Vector3::new(-one, one, -one),
            Vector3::new(-phi_inv, zero, -phi),
            Vector3::new(phi_inv, zero, -phi),
        ],
        vec![
            Vector3::new(one, one, -one),
            Vector3::new(phi, phi_inv, zero),
            Vector3::new(phi, -phi_inv, zero),
            Vector3::new(one, -one, -one),
            Vector3::new(phi_inv, zero, -phi),
        ],
        vec![
            Vector3::new(-one, one, -one),
            Vector3::new(-phi, phi_inv, zero),
            Vector3::new(-phi, -phi_inv, zero),
            Vector3::new(-one, -one, -one),
            Vector3::new(-phi_inv, zero, -phi),
        ],
    ];

    let scale = size * convert(3f64.sqrt() / 2.0);
    faces
        .iter()
        .flat_map(|face| {
            let normal = (face[1] - face[0]).cross(&(face[2] - face[1]));
            let transformed_face = face.iter().map(|v| centre + v * scale).collect();
            triangulate_polygon(&transformed_face, &normal, Arc::clone(&material))
        })
        .collect()
}

use itertools::izip;
use nalgebra::{convert, Point3, Vector3};

use crate::materials::Material;
use crate::raycasting::Triangle;

use std::sync::Arc;

pub fn triangulate_polygon(
    vertices: &Vec<Point3<f64>>,
    normal: &Vector3<f64>,
    material: Arc<dyn Material>,
) -> Vec<Triangle> {
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

pub fn generate_dodecahedron(
    centre: Point3<f64>,
    size: f64,
    material: Arc<dyn Material>,
) -> Vec<Triangle> {
    let phi = convert((1.0 + (5.0_f64).sqrt()) / 2.0);
    let phi_inv = 1.0 / phi;

    let faces = vec![
        vec![
            Vector3::new(phi_inv, 0.0, phi),
            Vector3::new(-phi_inv, 0.0, phi),
            Vector3::new(-1.0, -1.0, 1.0),
            Vector3::new(0.0, -phi, phi_inv),
            Vector3::new(1.0, -1.0, 1.0),
        ],
        vec![
            Vector3::new(phi_inv, 0.0, phi),
            Vector3::new(-phi_inv, 0.0, phi),
            Vector3::new(-1.0, 1.0, 1.0),
            Vector3::new(0.0, phi, phi_inv),
            Vector3::new(1.0, 1.0, 1.0),
        ],
        vec![
            Vector3::new(phi_inv, 0.0, phi),
            Vector3::new(1.0, -1.0, 1.0),
            Vector3::new(phi, -phi_inv, 0.0),
            Vector3::new(phi, phi_inv, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
        ],
        vec![
            Vector3::new(-phi_inv, 0.0, phi),
            Vector3::new(-1.0, -1.0, 1.0),
            Vector3::new(-phi, -phi_inv, 0.0),
            Vector3::new(-phi, phi_inv, 0.0),
            Vector3::new(-1.0, 1.0, 1.0),
        ],
        vec![
            Vector3::new(-1.0, -1.0, 1.0),
            Vector3::new(-phi, -phi_inv, 0.0),
            Vector3::new(-1.0, -1.0, -1.0),
            Vector3::new(0.0, -phi, -phi_inv),
            Vector3::new(0.0, -phi, phi_inv),
        ],
        vec![
            Vector3::new(0.0, -phi, phi_inv),
            Vector3::new(0.0, -phi, -phi_inv),
            Vector3::new(1.0, -1.0, -1.0),
            Vector3::new(phi, -phi_inv, 0.0),
            Vector3::new(1.0, -1.0, 1.0),
        ],
        vec![
            Vector3::new(0.0, phi, phi_inv),
            Vector3::new(0.0, phi, -phi_inv),
            Vector3::new(-1.0, 1.0, -1.0),
            Vector3::new(-phi, phi_inv, 0.0),
            Vector3::new(-1.0, 1.0, 1.0),
        ],
        vec![
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(phi, phi_inv, 0.0),
            Vector3::new(1.0, 1.0, -1.0),
            Vector3::new(0.0, phi, -phi_inv),
            Vector3::new(0.0, phi, phi_inv),
        ],
        vec![
            Vector3::new(1.0, -1.0, -1.0),
            Vector3::new(0.0, -phi, -phi_inv),
            Vector3::new(-1.0, -1.0, -1.0),
            Vector3::new(-phi_inv, 0.0, -phi),
            Vector3::new(phi_inv, 0.0, -phi),
        ],
        vec![
            Vector3::new(1.0, 1.0, -1.0),
            Vector3::new(0.0, phi, -phi_inv),
            Vector3::new(-1.0, 1.0, -1.0),
            Vector3::new(-phi_inv, 0.0, -phi),
            Vector3::new(phi_inv, 0.0, -phi),
        ],
        vec![
            Vector3::new(1.0, 1.0, -1.0),
            Vector3::new(phi, phi_inv, 0.0),
            Vector3::new(phi, -phi_inv, 0.0),
            Vector3::new(1.0, -1.0, -1.0),
            Vector3::new(phi_inv, 0.0, -phi),
        ],
        vec![
            Vector3::new(-1.0, 1.0, -1.0),
            Vector3::new(-phi, phi_inv, 0.0),
            Vector3::new(-phi, -phi_inv, 0.0),
            Vector3::new(-1.0, -1.0, -1.0),
            Vector3::new(-phi_inv, 0.0, -phi),
        ],
    ];

    let scale = size * 3f64.sqrt() / 2.0;
    faces
        .iter()
        .flat_map(|face| {
            let normal = (face[1] - face[0]).cross(&(face[2] - face[1]));
            let transformed_face = face.iter().map(|v| centre + v * scale).collect();
            triangulate_polygon(&transformed_face, &normal, Arc::clone(&material))
        })
        .collect()
}

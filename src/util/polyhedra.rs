use itertools::izip;

use crate::materials::Material;
use crate::math::Vec3;
use crate::raycasting::{Primitive, Triangle};

use std::sync::Arc;

pub fn triangulate_polygon(
    vertices: &[Vec3],
    normal: &Vec3,
    material: Arc<dyn Material>,
) -> Vec<Arc<dyn Primitive>> {
    assert!(vertices.len() >= 3);
    let hinge = vertices[0];
    izip!(vertices.iter().skip(1), vertices.iter().skip(2))
        .map(|(a, b)| {
            Arc::new(Triangle {
                vertices: [hinge, *a, *b],
                normals: [*normal, *normal, *normal],
                material: Arc::clone(&material),
            }) as Arc<dyn Primitive>
        })
        .collect()
}

pub fn generate_dodecahedron(
    centre: Vec3,
    size: f64,
    material: Arc<dyn Material>,
) -> Vec<Arc<dyn Primitive>> {
    let phi = (1.0 + (5.0_f64).sqrt()) / 2.0;
    let phi_inv = 1.0 / phi;

    let faces = vec![
        vec![
            Vec3::new(phi_inv, 0.0, phi),
            Vec3::new(-phi_inv, 0.0, phi),
            Vec3::new(-1.0, -1.0, 1.0),
            Vec3::new(0.0, -phi, phi_inv),
            Vec3::new(1.0, -1.0, 1.0),
        ],
        vec![
            Vec3::new(phi_inv, 0.0, phi),
            Vec3::new(-phi_inv, 0.0, phi),
            Vec3::new(-1.0, 1.0, 1.0),
            Vec3::new(0.0, phi, phi_inv),
            Vec3::new(1.0, 1.0, 1.0),
        ],
        vec![
            Vec3::new(phi_inv, 0.0, phi),
            Vec3::new(1.0, -1.0, 1.0),
            Vec3::new(phi, -phi_inv, 0.0),
            Vec3::new(phi, phi_inv, 0.0),
            Vec3::new(1.0, 1.0, 1.0),
        ],
        vec![
            Vec3::new(-phi_inv, 0.0, phi),
            Vec3::new(-1.0, -1.0, 1.0),
            Vec3::new(-phi, -phi_inv, 0.0),
            Vec3::new(-phi, phi_inv, 0.0),
            Vec3::new(-1.0, 1.0, 1.0),
        ],
        vec![
            Vec3::new(-1.0, -1.0, 1.0),
            Vec3::new(-phi, -phi_inv, 0.0),
            Vec3::new(-1.0, -1.0, -1.0),
            Vec3::new(0.0, -phi, -phi_inv),
            Vec3::new(0.0, -phi, phi_inv),
        ],
        vec![
            Vec3::new(0.0, -phi, phi_inv),
            Vec3::new(0.0, -phi, -phi_inv),
            Vec3::new(1.0, -1.0, -1.0),
            Vec3::new(phi, -phi_inv, 0.0),
            Vec3::new(1.0, -1.0, 1.0),
        ],
        vec![
            Vec3::new(0.0, phi, phi_inv),
            Vec3::new(0.0, phi, -phi_inv),
            Vec3::new(-1.0, 1.0, -1.0),
            Vec3::new(-phi, phi_inv, 0.0),
            Vec3::new(-1.0, 1.0, 1.0),
        ],
        vec![
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(phi, phi_inv, 0.0),
            Vec3::new(1.0, 1.0, -1.0),
            Vec3::new(0.0, phi, -phi_inv),
            Vec3::new(0.0, phi, phi_inv),
        ],
        vec![
            Vec3::new(1.0, -1.0, -1.0),
            Vec3::new(0.0, -phi, -phi_inv),
            Vec3::new(-1.0, -1.0, -1.0),
            Vec3::new(-phi_inv, 0.0, -phi),
            Vec3::new(phi_inv, 0.0, -phi),
        ],
        vec![
            Vec3::new(1.0, 1.0, -1.0),
            Vec3::new(0.0, phi, -phi_inv),
            Vec3::new(-1.0, 1.0, -1.0),
            Vec3::new(-phi_inv, 0.0, -phi),
            Vec3::new(phi_inv, 0.0, -phi),
        ],
        vec![
            Vec3::new(1.0, 1.0, -1.0),
            Vec3::new(phi, phi_inv, 0.0),
            Vec3::new(phi, -phi_inv, 0.0),
            Vec3::new(1.0, -1.0, -1.0),
            Vec3::new(phi_inv, 0.0, -phi),
        ],
        vec![
            Vec3::new(-1.0, 1.0, -1.0),
            Vec3::new(-phi, phi_inv, 0.0),
            Vec3::new(-phi, -phi_inv, 0.0),
            Vec3::new(-1.0, -1.0, -1.0),
            Vec3::new(-phi_inv, 0.0, -phi),
        ],
    ];

    let scale = size * 3f64.sqrt() / 2.0;
    faces
        .iter()
        .flat_map(|face| {
            let normal = (face[1] - face[0]).cross(&(face[2] - face[1]));
            let transformed_face: Vec<_> = face.iter().map(|v| centre + v * scale).collect();
            triangulate_polygon(&transformed_face, &normal, Arc::clone(&material))
        })
        .collect()
}

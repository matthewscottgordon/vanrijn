use nalgebra::{Point3, RealField, Vector2, Vector3};

use super::materials::Material;
use super::raycasting::{Intersect, IntersectionInfo, Ray};

use std::rc::Rc;

pub struct Triangle<T: RealField> {
    pub vertices: [Point3<T>; 3],
    pub normals: [Vector3<T>; 3],
    pub material: Rc<dyn Material<T>>,
}

impl<T: RealField> Intersect<T> for Triangle<T> {
    fn intersect<'a>(&'a self, ray: &Ray<T>) -> Option<IntersectionInfo<T>> {
        let translation = -ray.origin.coords;
        let indices = indices_with_index_of_largest_element_last(&ray.direction);
        let permuted_ray_direction = permute_vector_elements(&ray.direction, &indices);
        let shear_slopes = calculate_shear_to_z_axis(&permuted_ray_direction);
        let transformed_vertices: Vec<Vector3<T>> = self
            .vertices
            .iter()
            .map(|elem| {
                apply_shear_to_z_axis(
                    &permute_vector_elements(&(elem.coords + translation), &indices),
                    &shear_slopes,
                )
            })
            .collect();
        let edge_functions = signed_edge_functions(&transformed_vertices);
        if edge_functions.iter().all(|e| e.is_sign_positive())
            || edge_functions.iter().all(|e| e.is_sign_negative())
        {
            let barycentric_coordinates = barycentric_coordinates_from_signed_edge_functions(
                Vector3::from_iterator(edge_functions.iter().map(|e| e.abs())),
            );
            let transformed_z = barycentric_coordinates
                .iter()
                .zip(transformed_vertices.iter())
                .map(|(&coord, vertex)| vertex.z * coord)
                .fold(T::zero(), |acc, z| acc + z);
            if transformed_z.is_positive() != permuted_ray_direction.z.is_positive() {
                return None;
            }
            let location: Point3<T> = barycentric_coordinates
                .iter()
                .zip(self.vertices.iter())
                .map(|(&barycentric_coord, vertex)| vertex.coords * barycentric_coord)
                .fold(Point3::new(T::zero(), T::zero(), T::zero()), |a, e| a + e);
            let distance = (ray.origin - location).norm();
            let normal: Vector3<T> = barycentric_coordinates
                .iter()
                .zip(self.normals.iter())
                .fold(Vector3::zeros(), |acc, (&coord, vertex)| {
                    acc + vertex * coord
                })
                .normalize();
            let cotangent = (self.vertices[0] - self.vertices[1])
                .cross(&normal)
                .normalize();
            let tangent = cotangent.cross(&normal).normalize();
            let retro = (ray.origin - location).normalize();
            let material = Rc::clone(&self.material);
            Some(IntersectionInfo {
                distance,
                location,
                normal,
                tangent,
                cotangent,
                retro,
                material,
            })
        } else {
            None
        }
    }
}

fn indices_with_index_of_largest_element_last<T: RealField>(v: &Vector3<T>) -> [usize; 3] {
    if v.x > v.y {
        if v.z > v.x {
            [0, 1, 2]
        } else {
            [1, 2, 0]
        }
    } else {
        if v.z > v.y {
            [0, 1, 2]
        } else {
            [2, 0, 1]
        }
    }
}

fn is_valid_permutation(indices: &[usize; 3]) -> bool {
    (0..2).all(|i: usize| indices.iter().any(|&j| j == i))
}

fn permute_vector_elements<T: RealField>(v: &Vector3<T>, indices: &[usize; 3]) -> Vector3<T> {
    debug_assert!(is_valid_permutation(&indices));
    Vector3::new(v[indices[0]], v[indices[1]], v[indices[2]])
}

fn calculate_shear_to_z_axis<T: RealField>(v: &Vector3<T>) -> Vector2<T> {
    Vector2::new(-v.x / v.z, -v.y / v.z)
}

fn apply_shear_to_z_axis<T: RealField>(v: &Vector3<T>, s: &Vector2<T>) -> Vector3<T> {
    Vector3::new(v.x + s.x * v.z, v.y + s.y * v.z, v.z)
}

fn signed_edge_function<T: RealField>(a: &Vector3<T>, b: &Vector3<T>) -> T {
    a.x * b.y - b.x * a.y
}

fn signed_edge_functions<T: RealField>(vertices: &Vec<Vector3<T>>) -> Vector3<T> {
    // Iterate over the inputs in such a way that each output element is calculated
    // from the twoother elements of the input. ( (y,z) -> x, (z,x) -> y, (x,y) -> z )
    Vector3::from_iterator(
        vertices
            .iter()
            .cycle()
            .skip(1)
            .zip(vertices.iter().cycle().skip(2))
            .take(vertices.len())
            .map(|(v1, v2)| signed_edge_function(v1, v2)),
    )
}

fn barycentric_coordinates_from_signed_edge_functions<T: RealField>(e: Vector3<T>) -> Vector3<T> {
    e * (T::one() / e.iter().fold(T::zero(), |a, &b| a + b))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod index_of_largest_element {
        use super::*;
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn result_is_valid_permutation(v: Vector3<f32>) -> bool {
            let indices = indices_with_index_of_largest_element_last(&v);
            is_valid_permutation(&indices)
        }

        #[quickcheck]
        fn result_includes_x(v: Vector3<f32>) -> bool {
            let indices = indices_with_index_of_largest_element_last(&v);
            indices.iter().any(|&i| i == 0)
        }

        #[quickcheck]
        fn result_includes_y(v: Vector3<f32>) -> bool {
            let indices = indices_with_index_of_largest_element_last(&v);
            indices.iter().any(|&i| i == 1)
        }

        #[quickcheck]
        fn result_includes_z(v: Vector3<f32>) -> bool {
            let indices = indices_with_index_of_largest_element_last(&v);
            indices.iter().any(|&i| i == 2)
        }

        #[quickcheck]
        fn last_index_is_greater_than_or_equal_to_x(v: Vector3<f32>) -> bool {
            let indices = indices_with_index_of_largest_element_last(&v);
            v[indices[2]] >= v.x
        }

        #[quickcheck]
        fn last_index_is_greater_than_or_equal_to_y(v: Vector3<f32>) -> bool {
            let indices = indices_with_index_of_largest_element_last(&v);
            v[indices[2]] >= v.y
        }

        #[quickcheck]
        fn last_index_is_greater_than_or_equal_to_z(v: Vector3<f32>) -> bool {
            let indices = indices_with_index_of_largest_element_last(&v);
            v[indices[2]] >= v.z
        }
    }

    mod permute_vector_elements {
        use super::*;
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn permute_and_reverse_yields_input(v: Vector3<f32>) -> bool {
            let indices = indices_with_index_of_largest_element_last(&v);
            v == reverse_permute_vector_elements(&permute_vector_elements(&v, &indices), &indices)
        }

        #[quickcheck]
        fn last_index_is_greater_than_or_equal_to_x(v: Vector3<f32>) -> bool {
            let p = permute_vector_elements(&v, &indices_with_index_of_largest_element_last(&v));
            p.z >= v.x
        }

        #[quickcheck]
        fn last_index_is_greater_than_or_equal_to_y(v: Vector3<f32>) -> bool {
            let p = permute_vector_elements(&v, &indices_with_index_of_largest_element_last(&v));
            p.z >= v.y
        }

        #[quickcheck]
        fn last_index_is_greater_than_or_equal_to_z(v: Vector3<f32>) -> bool {
            let p = permute_vector_elements(&v, &indices_with_index_of_largest_element_last(&v));
            p.z >= v.z
        }
    }

    mod shear_to_z_axis {
        use super::*;
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn shear_to_z_axis_makes_x_zero(v: Vector3<f32>) -> bool {
            let s = calculate_shear_to_z_axis(&v);
            apply_shear_to_z_axis(&v, &s).x.abs() < 0.00001
        }

        #[quickcheck]
        fn shear_to_z_axis_makes_y_zero(v: Vector3<f32>) -> bool {
            let s = calculate_shear_to_z_axis(&v);
            apply_shear_to_z_axis(&v, &s).y.abs() < 0.00001
        }

        #[quickcheck]
        fn shear_to_z_axis_leaves_z_unchanged(v: Vector3<f32>) -> bool {
            let s = calculate_shear_to_z_axis(&v);
            apply_shear_to_z_axis(&v, &s).z == v.z
        }
    }

    mod barycentric_coordinates {
        use super::*;
        use quickcheck::TestResult;
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn sign_of_signed_edge_function_matches_winding(
            a: Vector3<f32>,
            b: Vector3<f32>,
        ) -> TestResult {
            let a_2d = Vector2::new(a.x, a.y);
            let b_2d = Vector2::new(b.x, b.y);
            let c_2d = Vector2::new(0.0, 0.0);
            let winding = (b_2d - a_2d).perp(&(c_2d - b_2d));
            if winding.abs() < 0.00001 {
                TestResult::discard()
            } else {
                let winding = winding.is_sign_positive();
                let area_sign = signed_edge_function(&a, &b).is_sign_positive();
                TestResult::from_bool(winding == area_sign)
            }
        }

        #[quickcheck]
        fn signed_edge_functions_has_same_result_as_signed_edge_function(
            a: Vector3<f32>,
            b: Vector3<f32>,
            c: Vector3<f32>,
        ) -> bool {
            let es = signed_edge_functions(&vec![a, b, c]);
            es[0] == signed_edge_function(&b, &c)
                && es[1] == signed_edge_function(&c, &a)
                && es[2] == signed_edge_function(&a, &b)
        }

        #[quickcheck]
        fn barycentric_coordinates_sum_to_one(
            a: Vector3<f64>,
            b: Vector3<f64>,
            c: Vector3<f64>,
        ) -> bool {
            let barycentric_coordinates =
                barycentric_coordinates_from_signed_edge_functions(signed_edge_functions(&vec![
                    a, b, c,
                ]));
            (barycentric_coordinates.iter().fold(0.0, |a, b| a + b) - 1.0).abs() < 0.00000001
        }
    }

    mod triangle_intersect {
        use super::*;
        use crate::materials::LambertianMaterial;
        use quickcheck::{Arbitrary, TestResult};
        use quickcheck_macros::quickcheck;

        #[test]
        fn intersection_passes_with_ray_along_z_axis_ccw_winding() {
            let target_triangle = Triangle {
                vertices: [
                    Point3::new(0.0, 1.0, 1.0),
                    Point3::new(1.0, -1.0, 1.0),
                    Point3::new(-1.0, -1.0, 1.0),
                ],
                normals: [Vector3::zeros(); 3],
                material: Rc::new(LambertianMaterial::new_dummy()),
            };
            let target_ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
            if let None = target_triangle.intersect(&target_ray) {
                panic!()
            }
        }

        #[test]
        fn intersection_passes_with_ray_along_z_axis_cw_winding() {
            let target_triangle = Triangle {
                vertices: [
                    Point3::new(0.0, 1.0, 1.0),
                    Point3::new(-1.0, -1.0, 1.0),
                    Point3::new(1.0, -1.0, 1.0),
                ],
                normals: [Vector3::zeros(); 3],
                material: Rc::new(LambertianMaterial::new_dummy()),
            };
            let target_ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
            if let None = target_triangle.intersect(&target_ray) {
                panic!()
            }
        }

        #[test]
        fn intersection_passes_with_ray_along_nagative_z_axis_ccw_winding() {
            let target_triangle = Triangle {
                vertices: [
                    Point3::new(0.0, 1.0, -1.0),
                    Point3::new(1.0, -1.0, -1.0),
                    Point3::new(-1.0, -1.0, -1.0),
                ],
                normals: [Vector3::zeros(); 3],
                material: Rc::new(LambertianMaterial::new_dummy()),
            };
            let target_ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0));
            if let None = target_triangle.intersect(&target_ray) {
                panic!()
            }
        }

        #[test]
        fn intersection_passes_with_ray_along_negativez_axis_cw_winding() {
            let target_triangle = Triangle {
                vertices: [
                    Point3::new(0.0, 1.0, -1.0),
                    Point3::new(-1.0, -1.0, -1.0),
                    Point3::new(1.0, -1.0, -1.0),
                ],
                normals: [Vector3::zeros(); 3],
                material: Rc::new(LambertianMaterial::new_dummy()),
            };
            let target_ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0));
            if let None = target_triangle.intersect(&target_ray) {
                panic!()
            }
        }

        #[test]
        fn intersection_passes_with_ray_along_z_axis_but_translated_ccw_winding() {
            let target_triangle = Triangle {
                vertices: [
                    Point3::new(5.0, 6.0, 6.0),
                    Point3::new(6.0, 4.0, 6.0),
                    Point3::new(4.0, 4.0, 6.0),
                ],
                normals: [Vector3::zeros(); 3],
                material: Rc::new(LambertianMaterial::new_dummy()),
            };
            let target_ray = Ray::new(Point3::new(5.0, 5.0, 5.0), Vector3::new(0.0, 0.0, 1.0));
            if let None = target_triangle.intersect(&target_ray) {
                panic!()
            }
        }

        #[test]
        fn intersection_passes_with_ray_at_angle_to_z_axisand_translated_ccw_winding() {
            let target_triangle = Triangle {
                vertices: [
                    Point3::new(6.0, 6.5, 6.0),
                    Point3::new(7.0, 4.5, 6.0),
                    Point3::new(5.0, 4.5, 6.0),
                ],
                normals: [Vector3::zeros(); 3],
                material: Rc::new(LambertianMaterial::new_dummy()),
            };
            let target_ray = Ray::new(Point3::new(5.0, 5.0, 5.0), Vector3::new(1.0, 0.5, 1.0));
            if let None = target_triangle.intersect(&target_ray) {
                panic!()
            }
        }

        fn intersect_with_centroid_and_test_result<
            F: Fn(Option<IntersectionInfo<f64>>, Point3<f64>) -> bool,
        >(
            vertex0: Point3<f64>,
            vertex1: Point3<f64>,
            vertex2: Point3<f64>,
            ray_origin: Point3<f64>,
            test: F,
        ) -> TestResult {
            let centroid: Point3<f64> = [vertex0.coords, vertex1.coords, vertex2.coords]
                .iter()
                .fold(Point3::new(0.0, 0.0, 0.0), |acc, &elem| acc + elem)
                / 3.0;
            let ray_direction = (centroid - ray_origin).normalize();
            let normal = (vertex1 - vertex0).cross(&(vertex2 - vertex0)).normalize();
            if normal.dot(&ray_direction).abs() < 0.000_000_1 {
                //Discard if triangle is too close to edge-on
                return TestResult::discard();
            }
            let target_triangle = Triangle {
                vertices: [
                    Point3::from(vertex0),
                    Point3::from(vertex1),
                    Point3::from(vertex2),
                ],
                normals: [normal; 3],
                material: Rc::new(LambertianMaterial::new_dummy()),
            };
            let ray = Ray::new(ray_origin, ray_direction);

            TestResult::from_bool(test(target_triangle.intersect(&ray), centroid))
        }

        #[quickcheck]
        fn intersection_with_centroid_hits(
            vertex0: Point3<f64>,
            vertex1: Point3<f64>,
            vertex2: Point3<f64>,
            ray_origin: Point3<f64>,
        ) -> TestResult {
            let centroid: Point3<f64> = [vertex0.coords, vertex1.coords, vertex2.coords]
                .iter()
                .fold(Point3::new(0.0, 0.0, 0.0), |acc, &elem| acc + elem)
                / 3.0;
            let ray_direction = (centroid - ray_origin).normalize();
            let normal = (vertex1 - vertex0).cross(&(vertex2 - vertex0)).normalize();
            if normal.dot(&ray_direction).abs() < 0.000_000_1 {
                //Discard if triangle is too close to edge-on
                return TestResult::discard();
            }
            let target_triangle = Triangle {
                vertices: [
                    Point3::from(vertex0),
                    Point3::from(vertex1),
                    Point3::from(vertex2),
                ],
                normals: [normal; 3],
                material: Rc::new(LambertianMaterial::new_dummy()),
            };
            let ray = Ray::new(ray_origin, ray_direction);

            if let Some(_) = target_triangle.intersect(&ray) {
                TestResult::passed()
            } else {
                TestResult::failed()
            }
        }

        #[quickcheck]
        fn intersection_with_centroid_hits_centroid(
            vertex0: Point3<f64>,
            vertex1: Point3<f64>,
            vertex2: Point3<f64>,
            ray_origin: Point3<f64>,
        ) -> TestResult {
            intersect_with_centroid_and_test_result(
                vertex0,
                vertex1,
                vertex2,
                ray_origin,
                |result, centroid| {
                    if let Some(IntersectionInfo { location, .. }) = result {
                        (location - centroid).norm() < 0.000_000_1
                    } else {
                        false
                    }
                },
            )
        }

        #[quickcheck]
        fn intersection_with_centroid_hits_at_expected_distance(
            vertex0: Point3<f64>,
            vertex1: Point3<f64>,
            vertex2: Point3<f64>,
            ray_origin: Point3<f64>,
        ) -> TestResult {
            intersect_with_centroid_and_test_result(
                vertex0,
                vertex1,
                vertex2,
                ray_origin,
                |result, centroid| {
                    if let Some(IntersectionInfo { distance, .. }) = result {
                        ((ray_origin - centroid).norm() - distance).abs() < 0.000_000_1
                    } else {
                        false
                    }
                },
            )
        }

        #[quickcheck]
        fn intersection_with_centroid_has_expected_normal(
            vertex0: Point3<f64>,
            vertex1: Point3<f64>,
            vertex2: Point3<f64>,
            ray_origin: Point3<f64>,
        ) -> TestResult {
            intersect_with_centroid_and_test_result(
                vertex0,
                vertex1,
                vertex2,
                ray_origin,
                |result, _| {
                    if let Some(IntersectionInfo { normal, .. }) = result {
                        (normal - (vertex1 - vertex0).cross(&(vertex2 - vertex0)).normalize())
                            .norm()
                            < 0.000_000_1
                    } else {
                        false
                    }
                },
            )
        }

        #[quickcheck]
        fn intersection_with_centroid_has_expected_retro(
            vertex0: Point3<f64>,
            vertex1: Point3<f64>,
            vertex2: Point3<f64>,
            ray_origin: Point3<f64>,
        ) -> TestResult {
            intersect_with_centroid_and_test_result(
                vertex0,
                vertex1,
                vertex2,
                ray_origin,
                |result, centroid| {
                    let expected_retro = (ray_origin - centroid).normalize();
                    if let Some(IntersectionInfo { retro, .. }) = result {
                        (expected_retro - retro).norm() < 0.000_000_1
                    } else {
                        false
                    }
                },
            )
        }

        #[derive(Clone, Copy, Debug)]
        struct BarycentricCoords {
            alpha: f64,
            beta: f64,
            gamma: f64,
        }

        impl quickcheck::Arbitrary for BarycentricCoords {
            fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
                let e = 0.000_000_1;
                let alpha = <f64 as Arbitrary>::arbitrary(g).abs().fract() * (1.0 - e) + e;
                let beta = <f64 as Arbitrary>::arbitrary(g).abs().fract() * (1.0 - alpha) + e;
                let gamma = 1.0 - (alpha + beta);
                BarycentricCoords { alpha, beta, gamma }
            }
        }

        fn intersect_with_barycentric_and_test_result<
            F: Fn(Option<IntersectionInfo<f64>>, Point3<f64>) -> bool,
        >(
            vertex0: Point3<f64>,
            vertex1: Point3<f64>,
            vertex2: Point3<f64>,
            ray_origin: Point3<f64>,
            barycentric_coords: BarycentricCoords,
            test: F,
        ) -> TestResult {
            let point = vertex0 * barycentric_coords.alpha
                + vertex1.coords * barycentric_coords.beta
                + vertex2.coords * barycentric_coords.gamma;
            let ray_direction = (point - ray_origin).normalize();
            let normal = (vertex1 - vertex0).cross(&(vertex2 - vertex0)).normalize();
            if normal.dot(&ray_direction).abs() < 0.000_000_1 {
                //Discard if triangle is too close to edge-on
                return TestResult::discard();
            }
            let target_triangle = Triangle {
                vertices: [
                    Point3::from(vertex0),
                    Point3::from(vertex1),
                    Point3::from(vertex2),
                ],
                normals: [normal; 3],
                material: Rc::new(LambertianMaterial::new_dummy()),
            };
            let ray = Ray::new(ray_origin, ray_direction);

            TestResult::from_bool(test(target_triangle.intersect(&ray), point))
        }

        #[quickcheck]
        fn point_with_arbitrary_barycentric_coords_hits(
            vertex0: Point3<f64>,
            vertex1: Point3<f64>,
            vertex2: Point3<f64>,
            ray_origin: Point3<f64>,
            barycentric_coords: BarycentricCoords,
        ) -> TestResult {
            intersect_with_barycentric_and_test_result(
                vertex0,
                vertex1,
                vertex2,
                ray_origin,
                barycentric_coords,
                |result, _point| {
                    if let Some(_) = result {
                        true
                    } else {
                        false
                    }
                },
            )
        }

        #[quickcheck]
        fn point_with_arbitrary_barycentric_coords_has_expected_normal(
            vertex0: Point3<f64>,
            vertex1: Point3<f64>,
            vertex2: Point3<f64>,
            ray_origin: Point3<f64>,
            barycentric_coords: BarycentricCoords,
        ) -> TestResult {
            intersect_with_barycentric_and_test_result(
                vertex0,
                vertex1,
                vertex2,
                ray_origin,
                barycentric_coords,
                |result, _point| {
                    let expected_normal =
                        (vertex1 - vertex0).cross(&(vertex2 - vertex0)).normalize();
                    if let Some(IntersectionInfo { normal, .. }) = result {
                        (normal - expected_normal).norm().abs() < 0.000_01
                    } else {
                        false
                    }
                },
            )
        }

        #[quickcheck]
        fn point_with_arbitrary_barycentric_coords_has_expected_distance(
            vertex0: Point3<f64>,
            vertex1: Point3<f64>,
            vertex2: Point3<f64>,
            ray_origin: Point3<f64>,
            barycentric_coords: BarycentricCoords,
        ) -> TestResult {
            intersect_with_barycentric_and_test_result(
                vertex0,
                vertex1,
                vertex2,
                ray_origin,
                barycentric_coords,
                |result, point| {
                    let expected_distance = (point - ray_origin).norm();
                    if let Some(IntersectionInfo { distance, .. }) = result {
                        (distance - expected_distance).abs() < 0.000_01
                    } else {
                        false
                    }
                },
            )
        }

        #[quickcheck]
        fn point_with_arbitrary_barycentric_coords_has_expected_retro(
            vertex0: Point3<f64>,
            vertex1: Point3<f64>,
            vertex2: Point3<f64>,
            ray_origin: Point3<f64>,
            barycentric_coords: BarycentricCoords,
        ) -> TestResult {
            intersect_with_barycentric_and_test_result(
                vertex0,
                vertex1,
                vertex2,
                ray_origin,
                barycentric_coords,
                |result, point| {
                    let expected_retro = (ray_origin - point).normalize();
                    if let Some(IntersectionInfo { retro, .. }) = result {
                        (retro - expected_retro).norm().abs() < 0.000_01
                    } else {
                        false
                    }
                },
            )
        }

        #[quickcheck]
        fn intersection_fails_when_ray_outside_first_edge(
            vertex0: Point3<f64>,
            vertex1: Point3<f64>,
            vertex2: Point3<f64>,
            ray_origin: Point3<f64>,
            uv: Vector2<f64>,
        ) -> bool {
            let uv_origin = Point3::from(vertex0);
            let u_axis = (vertex1 - vertex0).normalize();
            let w_axis = (vertex2 - vertex0).cross(&u_axis).normalize();
            let v_axis = w_axis.cross(&u_axis);
            let target_point = uv_origin + u_axis * uv.x + v_axis * uv.y.abs();
            let ray = Ray {
                origin: ray_origin,
                direction: (target_point - ray_origin).normalize(),
            };
            let triangle = Triangle {
                vertices: [vertex0, vertex1, vertex2],
                normals: [Vector3::zeros(); 3],
                material: Rc::new(LambertianMaterial::new_dummy()),
            };
            match triangle.intersect(&ray) {
                Some(_) => false,
                None => true,
            }
        }

        #[quickcheck]
        fn intersection_fails_when_ray_outside_second_edge(
            vertex0: Point3<f64>,
            vertex1: Point3<f64>,
            vertex2: Point3<f64>,
            ray_origin: Point3<f64>,
            uv: Vector2<f64>,
        ) -> bool {
            let uv_origin = Point3::from(vertex0);
            let u_axis = (vertex2 - vertex1).normalize();
            let w_axis = (vertex1 - vertex0).cross(&u_axis).normalize();
            let v_axis = w_axis.cross(&u_axis);
            let target_point = uv_origin + u_axis * uv.x + v_axis * uv.y.abs();
            let ray = Ray {
                origin: ray_origin,
                direction: (target_point - ray_origin).normalize(),
            };
            let triangle = Triangle {
                vertices: [vertex0, vertex1, vertex2],
                normals: [Vector3::zeros(); 3],
                material: Rc::new(LambertianMaterial::new_dummy()),
            };
            match triangle.intersect(&ray) {
                Some(_) => false,
                None => true,
            }
        }

        #[quickcheck]
        fn intersection_fails_when_ray_outside_third_edge(
            vertex0: Point3<f64>,
            vertex1: Point3<f64>,
            vertex2: Point3<f64>,
            ray_origin: Point3<f64>,
            uv: Vector2<f64>,
        ) -> bool {
            let uv_origin = Point3::from(vertex0);
            let u_axis = (vertex0 - vertex2).normalize();
            let w_axis = (vertex1 - vertex2).cross(&u_axis).normalize();
            let v_axis = w_axis.cross(&u_axis);
            let target_point = uv_origin + u_axis * uv.x + v_axis * uv.y.abs();
            let ray = Ray {
                origin: ray_origin,
                direction: (target_point - ray_origin).normalize(),
            };
            let triangle = Triangle {
                vertices: [vertex0, vertex1, vertex2],
                normals: [Vector3::zeros(); 3],
                material: Rc::new(LambertianMaterial::new_dummy()),
            };
            match triangle.intersect(&ray) {
                Some(_) => false,
                None => true,
            }
        }

        #[quickcheck]
        fn intersection_fails_when_triangle_is_behind_ray(
            vertex0: Point3<f64>,
            vertex1: Point3<f64>,
            vertex2: Point3<f64>,
            ray_origin: Point3<f64>,
            barycentric_coords: BarycentricCoords,
        ) -> bool {
            let point_behind_ray = vertex0.coords * barycentric_coords.alpha
                + vertex1.coords * barycentric_coords.beta
                + vertex2.coords * barycentric_coords.gamma;
            let ray = Ray {
                origin: ray_origin,
                direction: (ray_origin.coords - point_behind_ray).normalize(),
            };
            let triangle = Triangle {
                vertices: [vertex0, vertex1, vertex2],
                normals: [Vector3::zeros(); 3],
                material: Rc::new(LambertianMaterial::new_dummy()),
            };
            match triangle.intersect(&ray) {
                Some(_) => false,
                None => true,
            }
        }
    }
}

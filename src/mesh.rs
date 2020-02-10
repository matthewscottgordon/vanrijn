mod wavefront_obj {
    use crate::materials::Material;
    use crate::Real;

    use crate::raycasting::Triangle;

    use alga::general::SupersetOf;
    use nalgebra::{convert, Point3, Vector3};
    use obj::{IndexTuple, Obj, SimplePolygon};

    use std::io::Result;
    use std::path::Path;
    use std::sync::Arc;

    fn get_vertex_and_normal<T: Real>(
        index_tuple: &IndexTuple,
        vertex_positions: &Vec<[f32; 3]>,
        normal_positions: &Vec<[f32; 3]>,
    ) -> (Point3<T>, Vector3<T>)
    where
        T: SupersetOf<f32>,
    {
        let &IndexTuple(vertex_index, _, maybe_normal_index) = index_tuple;
        let vertex: Point3<T> = convert(Point3::from_slice(&vertex_positions[vertex_index]));
        let normal = match maybe_normal_index {
            Some(normal_index) => convert(Vector3::from_row_slice(&normal_positions[normal_index])),
            None => Vector3::zeros(),
        };
        (vertex, normal)
    }

    fn get_triangles<T: Real>(
        polygon: &SimplePolygon,
        vertex_positions: &Vec<[f32; 3]>,
        normal_positions: &Vec<[f32; 3]>,
        material: Arc<dyn Material<T>>,
    ) -> Vec<Triangle<T>>
    where
        T: SupersetOf<f32>,
    {
        if let Some(v0_index) = polygon.iter().next() {
            let (v0_vertex, v0_normal) =
                get_vertex_and_normal(v0_index, &vertex_positions, &normal_positions);
            polygon
                .iter()
                .skip(1)
                .zip(polygon.iter().skip(2))
                .map(|(v1_index, v2_index)| {
                    let (v1_vertex, v1_normal) =
                        get_vertex_and_normal(v1_index, &vertex_positions, &normal_positions);
                    let (v2_vertex, v2_normal) =
                        get_vertex_and_normal(v2_index, &vertex_positions, &normal_positions);
                    let vertices = [v0_vertex, v1_vertex, v2_vertex];
                    let normals = [v0_normal, v1_normal, v2_normal];
                    Triangle {
                        vertices,
                        normals,
                        material: material.clone(),
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }

    pub fn load_obj<T: Real>(
        filename: &Path,
        material: Arc<dyn Material<T>>,
    ) -> Result<Vec<Triangle<T>>>
    where
        T: SupersetOf<f32>,
    {
        let obj = Obj::<SimplePolygon>::load(filename)?;

        Ok(obj
            .objects
            .iter()
            .flat_map(|object| object.groups.iter())
            .flat_map(|group| group.polys.iter())
            .flat_map(|poly| get_triangles(poly, &obj.position, &obj.normal, material.clone()))
            .collect())
    }
}

pub use wavefront_obj::load_obj;

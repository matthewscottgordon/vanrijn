/// Load a model from a Wavefront .obj file
mod wavefront_obj {
    use crate::materials::Material;

    use crate::raycasting::{Primitive, Triangle};

    use nalgebra::{convert, Point3, Vector3};
    use obj::{IndexTuple, Obj, SimplePolygon};

    use std::io::Result;
    use std::path::Path;
    use std::sync::Arc;

    fn get_vertex_and_normal(
        index_tuple: &IndexTuple,
        vertex_positions: &[[f32; 3]],
        normal_positions: &[[f32; 3]],
    ) -> (Point3<f64>, Vector3<f64>) {
        let &IndexTuple(vertex_index, _, maybe_normal_index) = index_tuple;
        let vertex = convert(Point3::from_slice(&vertex_positions[vertex_index]));
        let normal = match maybe_normal_index {
            Some(normal_index) => convert(Vector3::from_row_slice(&normal_positions[normal_index])),
            None => Vector3::zeros(),
        };
        (vertex, normal.normalize())
    }

    fn get_triangles(
        polygon: &SimplePolygon,
        vertex_positions: &[[f32; 3]],
        normal_positions: &[[f32; 3]],
        material: Arc<dyn Material>,
    ) -> Vec<Triangle> {
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

    pub fn load_obj(
        filename: &Path,
        material: Arc<dyn Material>,
    ) -> Result<Vec<Arc<dyn Primitive>>> {
        let obj = Obj::<SimplePolygon>::load(filename)?;

        Ok(obj
            .objects
            .iter()
            .flat_map(|object| object.groups.iter())
            .flat_map(|group| group.polys.iter())
            .flat_map(|poly| get_triangles(poly, &obj.position, &obj.normal, material.clone()))
            .map(|triangle| Arc::new(triangle) as Arc<dyn Primitive>)
            .collect())
    }
}

pub use wavefront_obj::load_obj;

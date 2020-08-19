use nalgebra::{Matrix3, Vector3};

pub fn try_change_of_basis_matrix(
    x: &Vector3<f64>,
    y: &Vector3<f64>,
    z: &Vector3<f64>,
) -> Option<Matrix3<f64>> {
    Some(Matrix3::from_rows(&[
        x.transpose(),
        y.transpose(),
        z.transpose(),
    ]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod change_of_basis_matrix {
        use super::*;
        use quickcheck_macros::quickcheck;

        #[test]
        fn produces_isentity_when_passed_axes() {
            let target: Matrix3<f64> = try_change_of_basis_matrix(
                &Vector3::x_axis(),
                &Vector3::y_axis(),
                &Vector3::z_axis(),
            )
            .unwrap();
            assert!(target == Matrix3::identity())
        }

        #[quickcheck]
        fn swap_xy_does_not_change_z(v: Vector3<f64>) {
            let target: Matrix3<f64> = try_change_of_basis_matrix(
                &Vector3::y_axis(),
                &Vector3::x_axis(),
                &Vector3::z_axis(),
            )
            .unwrap();
            let v2 = target * v;
            assert!(v2.z == v.z)
        }

        #[quickcheck]
        fn swap_xy_copies_y_to_x(v: Vector3<f64>) {
            let target: Matrix3<f64> = try_change_of_basis_matrix(
                &Vector3::y_axis(),
                &Vector3::x_axis(),
                &Vector3::z_axis(),
            )
            .unwrap();
            let v2 = target * v;
            assert!(v2.x == v.y)
        }

        #[quickcheck]
        fn swap_xy_copies_x_to_y(v: Vector3<f64>) {
            let target: Matrix3<f64> = try_change_of_basis_matrix(
                &Vector3::y_axis(),
                &Vector3::x_axis(),
                &Vector3::z_axis(),
            )
            .unwrap();
            let v2 = target * v;
            assert!(v2.y == v.x)
        }
    }
}

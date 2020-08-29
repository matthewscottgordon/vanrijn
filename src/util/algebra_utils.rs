use crate::math::{Mat3, Vec3};

pub fn try_change_of_basis_matrix(x: &Vec3, y: &Vec3, z: &Vec3) -> Option<Mat3> {
    Some(Mat3::from_rows(x, y, z))
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
            let target: Mat3 =
                try_change_of_basis_matrix(&Vec3::unit_x(), &Vec3::unit_y(), &Vec3::unit_z())
                    .unwrap();
            assert!(target == Mat3::identity())
        }

        #[quickcheck]
        fn swap_xy_does_not_change_z(v: Vec3) {
            let target: Mat3 =
                try_change_of_basis_matrix(&Vec3::unit_y(), &Vec3::unit_x(), &Vec3::unit_z())
                    .unwrap();
            let v2 = target * v;
            assert!(v2.z() == v.z())
        }

        #[quickcheck]
        fn swap_xy_copies_y_to_x(v: Vec3) {
            let target: Mat3 =
                try_change_of_basis_matrix(&Vec3::unit_y(), &Vec3::unit_x(), &Vec3::unit_z())
                    .unwrap();
            let v2 = target * v;
            assert!(v2.x() == v.y())
        }

        #[quickcheck]
        fn swap_xy_copies_x_to_y(v: Vec3) {
            let target: Mat3 =
                try_change_of_basis_matrix(&Vec3::unit_y(), &Vec3::unit_x(), &Vec3::unit_z())
                    .unwrap();
            let v2 = target * v;
            assert!(v2.y() == v.x())
        }
    }
}

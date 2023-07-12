use crate::identity::Identity;
use crate::matrix_identity::MatrixIdentityOp;
use crate::matrix_types::Matrix4;
use crate::vec3::Vec3;
use crate::MathType;

pub fn translate<T: MathType + Identity>(translation: Vec3<T>) -> Matrix4<T> {
    let mut identity = Matrix4::identity();
    identity[(0, 3)] = translation.x();
    identity[(1, 3)] = translation.y();
    identity[(2, 3)] = translation.z();
    identity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_matrix_has_correct_values() {
        let translate = translate(Vec3::new(0.5, -1.0, 12.0));
        assert_eq!(
            Matrix4::new([
                [1.0, 0.0, 0.0, 0.5],
                [0.0, 1.0, 0.0, -1.0],
                [0.0, 0.0, 1.0, 12.0],
                [0.0, 0.0, 0.0, 1.0]
            ]),
            translate
        );
    }
}


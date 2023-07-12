use crate::matrix::Matrix;
use crate::MathType;

pub trait MatrixIdentityOp<T: MathType, const SIZE: usize> {
    fn identity() -> Matrix<T, SIZE, SIZE>;
}

macro_rules! impl_identity_for_type {
    ($type:ty, $identity_value:expr) => {
        impl<const SIZE: usize> MatrixIdentityOp<$type, SIZE> for Matrix<$type, SIZE, SIZE> {
            fn identity() -> Matrix<$type, SIZE, SIZE> {
                let mut values = [[<$type>::default(); SIZE]; SIZE];
                for column in 0..SIZE {
                    for row in 0..SIZE {
                        if row == column {
                            values[row][column] = $identity_value;
                        }
                    }
                }
                Matrix::new(values)
            }
        }
    };
}

impl_identity_for_type!(usize, 1);
impl_identity_for_type!(u8, 1);
impl_identity_for_type!(u16, 1);
impl_identity_for_type!(u32, 1);
impl_identity_for_type!(u64, 1);
impl_identity_for_type!(u128, 1);

impl_identity_for_type!(isize, 1);
impl_identity_for_type!(i8, 1);
impl_identity_for_type!(i16, 1);
impl_identity_for_type!(i32, 1);
impl_identity_for_type!(i64, 1);
impl_identity_for_type!(i128, 1);

impl_identity_for_type!(f32, 1.0);
impl_identity_for_type!(f64, 1.0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_of_f32_has_correct_values() {
        let identity = Matrix::<f32, 4, 4>::identity();
        assert_eq!(
            Matrix::<f32, 4, 4>::new([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]),
            identity
        );
    }
}

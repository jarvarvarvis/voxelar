use crate::identity::Identity;
use crate::matrix::Matrix;
use crate::MathType;

pub trait MatrixIdentityOp<T: MathType, const SIZE: usize> {
    fn identity() -> Matrix<T, SIZE, SIZE>;
}

impl<T: MathType + Identity, const SIZE: usize> MatrixIdentityOp<T, SIZE>
    for Matrix<T, SIZE, SIZE>
{
    fn identity() -> Matrix<T, SIZE, SIZE> {
        let mut values = [[T::default(); SIZE]; SIZE];
        for column in 0..SIZE {
            for row in 0..SIZE {
                if row == column {
                    values[row][column] = T::identity();
                }
            }
        }
        Self::new(values)
    }
}

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

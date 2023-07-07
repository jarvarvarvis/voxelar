use std::ops::*;

use crate::{error::VoxelarMathError, MathType};

#[derive(PartialEq, Clone, Debug)]
pub struct Matrix<T: MathType> {
    dimensions: (usize, usize),
    values: Vec<T>,
}

impl<T: MathType> Matrix<T> {
    pub fn empty(rows: usize, columns: usize) -> Self
    where
        T: Default,
    {
        let size = rows * columns;
        let values = (0..size).map(|_| T::default()).collect();
        Self {
            dimensions: (rows, columns),
            values,
        }
    }

    pub fn new_row_major<const ROWS: usize, const COLUMNS: usize>(
        matrix: [[T; COLUMNS]; ROWS],
    ) -> Self {
        let mut values = Vec::with_capacity(ROWS * COLUMNS);
        for row in 0..ROWS {
            for column in 0..COLUMNS {
                values.push(matrix[row][column]);
            }
        }

        Self {
            dimensions: (ROWS, COLUMNS),
            values,
        }
    }

    pub fn new_column_major<const ROWS: usize, const COLUMNS: usize>(
        matrix: [[T; COLUMNS]; ROWS],
    ) -> Self {
        let mut values = Vec::with_capacity(ROWS * COLUMNS);
        for column in 0..COLUMNS {
            for row in 0..ROWS {
                values.push(matrix[row][column]);
            }
        }

        Self {
            dimensions: (ROWS, COLUMNS),
            values,
        }
    }

    pub fn rows(&self) -> usize {
        self.dimensions.0
    }

    pub fn columns(&self) -> usize {
        self.dimensions.1
    }

    pub fn from_vec(values: Vec<T>, rows: usize, columns: usize) -> crate::Result<Self> {
        if values.len() != rows * columns {
            return Err(VoxelarMathError::InvalidFlatSize2D {
                expected_dim: (rows, columns),
                got: values.len(),
            });
        }

        Ok(Matrix {
            dimensions: (rows, columns),
            values,
        })
    }

    pub fn to_vec(self) -> Vec<T> {
        self.values
    }

    pub fn zip<ZipFn>(self, rhs: Self, op: ZipFn) -> Self
    where
        ZipFn: Fn(T, T) -> T,
    {
        assert!(
            self.dimensions == rhs.dimensions,
            "Matrix dimensions must match! {:?} != {:?}",
            self.dimensions,
            rhs.dimensions
        );
        let (rows, columns) = self.dimensions;
        let zipped = self
            .values
            .into_iter()
            .zip(rhs.values.into_iter())
            .map(|(left, right)| op(left, right));
        Matrix::from_vec(zipped.collect(), rows, columns).unwrap()
    }
}

impl<T: MathType + Add<Output = T>> Add for Matrix<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip(rhs, |left, right| left + right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_matrix_has_correct_values_and_dimensions() {
        let matrix = Matrix::empty(3, 4);

        assert_eq!((3, 4), matrix.dimensions);
        assert_eq!(3, matrix.rows());
        assert_eq!(4, matrix.columns());
        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], matrix.values);
        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], matrix.to_vec());
    }

    #[test]
    fn new_column_major_matrix_has_correct_values() {
        let matrix = Matrix::new_column_major([[1, 2, -9], [5, 0, 3]]);

        assert_eq!((2, 3), matrix.dimensions);
        assert_eq!(2, matrix.rows());
        assert_eq!(3, matrix.columns());
        assert_eq!(vec![1, 5, 2, 0, -9, 3], matrix.values);
        assert_eq!(vec![1, 5, 2, 0, -9, 3], matrix.to_vec());
    }

    #[test]
    fn new_row_major_matrix_has_correct_values() {
        let matrix = Matrix::new_row_major([[1, 2, -9], [5, 0, 3]]);

        assert_eq!((2, 3), matrix.dimensions);
        assert_eq!(2, matrix.rows());
        assert_eq!(3, matrix.columns());
        assert_eq!(vec![1, 2, -9, 5, 0, 3], matrix.values);
        assert_eq!(vec![1, 2, -9, 5, 0, 3], matrix.to_vec());
    }

    #[test]
    fn matrix_from_vec_has_correct_dimensions_and_values() {
        let matrix = Matrix::from_vec(vec![0, 1, 2, 3, 4, 5], 3, 2).unwrap();

        assert_eq!((3, 2), matrix.dimensions);
        assert_eq!(vec![0, 1, 2, 3, 4, 5], matrix.values);
        assert_eq!(vec![0, 1, 2, 3, 4, 5], matrix.to_vec());
    }

    #[test]
    fn add_matrices() {
        let first = Matrix::new_column_major([[1.0, 2.0, -9.0], [-9.0, 0.0, 4.5]]);
        let other = Matrix::new_column_major([[4.5, -6.2, 9.0], [5.5, 0.0, 3.0]]);

        let expected = Matrix::new_column_major([[5.5, -4.2, 0.0], [-3.5, 0.0, 7.5]]);

        assert_eq!(expected, first + other);
    }
}

use crate::MathType;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T: MathType, const COLUMNS: usize, const ROWS: usize> {
    values: Vec<T>,
}

impl<T: MathType, const COLUMNS: usize, const ROWS: usize> Matrix<T, COLUMNS, ROWS> {
    pub fn empty() -> Self {
        let values = vec![T::default(); ROWS * COLUMNS];
        Self { values }
    }

    pub fn new(matrix: [[T; COLUMNS]; ROWS]) -> Self {
        let mut values = Vec::with_capacity(ROWS * COLUMNS);

        for column in 0..COLUMNS {
            for row in 0..ROWS {
                values.push(matrix[row][column]);
            }
        }

        Self { values }
    }

    pub fn rows(&self) -> usize {
        ROWS
    }

    pub fn columns(&self) -> usize {
        COLUMNS
    }

    pub fn get(&self, row: usize, column: usize) -> T {
        self.values[column * ROWS + row]
    }

    pub fn get_ref(&self, row: usize, column: usize) -> &T {
        &self.values[column * ROWS + row]
    }

    pub fn get_mut(&mut self, row: usize, column: usize) -> &mut T {
        &mut self.values[column * ROWS + row]
    }
}

impl<T: MathType, const COLUMNS: usize, const ROWS: usize> std::ops::Index<(usize, usize)>
    for Matrix<T, COLUMNS, ROWS>
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get_ref(index.0, index.1)
    }
}

impl<T: MathType + std::ops::Add<Output = T>, const COLUMNS: usize, const ROWS: usize> std::ops::Add
    for Matrix<T, COLUMNS, ROWS>
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for column in 0..COLUMNS {
            for row in 0..ROWS {
                *self.get_mut(row, column) = self.get(row, column) + rhs.get(row, column);
            }
        }
        self
    }
}

impl<T: MathType + std::ops::Sub<Output = T>, const COLUMNS: usize, const ROWS: usize> std::ops::Sub
    for Matrix<T, COLUMNS, ROWS>
{
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for column in 0..COLUMNS {
            for row in 0..ROWS {
                *self.get_mut(row, column) = self.get(row, column) - rhs.get(row, column);
            }
        }
        self
    }
}

impl<T: MathType + std::ops::Mul<Output = T>, const COLUMNS: usize, const ROWS: usize>
    std::ops::Mul<T> for Matrix<T, COLUMNS, ROWS>
{
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        for column in 0..COLUMNS {
            for row in 0..ROWS {
                *self.get_mut(row, column) = self.get(row, column) * rhs;
            }
        }
        self
    }
}

impl<T: MathType + std::ops::Div<Output = T>, const COLUMNS: usize, const ROWS: usize> std::ops::Div
    for Matrix<T, COLUMNS, ROWS>
{
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        for column in 0..COLUMNS {
            for row in 0..ROWS {
                *self.get_mut(row, column) = self.get(row, column) / rhs.get(row, column);
            }
        }
        self
    }
}

impl<T: MathType + std::ops::Div<Output = T>, const COLUMNS: usize, const ROWS: usize>
    std::ops::Div<T> for Matrix<T, COLUMNS, ROWS>
{
    type Output = Self;

    fn div(mut self, rhs: T) -> Self::Output {
        for column in 0..COLUMNS {
            for row in 0..ROWS {
                *self.get_mut(row, column) = self.get(row, column) / rhs;
            }
        }
        self
    }
}

impl<
        T: MathType + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
        const COLUMNS: usize,
        const ROWS: usize,
        const OTHER_COLUMNS: usize,
    > std::ops::Mul<Matrix<T, OTHER_COLUMNS, COLUMNS>> for Matrix<T, COLUMNS, ROWS>
{
    type Output = Matrix<T, OTHER_COLUMNS, ROWS>;

    fn mul(self, rhs: Matrix<T, OTHER_COLUMNS, COLUMNS>) -> Self::Output {
        let mut result = Matrix::<T, OTHER_COLUMNS, ROWS>::empty();

        for row in 0..ROWS {
            for other_column in 0..OTHER_COLUMNS {
                let mut sum = T::default();
                for column in 0..COLUMNS {
                    sum = sum + self.get(row, column) * rhs.get(column, other_column);
                }
                *result.get_mut(row, other_column) = sum;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Mat2 = Matrix<i32, 2, 2>;
    type Mat2x3 = Matrix<i32, 2, 3>;
    type Mat2d = Matrix<f64, 2, 2>;
    type Mat2x3d = Matrix<f64, 2, 3>;

    #[test]
    fn new_mat2_has_correct_dim() {
        let matrix = Mat2::new([[0, 0], [0, 0]]);
        assert_eq!(2, matrix.rows());
        assert_eq!(2, matrix.columns());
    }

    #[test]
    fn new_mat2x3_has_correct_dim() {
        let matrix = Mat2x3::new([[0, 0], [0, 0], [0, 0]]);
        assert_eq!(3, matrix.rows());
        assert_eq!(2, matrix.columns());
    }

    #[test]
    fn new_mat2() {
        let matrix = Mat2::new([[0, 1], [4, 2]]);
        assert_eq!(vec![0, 4, 1, 2], matrix.values);
    }

    #[test]
    fn new_mat2x3() {
        let matrix = Mat2x3::new([[0, 3], [-6, 1], [2, 5]]);
        assert_eq!(vec![0, -6, 2, 3, 1, 5], matrix.values);
    }

    #[test]
    fn new_mat2_has_values_at_correct_positions() {
        let matrix = Mat2::new([[0, 6], [1, 3]]);
        assert_eq!(0, matrix.get(0, 0));
        assert_eq!(6, matrix.get(0, 1));
        assert_eq!(1, matrix.get(1, 0));
        assert_eq!(3, matrix.get(1, 1));
    }

    #[test]
    fn new_mat2x3_has_values_at_correct_positions() {
        let matrix = Mat2x3::new([[0, 6], [1, 3], [5, -2]]);
        assert_eq!(0, matrix.get(0, 0));
        assert_eq!(6, matrix.get(0, 1));
        assert_eq!(1, matrix.get(1, 0));
        assert_eq!(3, matrix.get(1, 1));
        assert_eq!(5, matrix.get(2, 0));
        assert_eq!(-2, matrix.get(2, 1));
    }

    #[test]
    fn index_operator_on_mat2() {
        let matrix = Mat2::new([[4, 5], [1, 0]]);
        assert_eq!(4, matrix[(0, 0)]);
        assert_eq!(5, matrix[(0, 1)]);
        assert_eq!(1, matrix[(1, 0)]);
        assert_eq!(0, matrix[(1, 1)]);
    }

    #[test]
    fn index_operator_on_mat2x3() {
        let matrix = Mat2x3d::new([[1.0, 5.0], [-4.0, 3.0], [1.0, 0.0]]);
        assert_eq!(1.0, matrix[(0, 0)]);
        assert_eq!(5.0, matrix[(0, 1)]);
        assert_eq!(-4.0, matrix[(1, 0)]);
        assert_eq!(3.0, matrix[(1, 1)]);
        assert_eq!(1.0, matrix[(2, 0)]);
        assert_eq!(0.0, matrix[(2, 1)]);
    }

    #[test]
    fn add_mat2() {
        let first = Mat2d::new([[1.0, -2.1], [-5.0, 0.0]]);
        let second = Mat2d::new([[4.5, 1.1], [7.0, -9.5]]);

        let expected = Mat2d::new([[5.5, -1.0], [2.0, -9.5]]);
        assert_eq!(expected, first + second);
    }

    #[test]
    fn add_mat2x3() {
        let first = Mat2x3::new([[1, 2], [-5, 0], [9, 8]]);
        let second = Mat2x3::new([[4, 1], [7, -9], [-1, 5]]);

        let expected = Mat2x3::new([[5, 3], [2, -9], [8, 13]]);
        assert_eq!(expected, first + second);
    }

    #[test]
    fn sub_mat2() {
        let first = Mat2d::new([[1.0, -2.1], [-5.0, 0.0]]);
        let second = Mat2d::new([[4.5, 1.1], [7.0, -9.5]]);

        let expected = Mat2d::new([[-3.5, -3.2], [-12.0, 9.5]]);
        assert_eq!(expected, first - second);
    }

    #[test]
    fn sub_mat2x3() {
        let first = Mat2x3::new([[1, 2], [-5, 0], [9, 8]]);
        let second = Mat2x3::new([[4, 1], [7, -9], [-1, 5]]);

        let expected = Mat2x3::new([[-3, 1], [-12, 9], [10, 3]]);
        assert_eq!(expected, first - second);
    }

    #[test]
    fn multiply_mat2_and_mat2x3() {
        let first = Mat2x3d::new([[1.0, 0.5], [2.0, 1.5], [0.5, 4.0]]);
        let second = Mat2d::new([[5.0, -2.0], [1.5, 0.5]]);

        let result = first * second;
        let expected = Mat2x3d::new([[5.75, -1.75], [12.25, -3.25], [8.5, 1.0]]);

        assert_eq!(expected, result);
    }
}

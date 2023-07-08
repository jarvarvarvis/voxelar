use crate::mat_macros::*;
use crate::MathType;

make_mat_type! {
    Mat2 {
        size = (2, 2)
    }
}

make_mat_type! {
    Mat2x3 {
        size = (2, 3)
    }
}

make_mat_type! {
    Mat2x4 {
        size = (2, 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let matrix = Mat2::new(
            [[0, 1], 
             [4, 2]]
        );
        assert_eq!(vec![0, 4, 1, 2], matrix.values);
    }

    #[test]
    fn new_mat2x3() {
        let matrix = Mat2x3::new(
            [[0, 3], 
             [-6, 1],
             [2, 5]]
        );
        assert_eq!(vec![0, -6, 2, 3, 1, 5], matrix.values);
    }

    #[test]
    fn new_mat2_has_values_at_correct_positions() {
        let matrix = Mat2::new(
            [[0, 6], 
             [1, 3]]
        );
        assert_eq!(0, matrix.get(0, 0));
        assert_eq!(6, matrix.get(0, 1));
        assert_eq!(1, matrix.get(1, 0));
        assert_eq!(3, matrix.get(1, 1));
    }

    #[test]
    fn new_mat2x3_has_values_at_correct_positions() {
        let matrix = Mat2x3::new(
            [[0, 6], 
             [1, 3],
             [5, -2]]
        );
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
        let matrix = Mat2x3::new([[1.0, 5.0], [-4.0, 3.0], [1.0, 0.0]]);
        assert_eq!(1.0, matrix[(0, 0)]);
        assert_eq!(5.0, matrix[(0, 1)]);
        assert_eq!(-4.0, matrix[(1, 0)]);
        assert_eq!(3.0, matrix[(1, 1)]);
        assert_eq!(1.0, matrix[(2, 0)]);
        assert_eq!(0.0, matrix[(2, 1)]);
    }

    #[test]
    fn add_mat2() {
        let first = Mat2::new([[1.0, -2.1], [-5.0, 0.0]]);
        let second = Mat2::new([[4.5, 1.1], [7.0, -9.5]]);

        let expected = Mat2::new([[5.5, -1.0], [2.0, -9.5]]);
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
        let first = Mat2::new([[1.0, -2.1], [-5.0, 0.0]]);
        let second = Mat2::new([[4.5, 1.1], [7.0, -9.5]]);

        let expected = Mat2::new([[-3.5, -3.2], [-12.0, 9.5]]);
        assert_eq!(expected, first - second);
    }

    #[test]
    fn sub_mat2x3() {
        let first = Mat2x3::new([[1, 2], [-5, 0], [9, 8]]);
        let second = Mat2x3::new([[4, 1], [7, -9], [-1, 5]]);

        let expected = Mat2x3::new([[-3, 1], [-12, 9], [10, 3]]);
        assert_eq!(expected, first - second);
    }
}

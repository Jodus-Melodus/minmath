pub mod matrix;

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn test_initialization() {
        let matrix = Matrix::new([[4, 3, -2], [0, 2, 1], [0, 3, 1]]);

        println!("{}", matrix);
    }

    #[test]
    fn test_size() {
        let matrix = Matrix::new([[4, 3, -2], [0, 2, 1], [0, 3, 1]]);
        let (rows, columns) = matrix.size();

        assert!(rows == 3 && columns == 3);
    }

    #[test]
    fn test_scalar_addition() {
        let matrix = Matrix::new([[4, 3, -2], [0, 2, 1], [0, 3, 1]]);
        let scalar = 5;
        let actual = Matrix::new([[9, 8, 3], [5, 7, 6], [5, 8, 6]]);

        assert_eq!(matrix + scalar, actual);
    }

    #[test]
    fn test_scalar_subtraction() {
        let matrix = Matrix::new([[4, 3, -2], [0, 2, 1], [0, 3, 1]]);
        let scalar = 5;
        let actual = Matrix::new([[-1, -2, -7], [-5, -3, -4], [-5, -2, -4]]);

        assert_eq!(matrix - scalar, actual);
    }

    #[test]
    fn test_scalar_multiplication() {
        let matrix = Matrix::new([[4.0, 3.0, -2.0], [0.0, 2.0, 1.0], [0.0, 3.0, 1.0]]);
        let scalar = 5.0;
        let actual = Matrix::new([[20.0, 15.0, -10.0], [0.0, 10.0, 5.0], [0.0, 15.0, 5.0]]);

        assert_eq!(matrix * scalar, actual);
    }

    #[test]
    fn test_scalar_division() {
        let matrix = Matrix::new([[4.0, 3.0, -2.0], [0.0, 2.0, 1.0], [0.0, 3.0, 1.0]]);
        let scalar = 5.0;
        let actual = Matrix::new([
            [4.0 / 5.0, 3.0 / 5.0, -2.0 / 5.0],
            [0.0, 2.0 / 5.0, 1.0 / 5.0],
            [0.0, 3.0 / 5.0, 1.0 / 5.0],
        ]);

        assert_eq!(matrix / scalar, actual);
    }

    #[test]
    fn test_scalar_addition_assign() {
        let mut matrix = Matrix::new([[4, 3, -2], [0, 2, 1], [0, 3, 1]]);
        let scalar = 5;
        matrix += scalar;
        let actual = Matrix::new([[9, 8, 3], [5, 7, 6], [5, 8, 6]]);

        assert_eq!(matrix, actual);
    }

    #[test]
    fn test_scalar_subtraction_assign() {
        let mut matrix = Matrix::new([[4, 3, -2], [0, 2, 1], [0, 3, 1]]);
        let scalar = 5;
        matrix -= scalar;
        let actual = Matrix::new([[-1, -2, -7], [-5, -3, -4], [-5, -2, -4]]);

        assert_eq!(matrix, actual);
    }

    #[test]
    fn test_scalar_multiplication_assign() {
        let mut matrix = Matrix::new([[4.0, 3.0, -2.0], [0.0, 2.0, 1.0], [0.0, 3.0, 1.0]]);
        let scalar = 5.0;
        matrix *= scalar;
        let actual = Matrix::new([[20.0, 15.0, -10.0], [0.0, 10.0, 5.0], [0.0, 15.0, 5.0]]);

        assert_eq!(matrix, actual);
    }

    #[test]
    fn test_scalar_division_assign() {
        let mut matrix = Matrix::new([[4.0, 3.0, -2.0], [0.0, 2.0, 1.0], [0.0, 3.0, 1.0]]);
        let scalar = 5.0;
        matrix /= scalar;
        let actual = Matrix::new([
            [4.0 / 5.0, 3.0 / 5.0, -2.0 / 5.0],
            [0.0, 2.0 / 5.0, 1.0 / 5.0],
            [0.0, 3.0 / 5.0, 1.0 / 5.0],
        ]);

        assert_eq!(matrix, actual);
    }

    #[test]
    fn test_matrix_addition() {
        let a = Matrix::new([[4, 3, -2], [0, 2, 1], [0, 3, 1]]);
        let b = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let expected = Matrix::new([[5, 5, 1], [4, 7, 7], [7, 11, 10]]);
        assert_eq!(a + b, expected);
    }

    #[test]
    fn test_matrix_subtraction() {
        let a = Matrix::new([[4, 3, -2], [0, 2, 1], [0, 3, 1]]);
        let b = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let expected = Matrix::new([[3, 1, -5], [-4, -3, -5], [-7, -5, -8]]);
        assert_eq!(a - b, expected);
    }

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let b = Matrix::new([[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]]);
        let expected = Matrix::new([[58.0, 64.0], [139.0, 154.0]]);
        assert_eq!(a * b, expected);
    }

    #[test]
    fn test_non_square_matrix_multiplication() {
        // 2x3 * 3x2 = 2x2
        let a = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let b = Matrix::new([[7, 8], [9, 10], [11, 12]]);
        let expected = Matrix::new([[58, 64], [139, 154]]);
        assert_eq!(a * b, expected);
    }

    #[test]
    fn test_rectangular_matrix_multiplication() {
        // 3x2 * 2x4 = 3x4
        let a = Matrix::new([[1, 2], [3, 4], [5, 6]]);
        let b = Matrix::new([[7, 8, 9, 10], [11, 12, 13, 14]]);
        let expected = Matrix::new([
            [
                1 * 7 + 2 * 11,
                1 * 8 + 2 * 12,
                1 * 9 + 2 * 13,
                1 * 10 + 2 * 14,
            ],
            [
                3 * 7 + 4 * 11,
                3 * 8 + 4 * 12,
                3 * 9 + 4 * 13,
                3 * 10 + 4 * 14,
            ],
            [
                5 * 7 + 6 * 11,
                5 * 8 + 6 * 12,
                5 * 9 + 6 * 13,
                5 * 10 + 6 * 14,
            ],
        ]);
        assert_eq!(a * b, expected);
    }

    #[test]
    fn test_matrix_multiplication_identity() {
        // 2x2 * 2x2 identity = original
        let a = Matrix::new([[5, 7], [6, 8]]);
        let identity = Matrix::new([[1, 0], [0, 1]]);
        assert_eq!(a * identity, a);
    }
}

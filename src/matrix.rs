use std::{
    array,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::vector::Vector;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Matrix<T, const ROWS: usize, const COLUMNS: usize>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Debug
        + Display
        + Default,
{
    data: [[T; COLUMNS]; ROWS],
}

impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    pub fn new(data: [[T; COLUMNS]; ROWS]) -> Self {
        Self { data }
    }

    pub fn size(&self) -> (usize, usize) {
        (ROWS, COLUMNS)
    }

    pub fn to_vector(&self) -> Vector<T, ROWS> {
        assert_eq!(
            COLUMNS, 1,
            "Incorrect matrix dimension: Matrix should only have one column"
        );

        let mut data = [T::default(); ROWS];

        for i in 0..ROWS {
            data[i] = self.data[i][0];
        }

        Vector::new(data)
    }

    pub fn transpose(&self) -> Matrix<T, COLUMNS, ROWS> {
        let mut transposed = [[T::default(); ROWS]; COLUMNS];

        for r in 0..ROWS {
            for c in 0..COLUMNS {
                transposed[c][r] = self.data[r][c];
            }
        }

        Matrix::new(transposed)
    }
}

impl<T> Matrix<T, 2, 2>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    pub fn determinant(&self) -> T {
        self.data[0][0] * self.data[1][1] - self.data[1][0] * self.data[0][1]
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Add for Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] + rhs.data[r][c]));

        Self { data }
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Add<T> for Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] + rhs));

        Self { data }
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> AddAssign for Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    fn add_assign(&mut self, rhs: Self) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] += rhs.data[r][c];
            }
        }
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> AddAssign<T> for Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    fn add_assign(&mut self, rhs: T) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] += rhs;
            }
        }
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Sub for Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] - rhs.data[r][c]));

        Self { data }
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Sub<T> for Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] - rhs));

        Self { data }
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> SubAssign for Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    fn sub_assign(&mut self, rhs: Self) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] -= rhs.data[r][c];
            }
        }
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> SubAssign<T> for Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    fn sub_assign(&mut self, rhs: T) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] -= rhs;
            }
        }
    }
}

impl<T, const LROWS: usize, const LCOLUMNS: usize, const RROWS: usize, const RCOLUMNS: usize>
    Mul<Matrix<T, RROWS, RCOLUMNS>> for Matrix<T, LROWS, LCOLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    type Output = Matrix<T, LROWS, RCOLUMNS>;
    fn mul(self, rhs: Matrix<T, RROWS, RCOLUMNS>) -> Self::Output {
        assert_eq!(
            LCOLUMNS, RROWS,
            "Matrix dimension mismatch: Expected left matrix with shape (a * b) and right matrix with shape (b * c), \
             but got shapes (a*{}) and ({}*c).",
            LCOLUMNS, RROWS
        );

        let mut data = [[T::default(); RCOLUMNS]; LROWS];

        for i in 0..LROWS {
            for j in 0..RCOLUMNS {
                let mut sum = T::default();
                for k in 0..LCOLUMNS {
                    sum = sum + (self.data[i][k] * rhs.data[k][j]);
                }
                data[i][j] = sum;
            }
        }

        Matrix { data }
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Mul<T> for Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] * rhs));

        Self { data }
    }
}
impl<T, const ROWS: usize, const COLUMNS: usize> MulAssign<T> for Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    fn mul_assign(&mut self, rhs: T) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] *= rhs;
            }
        }
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Div<T> for Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] / rhs));

        Self { data }
    }
}
impl<T, const ROWS: usize, const COLUMNS: usize> DivAssign<T> for Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    fn div_assign(&mut self, rhs: T) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] /= rhs;
            }
        }
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Debug for Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix ({}x{}):", ROWS, COLUMNS)?;
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                write!(f, "{:?} ", self.data[r][c])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Display for Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Copy
        + Debug
        + Display
        + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix ({}x{}):", ROWS, COLUMNS)?;
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                write!(f, "{:?} ", self.data[r][c])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod matrix_tests {
    use crate::{matrix::Matrix, vector::Vector};

    #[test]
    fn test_initialization() {
        let matrix = Matrix::new([[4, 3, -2], [0, 2, 1], [0, 3, 1]]);

        dbg!("{}", matrix);
    }

    #[test]
    fn test_size() {
        let matrix = Matrix::new([[4, 3, -2], [0, 2, 1], [0, 3, 1]]);
        let (rows, columns) = matrix.size();

        assert!(rows == 3 && columns == 3);
    }

    #[test]
    fn test_conversion() {
        let matrix = Matrix::new([[4], [0], [-3]]);
        let vector = Vector::new([4, 0, -3]);
        let vector_result = matrix.to_vector();

        dbg!("{}", vector);
        dbg!("{}", vector_result);

        assert_eq!(vector, vector_result);
    }

    #[test]
    fn test_transpose() {
        let matrix = Matrix::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let transpose = Matrix::new([[1.0, 4.0], [2.0, 5.0], [3.0, 6.0]]);
        let transposed = matrix.transpose();

        assert_eq!(transpose, transposed);
    }

    #[test]
    fn test_scalar_matrix_addition() {
        let matrix = Matrix::new([[4, 3, -2], [0, 2, 1], [0, 3, 1]]);
        let scalar = 5;
        let actual = Matrix::new([[9, 8, 3], [5, 7, 6], [5, 8, 6]]);

        assert_eq!(matrix + scalar, actual);
    }

    #[test]
    fn test_scalar_matrix_subtraction() {
        let matrix = Matrix::new([[4, 3, -2], [0, 2, 1], [0, 3, 1]]);
        let scalar = 5;
        let actual = Matrix::new([[-1, -2, -7], [-5, -3, -4], [-5, -2, -4]]);

        assert_eq!(matrix - scalar, actual);
    }

    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix = Matrix::new([[4.0, 3.0, -2.0], [0.0, 2.0, 1.0], [0.0, 3.0, 1.0]]);
        let scalar = 5.0;
        let actual = Matrix::new([[20.0, 15.0, -10.0], [0.0, 10.0, 5.0], [0.0, 15.0, 5.0]]);

        assert_eq!(matrix * scalar, actual);
    }

    #[test]
    fn test_scalar_matrix_division() {
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
    fn test_scalar_matrix_addition_assign() {
        let mut matrix = Matrix::new([[4, 3, -2], [0, 2, 1], [0, 3, 1]]);
        let scalar = 5;
        matrix += scalar;
        let actual = Matrix::new([[9, 8, 3], [5, 7, 6], [5, 8, 6]]);

        assert_eq!(matrix, actual);
    }

    #[test]
    fn test_scalar_matrix_subtraction_assign() {
        let mut matrix = Matrix::new([[4, 3, -2], [0, 2, 1], [0, 3, 1]]);
        let scalar = 5;
        matrix -= scalar;
        let actual = Matrix::new([[-1, -2, -7], [-5, -3, -4], [-5, -2, -4]]);

        assert_eq!(matrix, actual);
    }

    #[test]
    fn test_scalar_matrix_multiplication_assign() {
        let mut matrix = Matrix::new([[4.0, 3.0, -2.0], [0.0, 2.0, 1.0], [0.0, 3.0, 1.0]]);
        let scalar = 5.0;
        matrix *= scalar;
        let actual = Matrix::new([[20.0, 15.0, -10.0], [0.0, 10.0, 5.0], [0.0, 15.0, 5.0]]);

        assert_eq!(matrix, actual);
    }

    #[test]
    fn test_scalar_matrix_division_assign() {
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

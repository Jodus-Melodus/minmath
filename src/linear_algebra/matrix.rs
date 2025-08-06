use std::{
    array,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
};

use crate::{Number, linear_algebra::vector::Vector};

#[derive(Clone, Copy, PartialEq)]
pub struct Matrix<T: Number, const ROWS: usize, const COLUMNS: usize> {
    data: [[T; COLUMNS]; ROWS],
}

impl<T: Number, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS> {
    pub fn new(data: [[T; COLUMNS]; ROWS]) -> Self {
        Self { data }
    }

    pub fn rotation_matrix2x2(theta: f32) -> Matrix<f32, 2, 2> {
        Matrix::new([[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]])
    }

    pub fn rotation_matrix3x3_x(theta: f32) -> Matrix<f32, 3, 3> {
        Matrix::new([
            [1.0, 0.0, 0.0],
            [0.0, theta.cos(), -theta.sin()],
            [0.0, theta.sin(), theta.cos()],
        ])
    }
    pub fn rotation_matrix3x3_y(theta: f32) -> Matrix<f32, 3, 3> {
        Matrix::new([
            [theta.cos(), 0.0, theta.sin()],
            [0.0, 1.0, 0.0],
            [-theta.sin(), 0.0, theta.cos()],
        ])
    }
    pub fn rotation_matrix3x3_z(theta: f32) -> Matrix<f32, 3, 3> {
        Matrix::new([
            [theta.cos(), -theta.sin(), 0.0],
            [theta.sin(), theta.cos(), 0.0],
            [0.0, 0.0, 1.0],
        ])
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

impl<T: Number> Matrix<T, 2, 2> {
    pub fn determinant(&self) -> T {
        self.data[0][0] * self.data[1][1] - self.data[1][0] * self.data[0][1]
    }
}

impl<T: Number, const ROWS: usize, const COLUMNS: usize> Add for Matrix<T, ROWS, COLUMNS> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] + rhs.data[r][c]));

        Self { data }
    }
}

impl<T: Number, const ROWS: usize, const COLUMNS: usize> Add<T> for Matrix<T, ROWS, COLUMNS> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] + rhs));

        Self { data }
    }
}

impl<T: Number, const ROWS: usize, const COLUMNS: usize> AddAssign for Matrix<T, ROWS, COLUMNS> {
    fn add_assign(&mut self, rhs: Self) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] += rhs.data[r][c];
            }
        }
    }
}

impl<T: Number, const ROWS: usize, const COLUMNS: usize> AddAssign<T> for Matrix<T, ROWS, COLUMNS> {
    fn add_assign(&mut self, rhs: T) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] += rhs;
            }
        }
    }
}

impl<T: Number, const ROWS: usize, const COLUMNS: usize> Sub for Matrix<T, ROWS, COLUMNS> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] - rhs.data[r][c]));

        Self { data }
    }
}

impl<T: Number, const ROWS: usize, const COLUMNS: usize> Sub<T> for Matrix<T, ROWS, COLUMNS> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] - rhs));

        Self { data }
    }
}

impl<T: Number, const ROWS: usize, const COLUMNS: usize> SubAssign for Matrix<T, ROWS, COLUMNS> {
    fn sub_assign(&mut self, rhs: Self) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] -= rhs.data[r][c];
            }
        }
    }
}

impl<T: Number, const ROWS: usize, const COLUMNS: usize> SubAssign<T> for Matrix<T, ROWS, COLUMNS> {
    fn sub_assign(&mut self, rhs: T) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] -= rhs;
            }
        }
    }
}

impl<
    T: Number,
    const LROWS: usize,
    const LCOLUMNS: usize,
    const RROWS: usize,
    const RCOLUMNS: usize,
> Mul<Matrix<T, RROWS, RCOLUMNS>> for Matrix<T, LROWS, LCOLUMNS>
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

impl<T: Number, const ROWS: usize, const COLUMNS: usize> Mul<T> for Matrix<T, ROWS, COLUMNS> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] * rhs));

        Self { data }
    }
}
impl<T: Number, const ROWS: usize, const COLUMNS: usize> MulAssign<T> for Matrix<T, ROWS, COLUMNS> {
    fn mul_assign(&mut self, rhs: T) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] *= rhs;
            }
        }
    }
}

impl<T: Number, const ROWS: usize, const COLUMNS: usize> Div<T> for Matrix<T, ROWS, COLUMNS> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] / rhs));

        Self { data }
    }
}
impl<T: Number, const ROWS: usize, const COLUMNS: usize> DivAssign<T> for Matrix<T, ROWS, COLUMNS> {
    fn div_assign(&mut self, rhs: T) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] /= rhs;
            }
        }
    }
}

impl<T: Number, const ROWS: usize, const COLUMNS: usize> Debug for Matrix<T, ROWS, COLUMNS> {
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

impl<T: Number, const ROWS: usize, const COLUMNS: usize> Display for Matrix<T, ROWS, COLUMNS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix ({}x{}):", ROWS, COLUMNS)?;
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                write!(f, "{} ", self.data[r][c])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Number, const ROWS: usize, const COLUMNS: usize> Index<usize> for Matrix<T, ROWS, COLUMNS> {
    type Output = [T; COLUMNS];
    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}

impl<T: Number, const ROWS: usize, const COLUMNS: usize> IndexMut<usize>
    for Matrix<T, ROWS, COLUMNS>
{
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row]
    }
}

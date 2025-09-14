use std::{
    array,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Clone, Copy, PartialEq)]
pub struct Matrix<const ROWS: usize, const COLUMNS: usize> {
    data: [[f32; COLUMNS]; ROWS],
}

impl<const ROWS: usize, const COLUMNS: usize> Matrix<ROWS, COLUMNS> {
    pub fn new(data: [[f32; COLUMNS]; ROWS]) -> Self {
        Self { data }
    }

    pub fn rotation_matrix2x2(theta: f32) -> Matrix<2, 2> {
        Matrix::new([[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]])
    }

    pub fn rotation_matrix3x3_x(theta: f32) -> Matrix<3, 3> {
        Matrix::new([
            [1.0, 0.0, 0.0],
            [0.0, theta.cos(), -theta.sin()],
            [0.0, theta.sin(), theta.cos()],
        ])
    }
    pub fn rotation_matrix3x3_y(theta: f32) -> Matrix<3, 3> {
        Matrix::new([
            [theta.cos(), 0.0, theta.sin()],
            [0.0, 1.0, 0.0],
            [-theta.sin(), 0.0, theta.cos()],
        ])
    }
    pub fn rotation_matrix3x3_z(theta: f32) -> Matrix<3, 3> {
        Matrix::new([
            [theta.cos(), -theta.sin(), 0.0],
            [theta.sin(), theta.cos(), 0.0],
            [0.0, 0.0, 1.0],
        ])
    }

    pub fn size(&self) -> (usize, usize) {
        (ROWS, COLUMNS)
    }

    pub fn transpose(&self) -> Matrix<COLUMNS, ROWS> {
        let mut transposed = [[f32::default(); ROWS]; COLUMNS];

        for r in 0..ROWS {
            for c in 0..COLUMNS {
                transposed[c][r] = self.data[r][c];
            }
        }

        Matrix::new(transposed)
    }
}

impl Matrix<2, 2> {
    pub fn determinant(&self) -> f32 {
        self.data[0][0] * self.data[1][1] - self.data[1][0] * self.data[0][1]
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Add for Matrix<ROWS, COLUMNS> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] + rhs.data[r][c]));

        Self { data }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Add<f32> for Matrix<ROWS, COLUMNS> {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] + rhs));

        Self { data }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> AddAssign for Matrix<ROWS, COLUMNS> {
    fn add_assign(&mut self, rhs: Self) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] += rhs.data[r][c];
            }
        }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> AddAssign<f32> for Matrix<ROWS, COLUMNS> {
    fn add_assign(&mut self, rhs: f32) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] += rhs;
            }
        }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Sub for Matrix<ROWS, COLUMNS> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] - rhs.data[r][c]));

        Self { data }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Sub<f32> for Matrix<ROWS, COLUMNS> {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] - rhs));

        Self { data }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> SubAssign for Matrix<ROWS, COLUMNS> {
    fn sub_assign(&mut self, rhs: Self) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] -= rhs.data[r][c];
            }
        }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> SubAssign<f32> for Matrix<ROWS, COLUMNS> {
    fn sub_assign(&mut self, rhs: f32) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] -= rhs;
            }
        }
    }
}

impl<const LROWS: usize, const LCOLUMNS: usize, const RROWS: usize, const RCOLUMNS: usize>
    Mul<Matrix<RROWS, RCOLUMNS>> for Matrix<LROWS, LCOLUMNS>
{
    type Output = Matrix<LROWS, RCOLUMNS>;
    fn mul(self, rhs: Matrix<RROWS, RCOLUMNS>) -> Self::Output {
        assert_eq!(
            LCOLUMNS, RROWS,
            "Matrix dimension mismatch: Expected left matrix with shape (a * b) and right matrix with shape (b * c), \
             but got shapes (a*{}) and ({}*c).",
            LCOLUMNS, RROWS
        );

        let mut data = [[f32::default(); RCOLUMNS]; LROWS];

        for i in 0..LROWS {
            for j in 0..RCOLUMNS {
                let mut sum = f32::default();
                for k in 0..LCOLUMNS {
                    sum = sum + (self.data[i][k] * rhs.data[k][j]);
                }
                data[i][j] = sum;
            }
        }

        Matrix { data }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Mul<f32> for Matrix<ROWS, COLUMNS> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] * rhs));

        Self { data }
    }
}
impl<const ROWS: usize, const COLUMNS: usize> MulAssign<f32> for Matrix<ROWS, COLUMNS> {
    fn mul_assign(&mut self, rhs: f32) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] *= rhs;
            }
        }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Div<f32> for Matrix<ROWS, COLUMNS> {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        let data = array::from_fn(|r| array::from_fn(|c| self.data[r][c] / rhs));

        Self { data }
    }
}
impl<const ROWS: usize, const COLUMNS: usize> DivAssign<f32> for Matrix<ROWS, COLUMNS> {
    fn div_assign(&mut self, rhs: f32) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] /= rhs;
            }
        }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Debug for Matrix<ROWS, COLUMNS> {
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

impl<const ROWS: usize, const COLUMNS: usize> Display for Matrix<ROWS, COLUMNS> {
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

impl<const ROWS: usize, const COLUMNS: usize> Index<usize> for Matrix<ROWS, COLUMNS> {
    type Output = [f32; COLUMNS];
    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}

impl<const ROWS: usize, const COLUMNS: usize> IndexMut<usize> for Matrix<ROWS, COLUMNS> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row]
    }
}

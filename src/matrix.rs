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
            "Can only convert a single column matrix to a vector"
        );

        let mut data = [T::default(); ROWS];

        for i in 0..ROWS {
            data[i] = self.data[i][0];
        }

        Vector::new(data)
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
        assert_eq!(LCOLUMNS, RROWS);

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

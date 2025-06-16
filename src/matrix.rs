use std::{
    array,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

pub struct Matrix<T, const ROWS: usize, const COLUMNS: usize>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy + Debug,
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
        + Debug,
{
    pub fn new(data: [[T; COLUMNS]; ROWS]) -> Self {
        Self { data }
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
        + Debug,
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
        + Debug,
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
        + Debug,
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
        + Debug,
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
        + Debug,
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
        + Debug,
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
        + Debug,
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
        + Debug,
{
    fn sub_assign(&mut self, rhs: T) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.data[r][c] -= rhs;
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
        + Debug,
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
        + Debug,
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

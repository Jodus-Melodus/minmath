use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

pub struct Matrix<TYPE, const ROWS: usize, const COLUMNS: usize>
where
    TYPE: Add<Output = TYPE>
        + Sub<Output = TYPE>
        + Mul<Output = TYPE>
        + Div<Output = TYPE>
        + Copy
        + Debug,
{
    data: [[TYPE; COLUMNS]; ROWS],
}

impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy + Debug,
{
    pub fn new(data: [[T; COLUMNS]; ROWS]) -> Self {
        Self { data }
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Debug for Matrix<T, ROWS, COLUMNS>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy + Debug,
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
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy + Debug,
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

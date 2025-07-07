use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::matrix::Matrix;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vector<T, const SIZE: usize>
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
    data: [T; SIZE],
}

impl<T, const SIZE: usize> Vector<T, SIZE>
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
    pub fn new(data: [T; SIZE]) -> Self {
        Self { data }
    }

    pub fn size(&self) -> usize {
        SIZE
    }

    pub fn to_matrix(&self) -> Matrix<T, SIZE, 1> {
        let mut data = [[T::default(); 1]; SIZE];

        for i in 0..SIZE {
            data[i][0] = self.data[i];
        }

        Matrix::new(data)
    }
}

impl<T, const SIZE: usize> Debug for Vector<T, SIZE>
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
        writeln!(f, "Vector {}: [", SIZE)?;
        write!(
            f,
            "{}",
            self.data
                .map(|v| format!("{}", v))
                .iter()
                .cloned()
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(f, "]")?;
        Ok(())
    }
}

impl<T, const SIZE: usize> Display for Vector<T, SIZE>
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
        writeln!(f, "Vector {}: [", SIZE)?;
        write!(
            f,
            "{}",
            self.data
                .map(|v| format!("{}", v))
                .iter()
                .cloned()
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(f, "]")?;
        Ok(())
    }
}

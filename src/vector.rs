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

    pub fn dot(&self, rhs: Self) -> T {
        let mut result = T::default();
        for i in 0..SIZE {
            result += self.data[i] * rhs.data[i];
        }
        result
    }
}

impl<T> Vector<T, 3>
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
    pub fn cross(&self, rhs: Self) -> Self {
        Vector::new([
            self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1],
            self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2],
            self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0],
        ])
    }
}

impl<T, const SIZE: usize> Add for Vector<T, SIZE>
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
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] += rhs.data[i];
        }
        Vector::new(result)
    }
}

impl<T, const SIZE: usize> Add<T> for Vector<T, SIZE>
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
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] += rhs;
        }
        Vector::new(result)
    }
}

impl<T, const SIZE: usize> AddAssign for Vector<T, SIZE>
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
        for i in 0..SIZE {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<T, const SIZE: usize> AddAssign<T> for Vector<T, SIZE>
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
        for i in 0..SIZE {
            self.data[i] += rhs;
        }
    }
}
impl<T, const SIZE: usize> Sub for Vector<T, SIZE>
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
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] -= rhs.data[i];
        }
        Vector::new(result)
    }
}

impl<T, const SIZE: usize> Sub<T> for Vector<T, SIZE>
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
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] -= rhs;
        }
        Vector::new(result)
    }
}

impl<T, const SIZE: usize> SubAssign for Vector<T, SIZE>
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
        for i in 0..SIZE {
            self.data[i] -= rhs.data[i];
        }
    }
}

impl<T, const SIZE: usize> SubAssign<T> for Vector<T, SIZE>
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
        for i in 0..SIZE {
            self.data[i] -= rhs;
        }
    }
}

impl<T, const SIZE: usize> Mul<T> for Vector<T, SIZE>
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
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] *= rhs;
        }
        Vector::new(result)
    }
}

impl<T, const SIZE: usize> MulAssign<T> for Vector<T, SIZE>
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
        for i in 0..SIZE {
            self.data[i] *= rhs;
        }
    }
}

impl<T, const SIZE: usize> Div<T> for Vector<T, SIZE>
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
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] /= rhs;
        }
        Vector::new(result)
    }
}

impl<T, const SIZE: usize> DivAssign<T> for Vector<T, SIZE>
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
        for i in 0..SIZE {
            self.data[i] /= rhs;
        }
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

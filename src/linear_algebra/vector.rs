use std::{
    fmt::{Debug, Display, Formatter, Result},
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
};

use crate::{Number, linear_algebra::matrix::Matrix};

#[derive(Clone, Copy, PartialEq)]
pub struct Vector<T: Number, const SIZE: usize> {
    pub data: [T; SIZE],
}

impl<T: Number, const SIZE: usize> Vector<T, SIZE> {
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

impl<T: Number> Vector<T, 3> {
    pub fn cross(&self, rhs: Self) -> Self {
        Vector::new([
            self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1],
            self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2],
            self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0],
        ])
    }
}

impl<T: Number, const SIZE: usize> Add for Vector<T, SIZE> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] += rhs.data[i];
        }
        Vector::new(result)
    }
}

impl<T: Number, const SIZE: usize> Add<T> for Vector<T, SIZE> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] += rhs;
        }
        Vector::new(result)
    }
}

impl<T: Number, const SIZE: usize> AddAssign for Vector<T, SIZE> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..SIZE {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<T: Number, const SIZE: usize> AddAssign<T> for Vector<T, SIZE> {
    fn add_assign(&mut self, rhs: T) {
        for i in 0..SIZE {
            self.data[i] += rhs;
        }
    }
}
impl<T: Number, const SIZE: usize> Sub for Vector<T, SIZE> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] -= rhs.data[i];
        }
        Vector::new(result)
    }
}

impl<T: Number, const SIZE: usize> Sub<T> for Vector<T, SIZE> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] -= rhs;
        }
        Vector::new(result)
    }
}

impl<T: Number, const SIZE: usize> SubAssign for Vector<T, SIZE> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..SIZE {
            self.data[i] -= rhs.data[i];
        }
    }
}

impl<T: Number, const SIZE: usize> SubAssign<T> for Vector<T, SIZE> {
    fn sub_assign(&mut self, rhs: T) {
        for i in 0..SIZE {
            self.data[i] -= rhs;
        }
    }
}

impl<T: Number, const SIZE: usize> Mul<T> for Vector<T, SIZE> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] *= rhs;
        }
        Vector::new(result)
    }
}

impl<T: Number, const SIZE: usize> MulAssign<T> for Vector<T, SIZE> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..SIZE {
            self.data[i] *= rhs;
        }
    }
}

impl<T: Number, const SIZE: usize> Div<T> for Vector<T, SIZE> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] /= rhs;
        }
        Vector::new(result)
    }
}

impl<T: Number, const SIZE: usize> DivAssign<T> for Vector<T, SIZE> {
    fn div_assign(&mut self, rhs: T) {
        for i in 0..SIZE {
            self.data[i] /= rhs;
        }
    }
}

impl<T: Number, const SIZE: usize> Debug for Vector<T, SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Vector ({}):", SIZE)?;
        let formatted = self
            .data
            .iter()
            .map(|v| format!("{:?}", v))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{}]", formatted)
    }
}

impl<T: Number, const SIZE: usize> Display for Vector<T, SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Vector ({}):", SIZE)?;
        let formatted = self
            .data
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{}]", formatted)
    }
}

impl<T: Number, const SIZE: usize> Index<usize> for Vector<T, SIZE> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}
impl<T: Number, const SIZE: usize> IndexMut<usize> for Vector<T, SIZE> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
}

use std::{
    fmt::{Debug, Display, Formatter, Result},
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
};

use crate::linear_algebra::matrix::Matrix;

#[derive(Clone, Copy, PartialEq)]
pub struct Vector<const SIZE: usize> {
    pub data: [f32; SIZE],
}

impl<const SIZE: usize> Vector<SIZE> {
    pub fn new(data: [f32; SIZE]) -> Self {
        Self { data }
    }

    pub fn size(&self) -> usize {
        SIZE
    }

    pub fn to_matrix(&self) -> Matrix<SIZE, 1> {
        let mut data = [[f32::default(); 1]; SIZE];

        for i in 0..SIZE {
            data[i][0] = self.data[i];
        }

        Matrix::new(data)
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        let mut result = f32::default();
        for i in 0..SIZE {
            result += self.data[i] * rhs.data[i];
        }
        result
    }
}

impl Vector<3> {
    pub fn cross(&self, rhs: Self) -> Self {
        Vector::new([
            self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1],
            self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2],
            self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0],
        ])
    }
}

impl<const SIZE: usize> Add for Vector<SIZE> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] += rhs.data[i];
        }
        Vector::new(result)
    }
}

impl<const SIZE: usize> Add<f32> for Vector<SIZE> {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] += rhs;
        }
        Vector::new(result)
    }
}

impl<const SIZE: usize> AddAssign for Vector<SIZE> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..SIZE {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<const SIZE: usize> AddAssign<f32> for Vector<SIZE> {
    fn add_assign(&mut self, rhs: f32) {
        for i in 0..SIZE {
            self.data[i] += rhs;
        }
    }
}
impl<const SIZE: usize> Sub for Vector<SIZE> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] -= rhs.data[i];
        }
        Vector::new(result)
    }
}

impl<const SIZE: usize> Sub<f32> for Vector<SIZE> {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] -= rhs;
        }
        Vector::new(result)
    }
}

impl<const SIZE: usize> SubAssign for Vector<SIZE> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..SIZE {
            self.data[i] -= rhs.data[i];
        }
    }
}

impl<const SIZE: usize> SubAssign<f32> for Vector<SIZE> {
    fn sub_assign(&mut self, rhs: f32) {
        for i in 0..SIZE {
            self.data[i] -= rhs;
        }
    }
}

impl<const SIZE: usize> Mul<f32> for Vector<SIZE> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] *= rhs;
        }
        Vector::new(result)
    }
}

impl<const SIZE: usize> MulAssign<f32> for Vector<SIZE> {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..SIZE {
            self.data[i] *= rhs;
        }
    }
}

impl<const SIZE: usize> Div<f32> for Vector<SIZE> {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        let mut result = self.data;
        for i in 0..SIZE {
            result[i] /= rhs;
        }
        Vector::new(result)
    }
}

impl<const SIZE: usize> DivAssign<f32> for Vector<SIZE> {
    fn div_assign(&mut self, rhs: f32) {
        for i in 0..SIZE {
            self.data[i] /= rhs;
        }
    }
}

impl<const SIZE: usize> Debug for Vector<SIZE> {
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

impl<const SIZE: usize> Display for Vector<SIZE> {
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

impl<const SIZE: usize> Index<usize> for Vector<SIZE> {
    type Output = f32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}
impl<const SIZE: usize> IndexMut<usize> for Vector<SIZE> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
}

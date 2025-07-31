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
        writeln!(f, "Vector ({}):", SIZE)?;
        write!(
            f,
            "[{}",
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
        writeln!(f, "Vector ({}):", SIZE)?;
        write!(
            f,
            "[{}",
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

#[cfg(test)]
mod vector_tests {
    use std::f32::consts::FRAC_PI_2;

    use crate::{matrix::Matrix, vector::Vector};

    #[test]
    fn test_initialization() {
        let vec = Vector::new([4, -3, 2]);

        dbg!("{}", vec);
    }

    #[test]
    fn test_size() {
        let vec1 = Vector::new([4]);
        let vec2 = Vector::new([4, -5]);
        let vec3 = Vector::new([9, 2, -1]);

        assert_eq!(vec1.size(), 1);
        assert_eq!(vec2.size(), 2);
        assert_eq!(vec3.size(), 3);
    }

    #[test]
    fn test_conversion() {
        let vector = Vector::new([5, -3, 2]);
        let matrix = Matrix::new([[5], [-3], [2]]);
        let matrix_result = vector.to_matrix();

        assert_eq!(matrix, matrix_result);
    }

    #[test]
    fn test_vector_addition() {
        let vec1 = Vector::new([4, 0, -1]);
        let vec2 = Vector::new([-3, 2, 0]);
        let vec3 = vec1 + vec2;

        assert_eq!(vec3, Vector::new([1, 2, -1]));
    }

    #[test]
    fn test_vector_subtraction() {
        let vec1 = Vector::new([4, 0, -1]);
        let vec2 = Vector::new([-3, 2, 0]);
        let vec3 = vec1 - vec2;

        assert_eq!(vec3, Vector::new([7, -2, -1]));
    }

    #[test]
    fn test_scalar_vector_addition() {
        let vec1 = Vector::new([3, -3, 9]);
        let res = vec1 + 5;

        assert_eq!(res, Vector::new([8, 2, 14]));
    }

    #[test]
    fn test_scalar_vector_addition_assign() {
        let mut vec1 = Vector::new([3, -3, 9]);
        vec1 += 5;

        assert_eq!(vec1, Vector::new([8, 2, 14]));
    }

    #[test]
    fn test_scalar_vector_subtraction() {
        let vec1 = Vector::new([3, -3, 9]);
        let res = vec1 - 5;

        assert_eq!(res, Vector::new([-2, -8, 4]));
    }

    #[test]
    fn test_scalar_vector_subtraction_assign() {
        let mut vec1 = Vector::new([3, -3, 9]);
        vec1 -= 5;

        assert_eq!(vec1, Vector::new([-2, -8, 4]));
    }

    #[test]
    fn test_scalar_vector_multiplication() {
        let vec1 = Vector::new([-3, 9, 2]);
        let res = vec1 * 3;

        assert_eq!(res, Vector::new([-9, 27, 6]));
    }

    #[test]
    fn test_scalar_vector_multiplication_assign() {
        let mut vec1 = Vector::new([3, -3, 9]);
        vec1 *= 3;

        assert_eq!(vec1, Vector::new([9, -9, 27]));
    }

    #[test]
    fn test_scalar_vector_division() {
        let vec1 = Vector::new([-3, 9, 2]);
        let res = vec1 / 3;

        assert_eq!(res, Vector::new([-1, 3, 2 / 3]));
    }

    #[test]
    fn test_scalar_vector_division_assign() {
        let mut vec1 = Vector::new([3, -3, 9]);
        vec1 /= 3;

        assert_eq!(vec1, Vector::new([1, -1, 3]));
    }

    #[test]
    fn test_vector_dot_product() {
        let vec1 = Vector::new([1, 2, 3]);
        let vec2 = Vector::new([4, 5, 6]);

        assert_eq!(vec1.dot(vec2), 32);
    }

    #[test]
    fn test_vector_cross_product() {
        let vec1 = Vector::new([1, 2, 3]);
        let vec2 = Vector::new([4, 5, 6]);

        assert_eq!(vec1.cross(vec2), Vector::new([-3, 6, -3]));
    }

    #[test]
    fn test_vector_rotation_x() {
        let v = Vector::new([1.0, 2.0, 3.0]);
        let m = v.to_matrix();
        let rotated = Matrix::<f32, 3, 3>::rotation_matrix3x3_x(FRAC_PI_2) * m;
        let result = rotated.to_vector();

        // Rotating (1,2,3) about X by 90°: (1, -3, 2)
        let expected = Vector::new([1.0, -3.0, 2.0]);
        assert!((result.data[0] - expected.data[0]).abs() < 1e-6);
        assert!((result.data[1] - expected.data[1]).abs() < 1e-6);
        assert!((result.data[2] - expected.data[2]).abs() < 1e-6);
    }

    #[test]
    fn test_vector_rotation_y() {
        let v = Vector::new([1.0, 2.0, 3.0]);
        let m = v.to_matrix();
        let rotated = Matrix::<f32, 3, 3>::rotation_matrix3x3_y(FRAC_PI_2) * m;
        let result = rotated.to_vector();

        // Rotating (1,2,3) about Y by 90°: (3, 2, -1)
        let expected = Vector::new([3.0, 2.0, -1.0]);
        assert!((result.data[0] - expected.data[0]).abs() < 1e-6);
        assert!((result.data[1] - expected.data[1]).abs() < 1e-6);
        assert!((result.data[2] - expected.data[2]).abs() < 1e-6);
    }

    #[test]
    fn test_vector_rotation_z() {
        let v = Vector::new([1.0, 2.0, 3.0]);
        let m = v.to_matrix();
        let rotated = Matrix::<f32, 3, 3>::rotation_matrix3x3_z(FRAC_PI_2) * m;
        let result = rotated.to_vector();

        // Rotating (1,2,3) about Z by 90°: (-2, 1, 3)
        let expected = Vector::new([-2.0, 1.0, 3.0]);
        assert!((result.data[0] - expected.data[0]).abs() < 1e-6);
        assert!((result.data[1] - expected.data[1]).abs() < 1e-6);
        assert!((result.data[2] - expected.data[2]).abs() < 1e-6);
    }
}

pub mod linear_algebra;
pub mod set_theory;
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

pub trait Number:
    Copy
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Default
    + Debug
    + Display
{
}

impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for u128 {}

impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}

impl Number for f32 {}
impl Number for f64 {}

impl Number for usize {}
impl Number for isize {}

pub trait OrderedNumber: Number + Ord {}
impl OrderedNumber for u8 {}
impl OrderedNumber for u16 {}
impl OrderedNumber for u32 {}
impl OrderedNumber for u64 {}
impl OrderedNumber for usize {}

impl OrderedNumber for i8 {}
impl OrderedNumber for i16 {}
impl OrderedNumber for i32 {}
impl OrderedNumber for i64 {}
impl OrderedNumber for isize {}

pub trait Float: Number {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn sqrt(self) -> Self;
}
impl Float for f32 {
    #[inline]
    fn sin(self) -> Self {
        self.sin()
    }
    #[inline]
    fn cos(self) -> Self {
        self.cos()
    }
    #[inline]
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl Float for f64 {
    #[inline]
    fn sin(self) -> Self {
        self.sin()
    }
    #[inline]
    fn cos(self) -> Self {
        self.cos()
    }
    #[inline]
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

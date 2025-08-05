pub mod linear_algebra;
pub mod probability;
pub mod set_theory;
use std::ops::{Add, Div, Mul, Sub};

pub trait Number:
    Copy
    + PartialOrd
    + Ord
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
}

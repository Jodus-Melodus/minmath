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

impl<T> Number for T where
    T: Copy
        + PartialOrd
        + Ord
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
{
}

pub struct Set<T: Number> {
    elements: Vec<T>,
}

impl<T: Number> Set<T> {
    pub fn new() -> Self {
        Set {
            elements: Vec::new(),
        }
    }

    pub fn n(&self) -> usize {
        self.elements.len()
    }

    pub fn add_element(&mut self, element: T) {
        if !self.elements.contains(&element) {
            self.elements.push(element);
            self.elements.sort();
        }
    }
}
impl<T: Number> From<Vec<T>> for Set<T> {
    fn from(value: Vec<T>) -> Self {
        let mut elements = value;
        elements.sort();
        elements.dedup();

        Set { elements }
    }
}

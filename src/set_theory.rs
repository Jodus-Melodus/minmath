use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Set<T: Number> {
    pub name: &'static str,
    elements: Vec<T>,
}

impl<T: Number> Set<T> {
    pub fn new() -> Self {
        Set {
            name: "S",
            elements: Vec::new(),
        }
    }

    pub fn size(&self) -> usize {
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

        Set {
            name: "S",
            elements,
        }
    }
}

impl<T: Number + ToString> Debug for Set<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} = {{{}}}",
            self.name,
            self.elements
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl<T: Number + ToString> Display for Set<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} = {{{}}}",
            self.name,
            self.elements
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

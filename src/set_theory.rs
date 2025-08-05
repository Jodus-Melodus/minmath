use crate::Number;
use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Set<T: Number> {
    pub name: &'static str,
    elements: Vec<T>,
}

impl<T: Number> Set<T> {
    /// Creates a new empty set with type `T`
    pub fn new() -> Self {
        Set {
            name: "S",
            elements: Vec::new(),
        }
    }

    /// Returns the size of the set (known as its cardinality)
    pub fn cardinality(&self) -> usize {
        self.elements.len()
    }

    /// Adds an element of type `T` to the set
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

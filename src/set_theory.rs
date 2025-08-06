use crate::OrderedNumber;
use std::fmt::{Debug, Display};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Set<T: OrderedNumber> {
    elements: Vec<T>,
}

impl<T: OrderedNumber> Set<T> {
    /// Creates a new empty set with type `T`
    pub fn new() -> Self {
        Set {
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

    /// Perform `or` between two sets
    pub fn or(&self, other: &Set<T>) -> Set<T> {
        let mut elements = self.elements.clone();
        elements.extend(other.elements.iter().cloned());
        elements.sort();
        elements.dedup();
        Set { elements }
    }

    /// Perform `and` between two sets
    pub fn and(&self, other: &Set<T>) -> Set<T> {
        let elements = self
            .elements
            .iter()
            .filter(|element| other.elements.contains(&element))
            .cloned()
            .collect();
        Set { elements }
    }
}

impl<T: OrderedNumber> From<Vec<T>> for Set<T> {
    fn from(value: Vec<T>) -> Self {
        let mut elements = value;
        elements.sort();
        elements.dedup();

        Set { elements }
    }
}

impl<T: OrderedNumber + ToString> Debug for Set<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{{{}}}",
            self.elements
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl<T: OrderedNumber + ToString> Display for Set<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{{{}}}",
            self.elements
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

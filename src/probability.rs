use std::collections::HashMap;

use crate::{Number, set_theory::Set};

pub type Probability = f64;

pub struct Venn<T: Number> {
    sets: HashMap<&'static str, Set<T>>,
}

impl<T: Number> Venn<T> {
    /// Creates a new Venn diagram of type `T`
    pub fn new() -> Self {
        Venn {
            sets: HashMap::new(),
        }
    }

    /// Add a `Set<T>` to the `Venn<T>`
    pub fn add_set(&mut self, set_name: &'static str, set: Set<T>) {
        if !self.sets.contains_key(set_name) {
            self.sets.insert(set_name, set);
        }
    }
}

impl<T: Number> From<HashMap<&'static str, Set<T>>> for Venn<T> {
    fn from(value: HashMap<&'static str, Set<T>>) -> Self {
        Venn { sets: value }
    }
}

pub fn complement(p: Probability) -> Probability {
    1.0 - p
}

pub fn a_or_b(a: Probability, b: Probability, a_and_b: Probability) -> Probability {
    a + b - a_and_b
}

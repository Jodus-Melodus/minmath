pub type Probability = f64;

pub fn complement(p: Probability) -> Probability {
    1.0 - p
}

pub fn a_or_b(a: Probability, b: Probability, a_and_b: Probability) -> Probability {
    a + b - a_and_b
}

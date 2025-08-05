use minmath::probability::{a_or_b, complement};

#[test]
fn test_complement() {
    assert_eq!(complement(0.0), 1.0);
    assert_eq!(complement(1.0), 0.0);
    assert_eq!(complement(0.5), 0.5);
    assert!((complement(0.25) - 0.75).abs() < 1e-12);
}

#[test]
fn test_a_or_b() {
    // Basic addition minus overlap
    assert_eq!(a_or_b(0.0, 0.0, 0.0), 0.0);
    assert_eq!(a_or_b(0.5, 0.4, 0.1), 0.8);
    assert_eq!(a_or_b(1.0, 0.0, 0.0), 1.0);
    assert_eq!(a_or_b(1.0, 1.0, 1.0), 1.0);
    // Check with floating point tolerance
    let result = a_or_b(0.3, 0.6, 0.2);
    assert!((result - 0.7).abs() < 1e-12);
}

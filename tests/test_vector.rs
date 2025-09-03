use minmath::linear_algebra::matrix::Matrix;
#[cfg(test)]
use minmath::linear_algebra::vector::Vector;

#[test]
fn test_new_and_size() {
    let v = Vector::new([1.0, 2.0, 3.0]);
    assert_eq!(v.size(), 3);
    assert_eq!(v.data, [1.0, 2.0, 3.0]);
}

#[test]
fn test_add_vector() {
    let a = Vector::new([1.0, 2.0, 3.0]);
    let b = Vector::new([4.0, 5.0, 6.0]);
    let c = a + b;
    assert_eq!(c, Vector::new([5.0, 7.0, 9.0]));
}

#[test]
fn test_add_scalar() {
    let a = Vector::new([1.0, 2.0, 3.0]);
    let c = a + 10.0;
    assert_eq!(c, Vector::new([11.0, 12.0, 13.0]));
}

#[test]
fn test_add_assign_vector() {
    let mut a = Vector::new([1.0, 2.0, 3.0]);
    let b = Vector::new([4.0, 5.0, 6.0]);
    a += b;
    assert_eq!(a, Vector::new([5.0, 7.0, 9.0]));
}

#[test]
fn test_add_assign_scalar() {
    let mut a = Vector::new([1.0, 2.0, 3.0]);
    a += 10.0;
    assert_eq!(a, Vector::new([11.0, 12.0, 13.0]));
}

#[test]
fn test_sub_vector() {
    let a = Vector::new([10.0, 20.0, 30.0]);
    let b = Vector::new([1.0, 2.0, 3.0]);
    let c = a - b;
    assert_eq!(c, Vector::new([9.0, 18.0, 27.0]));
}

#[test]
fn test_sub_scalar() {
    let a = Vector::new([10.0, 20.0, 30.0]);
    let c = a - 5.0;
    assert_eq!(c, Vector::new([5.0, 15.0, 25.0]));
}

#[test]
fn test_mul_scalar() {
    let a = Vector::new([1.0, 2.0, 3.0]);
    let c = a * 3.0;
    assert_eq!(c, Vector::new([3.0, 6.0, 9.0]));
}

#[test]
fn test_div_scalar() {
    let a = Vector::new([10.0, 20.0, 30.0]);
    let c = a / 10.0;
    assert_eq!(c, Vector::new([1.0, 2.0, 3.0]));
}

#[test]
fn test_dot_product() {
    let a = Vector::new([1.0, 2.0, 3.0]);
    let b = Vector::new([4.0, 5.0, 6.0]);
    let dot = a.dot(b);
    assert_eq!(dot, 1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.0);
}

#[test]
fn test_cross_product() {
    let a = Vector::new([1.0, 0.0, 0.0]);
    let b = Vector::new([0.0, 1.0, 0.0]);
    let c = a.cross(b);
    assert_eq!(c, Vector::new([0.0, 0.0, 1.0]));
}

#[test]
fn test_debug_display() {
    let v = Vector::new([1.0, 2.0, 3.0]);
    let debug_str = format!("{:?}", v);
    let display_str = format!("{}", v);
    assert!(debug_str.contains("Vector (3):"));
    assert!(debug_str.contains("1"));
    assert!(display_str.contains("Vector (3):"));
    assert!(display_str.contains("1"));
}

#[test]
fn test_sub_assign_vector() {
    let mut a = Vector::new([10.0, 20.0, 30.0]);
    a -= Vector::new([1.0, 2.0, 3.0]);
    assert_eq!(a, Vector::new([9.0, 18.0, 27.0]));
}

#[test]
fn test_sub_assign_scalar() {
    let mut a = Vector::new([10.0, 20.0, 30.0]);
    a -= 5.0;
    assert_eq!(a, Vector::new([5.0, 15.0, 25.0]));
}

#[test]
fn test_mul_assign_scalar() {
    let mut a = Vector::new([1.0, 2.0, 3.0]);
    a *= 3.0;
    assert_eq!(a, Vector::new([3.0, 6.0, 9.0]));
}

#[test]
fn test_div_assign_scalar() {
    let mut a = Vector::new([10.0, 20.0, 30.0]);
    a /= 10.0;
    assert_eq!(a, Vector::new([1.0, 2.0, 3.0]));
}

#[test]
fn test_to_matrix() {
    let v = Vector::new([1.0, 2.0, 3.0]);
    let m = v.to_matrix();

    // Expected matrix: SIZE x 1 matrix
    let expected = Matrix::new([[1.0], [2.0], [3.0]]);

    assert_eq!(m, expected);
}
#[test]
fn test_indexing() {
    let v = Vector::new([10.0, 20.0, 30.0]);
    assert_eq!(v[0], 10.0);
    assert_eq!(v[1], 20.0);
    assert_eq!(v[2], 30.0);
}

#[test]
fn test_index_mut() {
    let mut v = Vector::new([1.0, 2.0, 3.0]);
    v[0] = 10.0;
    v[1] = 20.0;
    v[2] = 30.0;
    assert_eq!(v, Vector::new([10.0, 20.0, 30.0]));
}

#[test]
fn test_display_and_debug() {
    let v = Vector::new([1.0, 2.0, 3.0]);
    let s = format!("{}", v);
    let d = format!("{:?}", v);
    assert!(s.contains("Vector"));
    assert!(d.contains("Vector"));
}

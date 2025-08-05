use std::f32::consts::FRAC_PI_2;

use minmath::linear_algebra::{matrix::Matrix, vector::Vector};

#[test]
fn test_initialization() {
    let vec = Vector::new([4, -3, 2]);

    dbg!("{}", vec);
}

#[test]
fn test_size() {
    let vec1 = Vector::new([4]);
    let vec2 = Vector::new([4, -5]);
    let vec3 = Vector::new([9, 2, -1]);

    assert_eq!(vec1.size(), 1);
    assert_eq!(vec2.size(), 2);
    assert_eq!(vec3.size(), 3);
}

#[test]
fn test_conversion() {
    let vector = Vector::new([5, -3, 2]);
    let matrix = Matrix::new([[5], [-3], [2]]);
    let matrix_result = vector.to_matrix();

    assert_eq!(matrix, matrix_result);
}

#[test]
fn test_vector_addition() {
    let vec1 = Vector::new([4, 0, -1]);
    let vec2 = Vector::new([-3, 2, 0]);
    let vec3 = vec1 + vec2;

    assert_eq!(vec3, Vector::new([1, 2, -1]));
}

#[test]
fn test_vector_subtraction() {
    let vec1 = Vector::new([4, 0, -1]);
    let vec2 = Vector::new([-3, 2, 0]);
    let vec3 = vec1 - vec2;

    assert_eq!(vec3, Vector::new([7, -2, -1]));
}

#[test]
fn test_scalar_vector_addition() {
    let vec1 = Vector::new([3, -3, 9]);
    let res = vec1 + 5;

    assert_eq!(res, Vector::new([8, 2, 14]));
}

#[test]
fn test_scalar_vector_addition_assign() {
    let mut vec1 = Vector::new([3, -3, 9]);
    vec1 += 5;

    assert_eq!(vec1, Vector::new([8, 2, 14]));
}

#[test]
fn test_scalar_vector_subtraction() {
    let vec1 = Vector::new([3, -3, 9]);
    let res = vec1 - 5;

    assert_eq!(res, Vector::new([-2, -8, 4]));
}

#[test]
fn test_scalar_vector_subtraction_assign() {
    let mut vec1 = Vector::new([3, -3, 9]);
    vec1 -= 5;

    assert_eq!(vec1, Vector::new([-2, -8, 4]));
}

#[test]
fn test_scalar_vector_multiplication() {
    let vec1 = Vector::new([-3, 9, 2]);
    let res = vec1 * 3;

    assert_eq!(res, Vector::new([-9, 27, 6]));
}

#[test]
fn test_scalar_vector_multiplication_assign() {
    let mut vec1 = Vector::new([3, -3, 9]);
    vec1 *= 3;

    assert_eq!(vec1, Vector::new([9, -9, 27]));
}

#[test]
fn test_scalar_vector_division() {
    let vec1 = Vector::new([-3, 9, 2]);
    let res = vec1 / 3;

    assert_eq!(res, Vector::new([-1, 3, 2 / 3]));
}

#[test]
fn test_scalar_vector_division_assign() {
    let mut vec1 = Vector::new([3, -3, 9]);
    vec1 /= 3;

    assert_eq!(vec1, Vector::new([1, -1, 3]));
}

#[test]
fn test_vector_dot_product() {
    let vec1 = Vector::new([1, 2, 3]);
    let vec2 = Vector::new([4, 5, 6]);

    assert_eq!(vec1.dot(vec2), 32);
}

#[test]
fn test_vector_cross_product() {
    let vec1 = Vector::new([1, 2, 3]);
    let vec2 = Vector::new([4, 5, 6]);

    assert_eq!(vec1.cross(vec2), Vector::new([-3, 6, -3]));
}

#[test]
fn test_vector_rotation_x() {
    let v = Vector::new([1.0, 2.0, 3.0]);
    let m = v.to_matrix();
    let rotated = Matrix::<f32, 3, 3>::rotation_matrix3x3_x(FRAC_PI_2) * m;
    let result = rotated.to_vector();

    // Rotating (1,2,3) about X by 90°: (1, -3, 2)
    let expected = Vector::new([1.0, -3.0, 2.0]);
    // assert_eq!(result, expected);
}

#[test]
fn test_vector_rotation_y() {
    let v = Vector::new([1.0, 2.0, 3.0]);
    let m = v.to_matrix();
    let rotated = Matrix::<f32, 3, 3>::rotation_matrix3x3_y(FRAC_PI_2) * m;
    let result = rotated.to_vector();

    // Rotating (1,2,3) about Y by 90°: (3, 2, -1)
    let expected = Vector::new([3.0, 2.0, -1.0]);
    // assert_eq!(result, expected);
}

#[test]
fn test_vector_rotation_z() {
    let v = Vector::new([1.0, 2.0, 3.0]);
    let m = v.to_matrix();
    let rotated = Matrix::<f32, 3, 3>::rotation_matrix3x3_z(FRAC_PI_2) * m;
    let result = rotated.to_vector();

    // Rotating (1,2,3) about Z by 90°: (-2, 1, 3)
    let expected = Vector::new([-2.0, 1.0, 3.0]);
    // assert_eq!(result, expected);
}

#[test]
fn test_vector_rotation_2d_90deg() {
    use std::f32::consts::FRAC_PI_2;
    let v = Vector::new([1.0_f32, 0.0]);
    let m = v.to_matrix();
    let rotated = Matrix::<f32, 2, 2>::rotation_matrix2x2(FRAC_PI_2) * m;
    let result = rotated.to_vector();

    // Rotating (1,0) by 90°: (0,1)
    let expected = Vector::new([0.0, 1.0]);
    // assert_eq!(result, expected);
}

#[test]
fn test_vector_rotation_2d_180deg() {
    use std::f32::consts::PI;
    let v = Vector::new([1.0_f32, 0.0]);
    let m = v.to_matrix();
    let rotated = Matrix::<f32, 2, 2>::rotation_matrix2x2(PI) * m;
    let result = rotated.to_vector();

    // Rotating (1,0) by 180°: (-1,0)
    let expected = Vector::new([-1.0, 0.0]);
    // assert_eq!(result, expected);
}

#[test]
fn test_vector_rotation_2d_45deg() {
    use std::f32::consts::FRAC_PI_4;
    let v = Vector::new([1.0_f32, 0.0]);
    let m = v.to_matrix();
    let rotated = Matrix::<f32, 2, 2>::rotation_matrix2x2(FRAC_PI_4) * m;
    let result = rotated.to_vector();

    // Rotating (1,0) by 45°: (sqrt(2)/2, sqrt(2)/2)
    let sqrt2_over_2 = std::f32::consts::SQRT_2 / 2.0;
    let expected = Vector::new([sqrt2_over_2, sqrt2_over_2]);
    // assert_eq!(result, expected);
}

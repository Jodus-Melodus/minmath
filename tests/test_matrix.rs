use minmath::linear_algebra::matrix::Matrix;

fn matrix2x2_f32(data: [[f32; 2]; 2]) -> Matrix<2, 2> {
    Matrix::new(data)
}

fn matrix3x3_f32(data: [[f32; 3]; 3]) -> Matrix<3, 3> {
    Matrix::new(data)
}

#[test]
fn test_new_and_indexing() {
    let m = matrix2x2_f32([[1.0, 2.0], [3.0, 4.0]]);
    assert_eq!(m[0][0], 1.0);
    assert_eq!(m[0][1], 2.0);
    assert_eq!(m[1][0], 3.0);
    assert_eq!(m[1][1], 4.0);
}

#[test]
fn test_add_matrix() {
    let a = matrix2x2_f32([[1.0, 2.0], [3.0, 4.0]]);
    let b = matrix2x2_f32([[5.0, 6.0], [7.0, 8.0]]);
    let c = a + b;
    assert_eq!(c, matrix2x2_f32([[6.0, 8.0], [10.0, 12.0]]));
}

#[test]
fn test_add_scalar() {
    let a = matrix2x2_f32([[1.0, 2.0], [3.0, 4.0]]);
    let b = a + 1.0;
    assert_eq!(b, matrix2x2_f32([[2.0, 3.0], [4.0, 5.0]]));
}

#[test]
fn test_sub_matrix() {
    let a = matrix2x2_f32([[5.0, 6.0], [7.0, 8.0]]);
    let b = matrix2x2_f32([[1.0, 2.0], [3.0, 4.0]]);
    let c = a - b;
    assert_eq!(c, matrix2x2_f32([[4.0, 4.0], [4.0, 4.0]]));
}

#[test]
fn test_sub_scalar() {
    let a = matrix2x2_f32([[2.0, 3.0], [4.0, 5.0]]);
    let b = a - 1.0;
    assert_eq!(b, matrix2x2_f32([[1.0, 2.0], [3.0, 4.0]]));
}

#[test]
fn test_mul_scalar() {
    let a = matrix2x2_f32([[1.0, 2.0], [3.0, 4.0]]);
    let b = a * 2.0;
    assert_eq!(b, matrix2x2_f32([[2.0, 4.0], [6.0, 8.0]]));
}

#[test]
fn test_div_scalar() {
    let a = matrix2x2_f32([[2.0, 4.0], [6.0, 8.0]]);
    let b = a / 2.0;
    assert_eq!(b, matrix2x2_f32([[1.0, 2.0], [3.0, 4.0]]));
}

#[test]
fn test_matrix_multiplication() {
    let a = matrix2x2_f32([[1.0, 2.0], [3.0, 4.0]]);
    let b = matrix2x2_f32([[2.0, 0.0], [1.0, 2.0]]);
    let c = a * b;
    assert_eq!(c, matrix2x2_f32([[4.0, 4.0], [10.0, 8.0]]));
}

#[test]
fn test_transpose() {
    let a = matrix2x2_f32([[1.0, 2.0], [3.0, 4.0]]);
    let t = a.transpose();
    assert_eq!(t, matrix2x2_f32([[1.0, 3.0], [2.0, 4.0]]));
}

#[test]
fn test_determinant() {
    let a = matrix2x2_f32([[4.0, 6.0], [3.0, 8.0]]);
    assert_eq!(a.determinant(), 14.0);
}

#[test]
fn test_rotation_matrix2x2() {
    let theta = std::f32::consts::FRAC_PI_2; // 90 degrees
    let rot = Matrix::<2, 2>::rotation_matrix2x2(theta);
    let expected = matrix2x2_f32([[0.0, -1.0], [1.0, 0.0]]);
    for r in 0..2 {
        for c in 0..2 {
            assert!((rot[r][c] - expected[r][c]).abs() < 1e-5);
        }
    }
}

#[test]
fn test_rotation_matrix3x3_z() {
    let theta = std::f32::consts::FRAC_PI_2; // 90 degrees
    let rot = Matrix::<3, 3>::rotation_matrix3x3_z(theta);
    let expected = matrix3x3_f32([[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]);
    for r in 0..3 {
        for c in 0..3 {
            assert!((rot[r][c] - expected[r][c]).abs() < 1e-5);
        }
    }
}

#[test]
fn test_size() {
    let m = matrix2x2_f32([[1.0, 2.0], [3.0, 4.0]]);
    assert_eq!(m.size(), (2, 2));
}

#[test]
fn test_display_format() {
    let m = matrix2x2_f32([[1.0, 2.0], [3.0, 4.0]]);
    let formatted = format!("{}", m);
    assert!(formatted.contains("Matrix (2x2):"));
    assert!(formatted.contains("1 2"));
    assert!(formatted.contains("3 4"));
}

#[test]
fn test_debug_format() {
    let m = matrix2x2_f32([[1.0, 2.0], [3.0, 4.0]]);
    let formatted = format!("{:?}", m);
    assert!(formatted.contains("Matrix (2x2):"));
    assert!(formatted.contains("1.0"));
    assert!(formatted.contains("4.0"));
}

#[test]
fn test_rotation_matrix3x3_x() {
    let theta = std::f32::consts::FRAC_PI_2; // 90 degrees
    let rot = Matrix::<3, 3>::rotation_matrix3x3_x(theta);
    let expected = Matrix::new([[1.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]]);

    for r in 0..3 {
        for c in 0..3 {
            assert!((rot[r][c] - expected[r][c]).abs() < 1e-5);
        }
    }
}

#[test]
fn test_rotation_matrix3x3_y() {
    let theta = std::f32::consts::FRAC_PI_2; // 90 degrees
    let rot = Matrix::<3, 3>::rotation_matrix3x3_y(theta);
    let expected = Matrix::new([[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]]);

    for r in 0..3 {
        for c in 0..3 {
            assert!((rot[r][c] - expected[r][c]).abs() < 1e-5);
        }
    }
}
#[test]
fn test_add_assign_scalar() {
    let mut m = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
    m += 1.0;
    assert_eq!(m, Matrix::new([[2.0, 3.0], [4.0, 5.0]]));
}

#[test]
fn test_add_assign_matrix() {
    let mut a = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
    let b = Matrix::new([[0.5, 1.0], [1.5, 2.0]]);
    a += b;
    assert_eq!(a, Matrix::new([[1.5, 3.0], [4.5, 6.0]]));
}

#[test]
fn test_sub_assign_scalar() {
    let mut m = Matrix::new([[5.0, 6.0], [7.0, 8.0]]);
    m -= 1.0;
    assert_eq!(m, Matrix::new([[4.0, 5.0], [6.0, 7.0]]));
}

#[test]
fn test_sub_assign_matrix() {
    let mut a = Matrix::new([[5.0, 6.0], [7.0, 8.0]]);
    let b = Matrix::new([[1.0, 1.0], [2.0, 2.0]]);
    a -= b;
    assert_eq!(a, Matrix::new([[4.0, 5.0], [5.0, 6.0]]));
}

#[test]
fn test_mul_assign_scalar() {
    let mut m = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
    m *= 2.0;
    assert_eq!(m, Matrix::new([[2.0, 4.0], [6.0, 8.0]]));
}

#[test]
fn test_div_assign_scalar() {
    let mut m = Matrix::new([[2.0, 4.0], [6.0, 8.0]]);
    m /= 2.0;
    assert_eq!(m, Matrix::new([[1.0, 2.0], [3.0, 4.0]]));
}
#[test]
fn test_index_mut() {
    let mut m = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
    m[0][1] = 10.0;
    m[1][0] = -5.0;

    assert_eq!(m[0][1], 10.0);
    assert_eq!(m[1][0], -5.0);
    assert_eq!(m, Matrix::new([[1.0, 10.0], [-5.0, 4.0]]));
}

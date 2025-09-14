#[cfg(test)]
mod tests {
    use minmath::linear_algebra::{matrix::Matrix, vector::Vector2};

    #[test]
    fn test_vector2_new_and_fields() {
        let v = Vector2::new(1.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn test_vector2_add_vector() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        let c = a + b;
        assert_eq!(c, Vector2::new(4.0, 6.0));
    }

    #[test]
    fn test_vector2_add_scalar() {
        let a = Vector2::new(1.0, 2.0);
        let c = a + 5.0;
        assert_eq!(c, Vector2::new(6.0, 7.0));
    }

    #[test]
    fn test_vector2_add_assign_vector() {
        let mut a = Vector2::new(1.0, 2.0);
        a += Vector2::new(3.0, 4.0);
        assert_eq!(a, Vector2::new(4.0, 6.0));
    }

    #[test]
    fn test_vector2_add_assign_scalar() {
        let mut a = Vector2::new(1.0, 2.0);
        a += 5.0;
        assert_eq!(a, Vector2::new(6.0, 7.0));
    }

    #[test]
    fn test_vector2_sub_vector() {
        let a = Vector2::new(5.0, 7.0);
        let b = Vector2::new(2.0, 3.0);
        let c = a - b;
        assert_eq!(c, Vector2::new(3.0, 4.0));
    }

    #[test]
    fn test_vector2_sub_scalar() {
        let a = Vector2::new(5.0, 7.0);
        let c = a - 2.0;
        assert_eq!(c, Vector2::new(3.0, 5.0));
    }

    #[test]
    fn test_vector2_sub_assign_vector() {
        let mut a = Vector2::new(5.0, 7.0);
        a -= Vector2::new(2.0, 3.0);
        assert_eq!(a, Vector2::new(3.0, 4.0));
    }

    #[test]
    fn test_vector2_sub_assign_scalar() {
        let mut a = Vector2::new(5.0, 7.0);
        a -= 2.0;
        assert_eq!(a, Vector2::new(3.0, 5.0));
    }

    #[test]
    fn test_vector2_mul_scalar() {
        let a = Vector2::new(2.0, 3.0);
        let c = a * 4.0;
        assert_eq!(c, Vector2::new(8.0, 12.0));
    }

    #[test]
    fn test_vector2_mul_assign_scalar() {
        let mut a = Vector2::new(2.0, 3.0);
        a *= 4.0;
        assert_eq!(a, Vector2::new(8.0, 12.0));
    }

    #[test]
    fn test_vector2_div_scalar() {
        let a = Vector2::new(8.0, 12.0);
        let c = a / 4.0;
        assert_eq!(c, Vector2::new(2.0, 3.0));
    }

    #[test]
    fn test_vector2_div_assign_scalar() {
        let mut a = Vector2::new(8.0, 12.0);
        a /= 4.0;
        assert_eq!(a, Vector2::new(2.0, 3.0));
    }

    #[test]
    fn test_vector2_dot_product() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        let dot = a.dot(b);
        assert_eq!(dot, 11.0);
    }

    #[test]
    fn test_vector2_to_matrix() {
        let v = Vector2::new(1.0, 2.0);
        let m = v.to_matrix();
        let expected = Matrix::new([[1.0], [2.0]]);
        assert_eq!(m, expected);
    }
}

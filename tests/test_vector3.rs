#[cfg(test)]
mod tests {
    use minmath::linear_algebra::{matrix::Matrix, vector::Vector3};

    #[test]
    fn test_vector3_new_and_fields() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector3_add_vector() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let c = a + b;
        assert_eq!(c, Vector3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vector3_add_scalar() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let c = a + 10.0;
        assert_eq!(c, Vector3::new(11.0, 12.0, 13.0));
    }

    #[test]
    fn test_vector3_add_assign_vector() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        a += Vector3::new(4.0, 5.0, 6.0);
        assert_eq!(a, Vector3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vector3_add_assign_scalar() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        a += 10.0;
        assert_eq!(a, Vector3::new(11.0, 12.0, 13.0));
    }

    #[test]
    fn test_vector3_sub_vector() {
        let a = Vector3::new(10.0, 20.0, 30.0);
        let b = Vector3::new(1.0, 2.0, 3.0);
        let c = a - b;
        assert_eq!(c, Vector3::new(9.0, 18.0, 27.0));
    }

    #[test]
    fn test_vector3_sub_scalar() {
        let a = Vector3::new(10.0, 20.0, 30.0);
        let c = a - 5.0;
        assert_eq!(c, Vector3::new(5.0, 15.0, 25.0));
    }

    #[test]
    fn test_vector3_sub_assign_vector() {
        let mut a = Vector3::new(10.0, 20.0, 30.0);
        a -= Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(a, Vector3::new(9.0, 18.0, 27.0));
    }

    #[test]
    fn test_vector3_sub_assign_scalar() {
        let mut a = Vector3::new(10.0, 20.0, 30.0);
        a -= 5.0;
        assert_eq!(a, Vector3::new(5.0, 15.0, 25.0));
    }

    #[test]
    fn test_vector3_mul_scalar() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let c = a * 3.0;
        assert_eq!(c, Vector3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_vector3_mul_assign_scalar() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        a *= 3.0;
        assert_eq!(a, Vector3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_vector3_div_scalar() {
        let a = Vector3::new(10.0, 20.0, 30.0);
        let c = a / 10.0;
        assert_eq!(c, Vector3::new(1.0, 2.0, 3.0));
    }
    #[test]
    fn test_vector3_div_assign_scalar() {
        let mut a = Vector3::new(10.0, 20.0, 30.0);
        a /= 10.0;
        assert_eq!(a, Vector3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vector3_dot_product() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let dot = a.dot(b);
        assert_eq!(dot, 1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.0); // 32.0
    }

    #[test]
    fn test_vector3_cross_product() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 1.0, 0.0);
        let c = a.cross(b);
        assert_eq!(c, Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_vector3_to_matrix() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let m = v.to_matrix();
        let expected = Matrix::new([[1.0], [2.0], [3.0]]);
        assert_eq!(m, expected);
    }

    #[test]
    fn test_vector3_debug_display() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let debug_str = format!("{:?}", v);
        assert!(debug_str.contains("Vector3"));
        assert!(debug_str.contains("1.0"));
    }
}

pub mod matrix;

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn test_initialization() {
        let matrix = Matrix::new([[4, 3, -2], [0, 2, 1], [0, 3, 1]]);

        println!("{}", matrix);
    }
}

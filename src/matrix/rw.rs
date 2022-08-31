//! Reading and writing operations.

use crate::error::MatrixError;

use super::Matrix;

impl<const R: usize, const C: usize> Matrix<R, C> {
    /// Checks and fetches the entry located at row `i` and column `j`.
    pub fn get(&self, i: usize, j: usize) -> Option<f64> {
        // Map a 2D coordinate to a coordinate on the internal 1D matrix.
        let index = i + C * j;

        if index > self.matrix.len() {
            return Option::None;
        }

        Some(self.matrix[index])
    }

    /// Checks and sets the entry located at row `i` and column `j`.
    pub fn set(&mut self, entry: f64, i: usize, j: usize) -> Result<(), MatrixError> {
        // Map a 2D coordinate to a coordinate on the internal 1D matrix.
        let index = i + C * j;

        if index > self.matrix.len() {
            return Err(MatrixError::UndefinedIndex);
        }

        self.matrix[index] = entry;

        Ok(())
    }

    /// Fills the matrix with the number `k`
    pub fn fill(&mut self, k: f64) {
        self.matrix = vec![k; C * R]
    }
}

#[cfg(test)]
mod matrix_rw {
    use super::*;

    #[test]
    fn test_successful_get() {
        let data = vec![3.14, 2.18, 6.28, 0.0];

        let matrix: Matrix<2, 2> = Matrix::new(data).unwrap();

        let result = matrix.get(0, 0);

        assert!(result.is_some());

        let result = result.unwrap();

        assert_eq!(result, 3.14);

        let other_result = matrix.get(1, 0);

        assert!(other_result.is_some());

        let other_result = other_result.unwrap();

        assert_eq!(other_result, 2.18);
    }

    #[test]
    fn test_unsuccessful_get() {
        let data = vec![3.14, 2.18, 6.28, 0.0];

        let matrix: Matrix<2, 2> = Matrix::new(data).unwrap();

        let result = matrix.get(5, 20);

        assert!(result.is_none());
    }

    #[test]
    fn test_successful_set() {
        let data = vec![3.14, 2.18, 6.28, 0.0];

        let mut matrix: Matrix<2, 2> = Matrix::new(data).unwrap();

        assert!(matrix.set(6.9, 0, 1).is_ok());

        assert_eq!(matrix.get(0, 1).unwrap(), 6.9);
    }

    #[test]
    fn test_unsuccessful_set() {
        let data = vec![3.14, 2.18, 6.28, 0.0];

        let mut matrix: Matrix<2, 2> = Matrix::new(data).unwrap();

        assert!(matrix.set(6.9, 500, 600).is_err());
    }

    #[test]
    fn test_fill() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let mut matrix: Matrix<2, 2> = Matrix::new(data).unwrap();

        assert_eq!(matrix.matrix, [1.0, 2.0, 3.0, 4.0]);

        matrix.fill(0.0);

        assert_eq!(matrix.matrix, [0.0, 0.0, 0.0, 0.0])
    }
}

//! Operations that only work on 2x2 matrices.

use crate::matrix::Matrix;

impl Matrix<2, 2> {
    /// Returns the determinant of the 2x2 matrix
    pub fn det(&self) -> f64 {
        // (     a        *        d    ) - (       b       *         c     )
        (self.matrix[0] * self.matrix[3]) - (self.matrix[1] * self.matrix[2])
    }

    /// Returns the inverse of the 2x2 matrix
    pub fn inv(&self) -> Self {
        let mut matrix: Vec<f64> = (0..4).map(|f| self.matrix[f]).collect();

        // Switch a and d.
        matrix.swap(0, 3);

        // Negate b and c
        matrix[1] *= -1.0;
        matrix[2] *= -1.0;

        // Create a new matrix and multiply it by the inverse of the original matrices determinant
        let mut matrix = Matrix::new(matrix).unwrap();
        matrix.scalar_multiply(1.0 / self.det());

        matrix
    }
}

#[cfg(test)]
mod test_matrix_2x2_ops {
    use super::*;

    #[test]
    fn test_determinant() {
        let data = vec![1.0, 2.0, 3.0, 4.0];

        let matrix: Matrix<2, 2> = Matrix::new(data).unwrap();

        assert_eq!(matrix.det(), -2.0)
    }

    #[test]
    fn test_inverse() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let matrix: Matrix<2, 2> = Matrix::new(data).unwrap();

        let inverse = matrix.inv();

        // Test that they are indeed inverses by multiplying them and making sure that the product is the identity matrix.
        assert_eq!((matrix.multiply(&inverse)).matrix, [1.0, 0.0, 0.0, 1.0]);
    }
}

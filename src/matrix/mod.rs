pub mod mtuple;
pub mod ops;
pub mod ops2x2;
pub mod transform;
pub mod rw;
use crate::error::MatrixError;

#[derive(Debug)]
/// An `R` x `J` matrix struct
pub struct Matrix<const R: usize, const C: usize> {
    matrix: Vec<f64>,
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    /// Creates a new `Matrix`, consuming the given vector.
    pub fn new(data: Vec<f64>) -> Result<Self, MatrixError> {
        // Check if the dimensions match
        if data.len() != C * R {
            return Err(MatrixError::InvalidDimensions);
        }

        Ok(Self { matrix: data })
    }

    /// Constructs a new identity matrix with `n` x `n` dimensions
    pub fn new_identity_matrix(n: usize) -> Matrix<R, C> {
        let matrix: Vec<f64> = (0..(n * n))
            .map(|i| if (i % n) == (i / n) { 1.0 } else { 0.0 })
            .collect();

        Self { matrix }
    }

    /// Returns the dimensions of the matrix in the form of (rows, columns).
    pub fn dimensions(&self) -> (usize, usize) {
        (R, C)
    }

    /// Returns a reference to the inernal 1D vector.
    pub fn data(&self) -> &Vec<f64> {
        &self.matrix
    }


}

#[cfg(test)]
mod matrix_construction_tests {
    use super::*;

    #[test]
    fn successful_2x2matrix_init() {
        let data = vec![0.0, 1.0, 2.0, 3.0];

        let matrix: Result<Matrix<2, 2>, _> = Matrix::new(data);

        assert!(matrix.is_ok());

        let matrix = matrix.unwrap();

        assert_eq!(matrix.dimensions(), (2, 2));
    }

    #[test]
    fn unsuccessful_2x2matrix_init() {
        let data = vec![0.0, 1.0, 2.0, 3.0];

        let matrix: Result<Matrix<2, 3>, _> = Matrix::new(data);

        assert!(matrix.is_err());
    }

    #[test]
    fn test_identity_matrices() {
        let identity2x2: Matrix<2, 2> = Matrix::new_identity_matrix(2);
        assert_eq!(identity2x2.matrix, [1.0, 0.0, 0.0, 1.0]);

        let identity3x3: Matrix<3, 3> = Matrix::new_identity_matrix(3);
        assert_eq!(
            identity3x3.matrix,
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );

        let identity4x4: Matrix<4, 4> = Matrix::new_identity_matrix(4);
        assert_eq!(
            identity4x4.matrix,
            [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,]
        )
    }

    #[test]
    fn test_dimensions() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let matrix: Matrix<2, 2> = Matrix::new(data).unwrap();

        assert_eq!(matrix.dimensions(), (2, 2));
        
    }

    #[test]
    fn test_get_data() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let matrix: Matrix<2, 2> = Matrix::new(data).unwrap();

        assert_eq!(matrix.data(), &vec![1.0, 2.0, 3.0, 4.0])

    }

}
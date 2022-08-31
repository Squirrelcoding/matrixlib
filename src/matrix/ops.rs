//! Operations such as matrix addition, subtraction, multiplication, scalar multiplication, etc.

use crate::{error::MatrixError, matrix::Matrix, vector::VectorN};

use super::mtuple::Direction;

impl<const R: usize, const C: usize> Matrix<R, C> {
    /// Multiplies the matrix by a scalar `a`
    pub fn scalar_multiply(&mut self, a: f64) {
        let matrix_prime: Vec<f64> = self.matrix.iter_mut().map(|f| *f * a).collect();

        self.matrix = matrix_prime
    }

    /// Performs matrix multiplication on this matrix and the matrix given in the `other` parameter.
    pub fn multiply<const K: usize>(&self, other: &Matrix<C, K>) -> Matrix<R, K> {
        // Get all the horizontal tuples in this matrix
        let horizontal_tuples: Vec<VectorN<C>> = (0..R)
            .map(|i| VectorN::new(self.get_tuple(i, Direction::Row).try_into().unwrap()))
            .collect();

        // Get all the vertical tuples in the other matrix
        let vertical_tuples: Vec<VectorN<C>> = (0..C)
            .map(|i| VectorN::new(other.get_tuple(i, Direction::Column).try_into().unwrap()))
            .collect();

        // Loop to calculate and set the dot product of each tuple.
        let dotproducts: Vec<f64> = (0..(R * K))
            .map(|i| {
                let x = i % K;
                let y = i / C;

                horizontal_tuples[y]
                    .dotproduct(&vertical_tuples[x])
                    .unwrap()
            })
            .collect();

        Matrix::new(dotproducts).unwrap()
    }
}

impl<const R: usize, const C: usize> std::ops::Add for Matrix<R, C> {
    type Output = Result<Matrix<R, C>, MatrixError>;

    /// Adds two matrices A and B
    fn add(self, rhs: Matrix<R, C>) -> Self::Output {
        // Get the internal matrices of both sides.
        let this_data = self.matrix;
        let other_data = rhs.matrix;

        // Add each of the entries together and write the result to the corresponding entry in the new vector.
        let new_matrix_data: Vec<f64> = (0..this_data.len())
            .map(|i| this_data[i] + other_data[i])
            .collect();

        // Make sure that the matrix construction goes as planned.
        match Matrix::new(new_matrix_data) {
            Ok(matrix) => Ok(matrix),
            Err(error) => Err(error),
        }
    }
}

impl<const R: usize, const C: usize> std::ops::Sub for Matrix<R, C> {
    type Output = Result<Matrix<R, C>, MatrixError>;

    /// Subtracts two matrices A and B
    fn sub(self, rhs: Matrix<R, C>) -> Self::Output {
        // Get the internal matrices of both sides.
        let this_data = self.matrix;
        let other_data = rhs.matrix;

        // Subtract each of the entries together and write the result to the corresponding entry in the new vector.
        let new_matrix_data: Vec<f64> = (0..this_data.len())
            .map(|i| this_data[i] - other_data[i])
            .collect();

        // Make sure that the matrix construction goes as planned.
        match Matrix::new(new_matrix_data) {
            Ok(matrix) => Ok(matrix),
            Err(error) => Err(error),
        }
    }
}

impl<const R: usize, const C: usize> std::cmp::PartialEq for Matrix<R, C> {
    fn eq(&self, other: &Self) -> bool {
        self.matrix == other.matrix
    }
}

#[cfg(test)]
mod matrixop_tests {
    use super::*;

    #[test]
    fn test_scalar_multiply() {
        let data = vec![1.0, 2.0, 3.0, 4.0];

        let mut matrix: Matrix<2, 2> = Matrix::new(data).unwrap();

        matrix.scalar_multiply(3.0);

        assert_eq!(matrix.data(), &[3.0, 6.0, 9.0, 12.0]);

        matrix.scalar_multiply(std::f64::consts::PI);

        assert_eq!(
            matrix.data(),
            &[
                3.0 * std::f64::consts::PI,
                6.0 * std::f64::consts::PI,
                9.0 * std::f64::consts::PI,
                12.0 * std::f64::consts::PI
            ]
        );
    }

    #[test]
    fn test_matrix_addition() {
        let entries_a = vec![0.0, 1.0, 2.0, 3.0];

        let a: Matrix<2, 2> = Matrix::new(entries_a).unwrap();

        let entries_b = vec![6.9, 4.2, 2.3, 5.7];

        let b: Matrix<2, 2> = Matrix::new(entries_b).unwrap();

        let c = (a + b).unwrap();

        assert_eq!(c.data(), &[6.9, 5.2, 4.3, 8.7]);
    }

    #[test]
    fn test_matrix_subtraction() {
        let entries_a = vec![0.0, 1.0, 2.0, 3.0];

        let a: Matrix<2, 2> = Matrix::new(entries_a).unwrap();

        let entries_b = vec![6.9, 4.2, 2.3, 5.7];

        let b: Matrix<2, 2> = Matrix::new(entries_b).unwrap();

        let c = (a - b).unwrap();

        // floating point error but its not really a big deal. for now.
        assert_eq!(c.data(), &[-6.9, -3.2, -0.2999999999999998, -2.7]);
    }

    #[test]
    fn test_matrix_multiplication_2x2() {
        let data_a = vec![1.0, 2.0, 3.0, 4.0];

        let data_b = vec![5.0, 2.0, 3.0, 4.0];

        let a: Matrix<2, 2> = Matrix::new(data_a).unwrap();
        let b: Matrix<2, 2> = Matrix::new(data_b).unwrap();

        let c = a.multiply(&b);

        assert_eq!(c.data(), &[11.0, 10.0, 27.0, 22.0]);
    }

    #[test]
    fn test_matrix_multiplication_mxn() {
        let data_a = vec![1.0, 2.0, 3.0, 4.0, 7.0, 9.0];

        let data_b = vec![5.0, 2.0, 6.0, 5.0];

        let a: Matrix<3, 2> = Matrix::new(data_a).unwrap();
        let b: Matrix<2, 2> = Matrix::new(data_b).unwrap();

        let c: Matrix<3, 2> = a.multiply(&b);

        assert_eq!(c.data(), &[17.0, 12.0, 39.0, 26.0, 89.0, 59.0]);
    }

    #[test]
    fn test_true_matrix_equivalence() {
        let data_a = vec![1.0, 2.0, 3.0, 4.0];
        let data_b = vec![1.0, 2.0, 3.0, 4.0];

        let a: Matrix<2, 2> = Matrix::new(data_a).unwrap();
        let b: Matrix<2, 2> = Matrix::new(data_b).unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn test_false_matrix_equivalence() {
        let data_a = vec![1.0, 2.0, 3.0, 4.0];
        let data_b = vec![5.0, 6.0, 7.0, 8.0];

        let a: Matrix<2, 2> = Matrix::new(data_a).unwrap();
        let b: Matrix<2, 2> = Matrix::new(data_b).unwrap();

        assert_ne!(a, b);
    }
}

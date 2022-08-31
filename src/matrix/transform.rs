//! Matrix-transformation related functionality.

use crate::{matrix::Matrix, vector::VectorN};

use super::mtuple::Direction;

impl<const N: usize> Matrix<N, N> {
    /// Transforms a vector through this matrix
    pub fn transform(&self, transformed_vec: &VectorN<N>) -> VectorN<N> {
        let newvec: [f64; N] = [0.0; N];
        let mut newvec = VectorN::new(newvec);

        (0..N).for_each(|i| {
            let mut vec: VectorN<N> =
                VectorN::new(self.get_tuple(i, Direction::Column).try_into().unwrap());

            newvec += vec.mul_cpy(transformed_vec.get(i).unwrap());
        });

        newvec
    }
}

#[cfg(test)]
mod test_matrix_transformations {
    use super::*;

    #[test]
    fn test_2dim_identity_transformation() {
        #[rustfmt::skip]
        let data = vec![
            1.0, 0.0,
            0.0, 1.0
        ];

        let matrix: Matrix<2, 2> = Matrix::new(data).unwrap();

        let vec = vec![3.0, 5.0];
        let vec :VectorN<2> = VectorN::new(vec.try_into().unwrap());

        let transformed_vec = matrix.transform(&vec);

        assert_eq!(transformed_vec, vec);
    }

    #[test]
    fn test_3dim_arbitrary_transformation() {
        #[rustfmt::skip]
        let data = vec![
            3.0, 2.0, 5.0,
            4.0, 7.0, 2.0,
            8.0, 2.0, 1.0
        ];

        let matrix: Matrix<3, 3> = Matrix::new(data).unwrap();

        let vec = vec![3.0, -7.0, 8.0];
        let vec :VectorN<3> = VectorN::new(vec.try_into().unwrap());

        let transformed_vec = matrix.transform(&vec);

        assert_eq!(transformed_vec.data(), &[35.0, -21.0, 18.0]);
    }

}

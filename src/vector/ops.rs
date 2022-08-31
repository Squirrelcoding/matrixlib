//! Vector operations such as addition, subtraction, dot products, etc.

use crate::{error::VectorError, vector::VectorN};

impl<const N: usize> VectorN<N> {
    /// Returns the dot product of this vector
    pub fn dotproduct(&self, other: &VectorN<N>) -> Result<f64, VectorError> {
        if self.dimensions() != other.dimensions() {
            return Err(VectorError::InvalidDimensions);
        }

        let dotproduct = (0..N).map(|i| self.data[i] * other.data[i]).sum();

        Ok(dotproduct)
    }

    /// Performs scalar multiplication on the vector.
    pub fn mul(&mut self, n: f64) {
        (0..N).for_each(|i| self.data[i] *= n);
    }

    /// Performs scalar multiplication on a clone of the vector and returning the result.
    pub fn mul_cpy(&mut self, n: f64) -> Self {
        let newvec: Vec<f64> = (0..N).map(|i| self.data[i] * n).collect();

        VectorN::new(newvec.try_into().unwrap())
    }

}

impl<const N: usize> std::ops::Add for VectorN<N> {
    type Output = VectorN<N>;

    fn add(self, rhs: VectorN<N>) -> Self::Output {
        let newvec: Vec<f64> = (0..N).map(|i| self.data[i] + rhs.data[i]).collect();

        VectorN::new(newvec.try_into().unwrap())
    }
}

impl<const N: usize> std::ops::AddAssign for VectorN<N> {
    fn add_assign(&mut self, rhs: Self) {
        let newvec: Vec<f64> = (0..N).map(|i| self.data[i] + rhs.data[i]).collect();

        *self = VectorN::new(newvec.try_into().unwrap());
    }
}

impl<const N: usize> std::cmp::PartialEq for VectorN<N> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

#[cfg(test)]
mod vec_op_tests {
    use super::*;

    #[test]
    fn test_scalar_multiplication() {
        let data = [1.0, 2.0, 3.0, 4.0];
        let mut vector: VectorN<4> = VectorN::new(data);

        vector.mul(5.0);

        assert_eq!(vector.data, [5.0, 10.0, 15.0, 20.0]);
    }

    #[test]
    fn test_vector_addition() {
        let data_a = [1.0, 2.0, 3.0, 4.0];
        let a: VectorN<4> = VectorN::new(data_a);

        let data_b = [5.0, 6.0, 7.0, 8.0];
        let b: VectorN<4> = VectorN::new(data_b);

        assert_eq!((a + b).data, [6.0, 8.0, 10.0, 12.0])
    }

    #[test]
    fn test_vector_addition_with_addassign() {
        let data_a = [1.0, 2.0, 3.0, 4.0];
        let mut a: VectorN<4> = VectorN::new(data_a);

        let data_b = [5.0, 6.0, 7.0, 8.0];
        let b: VectorN<4> = VectorN::new(data_b);

        a += b;

        assert_eq!(a.data, [6.0, 8.0, 10.0, 12.0]);
    }
}

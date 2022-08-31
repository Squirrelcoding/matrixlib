//! Vectors.

mod ops;
mod polar;

#[derive(Debug)]
/// An `N`x 1 vector struct.
pub struct VectorN<const N: usize> {
    data: [f64; N]
}

impl<const N: usize> VectorN<N> {
    /// Creates a new vector from an array of f64 values
    pub fn new(data: [f64; N]) -> Self {
        Self {
            data
        }
    }

    /// Returns the dimensions of the vector
    pub fn dimensions(&self) -> usize {
        N
    }

    /// Gets an entry in the vector
    pub fn get(&self, i: usize) -> Option<f64> {
        if i > self.data.len() {
            return None;
        } else {
            return Some(self.data[i]);
        }

    }

    /// Returns the internal data of the vector as a slice.
    pub fn data(&self) -> &[f64] {
        &self.data
    }

}


#[cfg(test)]
mod vector_tests {
    use super::VectorN;

    #[test]
    pub fn test_dotproduct() {
        let a = VectorN::new([1.0, 2.0, 3.0, 4.0]);

        let b = VectorN::new([6.9, 4.2, 3.5, 6.7]);

        let dotproduct = a.dotproduct(&b);

        assert!(dotproduct.is_ok());

        let dotproduct = dotproduct.unwrap();

        assert_eq!(dotproduct, 52.6);

    }
}
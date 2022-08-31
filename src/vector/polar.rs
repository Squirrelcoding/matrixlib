use crate::vector::VectorN;

impl VectorN<2> {


}

#[cfg(test)]
mod vector_polar_tests {
    use super::*;

    #[test]
    fn test_vector_as_polar_form() {
        let data = vec![-3.0, 5.0];
        let vector: VectorN<2> = VectorN::new(data.try_into().unwrap());


    }
}
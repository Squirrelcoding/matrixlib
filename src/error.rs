//! Error types

use thiserror::Error;

#[derive(Error, Debug)]
/// Error type for matrix-related errors
pub enum MatrixError {
    #[error("Expected an N x N matrix, recieved different dimensions.")]
    InvalidDimensions,
    #[error("An attempt to access a non-existent index was made.")]
    UndefinedIndex,
}

#[derive(Error, Debug)]
/// Error type for vector-related errors
pub enum VectorError {
    #[error("Expected an N-dimensional Vector, recieved different dimensions.")]
    InvalidDimensions,
}

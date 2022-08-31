//! Operations on individual an matrix tuple (mtuple).

use crate::matrix::Matrix;

pub enum Direction {
    Row,
    Column,
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    /// Gets an n-tuple from a matrix using an index `i` and a given direction (e.g row or column)
    pub fn get_tuple(&self, i: usize, direction: Direction) -> Vec<f64> {
        match direction {
            Direction::Row => {
                // Translates a row index into an index on the actual internal vector.
                let i_as_row_index = C * i;

                // This iterator extracts a specific row from the internal vector
                (i_as_row_index..(C + i_as_row_index))
                    .map(|i| self.matrix[i])
                    .collect()
            }
            Direction::Column => {
                // This iterator extracts a column from the internal vector vec by starting at the index incrementing by the amount of
                // columns every iteration and pushing the value at that entry.
                (i..self.matrix.len())
                    .step_by(C)
                    .map(|j| self.matrix[j])
                    .collect()
            }
        }
    }
}

#[cfg(test)]
mod matrix_mtuple_tests {
    use super::*;

    #[test]
    fn get_tuple_row() {
        let data = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0];

        let matrix: Matrix<4, 3> = Matrix::new(data).unwrap();

        let first_row = matrix.get_tuple(0, Direction::Row);
        let second_row = matrix.get_tuple(1, Direction::Row);
        let third_row = matrix.get_tuple(2, Direction::Row);
        let fourth_row = matrix.get_tuple(3, Direction::Row);

        assert_eq!(first_row, [0.0, 1.0, 2.0]);
        assert_eq!(second_row, [3.0, 4.0, 5.0]);
        assert_eq!(third_row, [6.0, 7.0, 8.0]);
        assert_eq!(fourth_row, [9.0, 10.0, 11.0]);
    }

    #[test]
    fn get_tuple_column() {
        let data = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0];

        let matrix: Matrix<4, 3> = Matrix::new(data).unwrap();

        let first_column = matrix.get_tuple(0, Direction::Column);
        let second_column = matrix.get_tuple(1, Direction::Column);
        let third_column = matrix.get_tuple(2, Direction::Column);

        assert_eq!(first_column, [0.0, 3.0, 6.0, 9.0]);
        assert_eq!(second_column, [1.0, 4.0, 7.0, 10.0]);
        assert_eq!(third_column, [2.0, 5.0, 8.0, 11.0]);
    }

    #[test]
    fn get_2x2matrix_row() {
        let data = vec![1.0, 2.0, 3.0, 4.0];

        let matrix: Matrix<2, 2> = Matrix::new(data).unwrap();

        assert_eq!(matrix.get_tuple(1, Direction::Row), [3.0, 4.0]);
    }

    #[test]
    fn get_2x2matrix_column() {
        let data = vec![1.0, 2.0, 3.0, 4.0];

        let matrix: Matrix<2, 2> = Matrix::new(data).unwrap();

        assert_eq!(matrix.get_tuple(1, Direction::Column), [2.0, 4.0]);
    }
}

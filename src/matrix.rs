use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Index;
use std::rc::Rc;
use std::cmp::PartialEq;
use std::ops::Mul;
use std::ops::Div;

/// Representation of 2-D matrix
#[derive(Debug, Clone)]
pub struct Matrix {
    matrix: Rc<Vec<Vec<f64>>>,
    dim: (usize, usize),
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n").unwrap();
        for h in 0..self.dim.0 {
            write!(f, "|").unwrap();
            for w in 0..self.dim.1 {
                write!(f, " {} ", self.matrix[h as usize][w as usize]).unwrap();
            }
            write!(f, "|\n").unwrap();
        }
        write!(f, "\n")
    }
}

impl Matrix {
    /// Create new matrix from Rust's vector of vectors
    ///
    /// <https://en.wikipedia.org/wiki/Matrix_(mathematics)>
    ///
    /// # Examples
    ///
    /// ```
    /// let vec_a = vec![0.0, 1.0, 2.0, 3.0];
    /// let vec_b = vec![1.0, 0.0, 1.0, 0.0];
    /// let vec_c = vec![5.0, 5.0, 5.0, 5.0];
    ///
    /// let matrix = lingebra::Matrix::new(vec![vec_a, vec_b, vec_c]);
    ///
    /// println!("{}", &matrix);
    ///
    /// //| 0  1  2  3 |
    /// //| 1  0  1  0 |
    /// //| 5  5  5  5 |
    /// ```
    pub fn new(matrix: Vec<Vec<f64>>) -> Matrix {
        let h_size = matrix.len();
        let w_size = matrix[0].len();
        for vec in &matrix {
            if vec.len() != w_size {
                panic!("Invalid matrix dimensions! All vectors should have same size, expected {} found {}", w_size, vec.len());
            }
        }
        Matrix {
            matrix: Rc::new(matrix),
            dim: (h_size, w_size),
        }
    }

    /// Create new row vector from Rust's vector
    ///
    /// <https://en.wikipedia.org/wiki/Row_and_column_vectors>
    ///
    /// # Examples
    ///
    /// ```
    /// let vec_a = vec![0.0, 1.0, 2.0, 3.0];
    ///
    /// let row_vector = lingebra::Matrix::row_vector(vec_a);
    ///
    /// println!("{}", &row_vector);
    ///
    /// //| 0  1  2  3 |
    /// ```
    pub fn row_vector(vector: Vec<f64>) -> Matrix {
        let w_size = vector.len();
        Matrix {
            matrix: Rc::new(vec![vector]),
            dim: (1, w_size),
        }
    }

    /// Create new column vector from Rust's vector
    ///
    /// <https://en.wikipedia.org/wiki/Row_and_column_vectors>
    ///
    /// # Examples
    ///
    /// ```
    /// let vec_a = vec![0.0, 1.0, 2.0, 3.0];
    ///
    /// let column_vector = lingebra::Matrix::column_vector(vec_a);
    ///
    /// println!("{}", &column_vector);
    ///
    /// //| 0 |
    /// //| 1 |
    /// //| 2 |
    /// //| 3 |
    /// ```
    pub fn column_vector(vector: Vec<f64>) -> Matrix {
        let h_size = vector.len();
        let mut matrix = Vec::new();
        for i in 0..h_size {
            matrix.push(vec![vector[i]]);
        };
        Matrix {
            matrix: Rc::new(matrix),
            dim: (h_size, 1),
        }
    }

    /// Create zero matrix of given size (height x width)
    ///
    /// <https://en.wikipedia.org/wiki/Zero_matrix>
    ///
    /// # Examples
    ///
    /// ```
    /// let (height, width) = (2, 3);
    /// let zero_matrix = lingebra::Matrix::zeroes(height, width);
    ///
    /// println!("{}", &zero_matrix);
    ///
    /// //| 0 0 0 |
    /// //| 0 0 0 |
    /// ```
    ///
    /// Zero matrix added to matrix of same dimension gives the original matrix:
    /// ```
    ///
    /// let matrix = lingebra::Matrix::new(vec![vec![0.0, 1.0, 2.0, 3.0],
    ///                                         vec![1.0, 0.0, 1.0, 0.0],
    ///                                         vec![5.0, 5.0, 5.0, 5.0]]);
    ///
    /// let zero_matrix = lingebra::Matrix::zeroes(3, 4);
    /// let result = &matrix + &zero_matrix;
    /// assert_eq!(result, matrix)
    /// ```
    pub fn zeroes(height: usize, width: usize) -> Matrix {
        let mut matrix = Vec::new();
        for _ in 0..height {
            matrix.push(vec![0.0; width])
        }
        Matrix {
            matrix: Rc::new(matrix),
            dim: (height, width),
        }
    }

    /// Create matrix of ones given size (height x width)
    ///
    /// <https://en.wikipedia.org/wiki/Matrix_of_ones>
    ///
    /// # Examples
    ///
    /// ```
    /// let (height, width) = (2, 3);
    /// let matrix_of_ones = lingebra::Matrix::ones(height, width);
    ///
    /// println!("{}", &matrix_of_ones);
    ///
    /// //| 1 1 1 |
    /// //| 1 1 1 |
    /// ```
    pub fn ones(height: usize, width: usize) -> Matrix {
        let mut matrix = Vec::new();
        for _ in 0..height {
            matrix.push(vec![1.0; width])
        }
        Matrix {
            matrix: Rc::new(matrix),
            dim: (height, width),
        }
    }

    /// Create identity matrix of given size (square)
    ///
    /// <https://en.wikipedia.org/wiki/Identity_matrix>
    ///
    /// # Examples
    ///
    /// ```
    /// let size = 3;
    /// let identitity_matrix = lingebra::Matrix::identity(size);
    ///
    /// println!("{}", &identitity_matrix);
    ///
    /// //| 1 0 0 |
    /// //| 0 1 0 |
    /// //| 0 0 1 |
    /// ```
    pub fn identity(size: usize) -> Matrix {
        let mut matrix = Vec::new();
        for x in 0..size {
            let mut line = vec![0.0; size];
            line[x] = 1.0;
            matrix.push(line);
        }
        Matrix {
            matrix: Rc::new(matrix),
            dim: (size, size),
        }
    }
}


/// Indexing for matrices
///
/// # Examples
///
/// ```
/// let size = 3;
/// let matrix = lingebra::Matrix::identity(size);
///
/// assert_eq!(matrix[0][0], 1.0);
/// assert_eq!(matrix[1][1], 1.0);
/// assert_eq!(matrix[2][2], 1.0);
/// ```
impl Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, line: usize) -> &Self::Output {
        &self.matrix[line]
    }
}

/// Equality test for matrices
///
/// # Examples
///
/// ```
/// let matrix_a = lingebra::Matrix::identity(2);
/// let matrix_b = lingebra::Matrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
/// let matrix_c = lingebra::Matrix::ones(2, 2);
/// let matrix_d = lingebra::Matrix::zeroes(1, 1);
/// assert_eq!(matrix_a, matrix_b);
/// assert_ne!(matrix_a, matrix_c);
/// assert_ne!(matrix_a, matrix_d);
/// ```
impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.matrix == other.matrix
    }
}

/// Addition for matrices
///
/// # Examples
///
/// ```
/// let matrix = lingebra::Matrix::ones(2, 2);
/// let expected = lingebra::Matrix::new(vec![vec![2.0, 2.0], vec![2.0, 2.0]]);
/// let result = &matrix + &matrix;
/// assert_eq!(result, expected);
/// ```
impl<'a> Add<&'a Matrix> for &'a Matrix {
    type Output = Matrix;

    fn add(self, other: Self) -> Matrix {
        assert_eq!(&self.dim, &other.dim);
        let mut result = vec![vec![0.0; self.dim.1]; self.dim.0];
        for x in 0..self.dim.0 {
            for y in 0..self.dim.1 {
                result[x][y] = &self.matrix[x][y] + &other.matrix[x][y];
            }
        }
        Matrix::new(result)
    }
}

/// Subtraction for matrices
///
/// # Examples
///
/// ```
/// let matrix_a = lingebra::Matrix::ones(2, 2);
/// let matrix_b = lingebra::Matrix::identity(2);
/// let expected = lingebra::Matrix::new(vec![vec![0.0, 1.0], vec![1.0, 0.0]]);
/// let result = &matrix_a - &matrix_b;
/// assert_eq!(result, expected);
/// ```
impl <'a> Sub<&'a Matrix> for &'a Matrix{
    type Output = Matrix;

    fn sub(self, other: Self) -> Matrix {
        assert_eq!(&self.dim, &other.dim);
        let mut result = vec![vec![0.0; self.dim.1]; self.dim.0];
        for x in 0..self.dim.0 {
            for y in 0..self.dim.1 {
                result[x][y] = &self.matrix[x][y] - &other.matrix[x][y];
            }
        }
        Matrix::new(result)
    }
}

/// Multiplication for matrices and scalars
///
/// # Examples
///
/// ```
/// let matrix = lingebra::Matrix::identity(2);
/// let expected = lingebra::Matrix::new(vec![vec![42.0, 0.0], vec![0.0, 42.0]]);
/// let result = &matrix * 42.0;
/// assert_eq!(result, expected);
/// ```
impl <'a> Mul<f64> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Matrix {
        let mut result = vec![vec![0.0; self.dim.1]; self.dim.0];
        for x in 0..self.dim.0 {
            for y in 0..self.dim.1 {
                result[x][y] = &self.matrix[x][y] * rhs;
            }
        }
        Matrix::new(result)
    }
}

/// Division for matrices and scalars
///
/// # Examples
///
/// ```
/// let matrix = lingebra::Matrix::identity(2);
/// let expected = lingebra::Matrix::new(vec![vec![0.2, 0.0], vec![0.0, 0.2]]);
/// let result = &matrix / 5.0;
/// assert_eq!(result, expected);
/// ```
impl <'a> Div<f64> for &'a Matrix {
    type Output = Matrix;

    fn div(self, rhs: f64) -> Matrix {
        let mut result = vec![vec![0.0; self.dim.1]; self.dim.0];
        for x in 0..self.dim.0 {
            for y in 0..self.dim.1 {
                result[x][y] = &self.matrix[x][y] / rhs;
            }
        }
        Matrix::new(result)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
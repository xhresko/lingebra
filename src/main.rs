//use std::ops::Add;
use std::fmt;

#[derive(Debug)]
pub struct Vector {
    vector: Vec<f64>,
}

impl Vector {
    pub fn new(vector: Vec<f64>) -> Vector {
        Vector { vector: vector }
    }
}

#[derive(Debug)]
pub struct Matrix {
    matrix: Vec<Vec<f64>>,
    dim: (u32, u32),
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
    pub fn new(matrix: Vec<Vec<f64>>) -> Matrix {
        let h_size = matrix.len() as u32;
        let w_size = matrix[0].len() as u32;
        for vec in &matrix {
            if vec.len() as u32 != w_size {
                panic!("Invalid matrix dimensions! All vectors should have same size, expected {} found {}", w_size, vec.len());
            }
        }
        Matrix {
            matrix: matrix,
            dim: (h_size, w_size),
        }
    }

    pub fn from_vector(vector: Vec<f64>) -> Matrix {
        let w_size = vector.len() as u32;
        Matrix {
            matrix: vec![vector],
            dim: (1, w_size),
        }
    }

    pub fn null(h_size: u32, w_size: u32) -> Matrix {
        let mut matrix = Vec::new();
        for _ in 0..h_size {
            matrix.push(vec![0.0; w_size as usize])
        }
        Matrix {
            matrix: matrix,
            dim: (h_size, w_size),
        }
    }

    pub fn one(size: u32) -> Matrix {
        let mut matrix = Vec::new();
        for x in 0..size {
            let mut line = vec![0.0; size as usize];
            line[x as usize] = 1.0;
            matrix.push(line);
        }
        Matrix {
            matrix: matrix,
            dim: (size, size),
        }
    }
}

//impl Add for Matrix {
//    type Output = Self;

//    fn add(self, other: Self) -> Self {
//	Self {
//for x in 0..self.dim.0 {

//	    }
//	}
//    }
//}

fn main() {
    let my_sample = vec![1.0, 2.1, 3.0];
    println!("This is sample: {:?}", &my_sample);
    let vec = Vector::new(my_sample);
    println!("This is vector: {:?}", &vec);

    let my_sample = vec![1.0, 2.1, 3.0];
    println!("This is sample: {:?}", &my_sample);
    let mat = Matrix::from_vector(my_sample);
    println!("This is matrix: {}", &mat);

    let mat_a = Matrix::new(vec![vec![0.0, 1.0, 55.0, 66.33], vec![1.0, 0.0, 1.0, 2.0]]);

    println!("This is also matrix: {}", &mat_a);

    let mat_b = Matrix::null(3, 5);
    println!("This is also matrix: {}", &mat_b);

    let mat_c = Matrix::one(3);
    println!("This is also matrix: {}", &mat_c);
}

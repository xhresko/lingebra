//! # LinGebra
//!
//! `lingebra` is a library that provides simple implementation of objects and operations
//! that are used in linear algebra.
mod matrix;

pub use matrix::Matrix;
use std::f64::consts::E;
use std::f64::consts::PI;

///
/// Sigma = standard deviation
/// Mu = mean
/// # Examples
///
/// ```
/// let x = 1.0;
/// let sigma = 1.0;
/// let mu = 1.0;
/// let result = lingebra::gaussian_function(x, sigma, mu);
/// assert_eq!(result, 0.3989422804014327);
/// ```
pub fn gaussian_function(x: f64, sigma: f64, mu: f64) -> f64 {
    (1.0 / (sigma * (2.0 * PI).sqrt())) * E.powf(-0.5 * ((x - mu) / sigma).powf(2.0))
}

/// Calculate size of a vector
///
/// # Examples
///
/// ```
/// let vector = vec![3.0, 4.0];
/// assert_eq!(lingebra::vector_size(&vector), 5.0);
/// ```
pub fn vector_size(vector: &[f64]) -> f64 {
    vector.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
}

/// Calculate inner product of two vectors
///
/// # Examples
///
/// ```
/// let vector_a = vec![3.0, 4.0];
/// let vector_b = vec![2.0, 1.0];
/// assert_eq!(lingebra::vector_dot_product(&vector_a, &vector_b), 10.0);
/// assert_eq!(lingebra::vector_dot_product(&vector_a, &vector_a).sqrt(),
///            lingebra::vector_size(&vector_a));
/// ```
pub fn vector_dot_product(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

pub fn vector_sum(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

pub fn scalar_projection(a: &[f64], b: &[f64]) -> f64 {
    vector_dot_product(a, b) / vector_size(b).powi(2)
}

pub fn vector_projection(a: &[f64], b: &[f64]) -> Vec<f64> {
    let scale = scalar_projection(a, b);
    b.iter().map(|x| x * scale).collect()
}

/// Check if two vectors are orthogonal to each other
///
/// # Examples
///
/// ```
/// let vector_a = vec![2.0, 1.0, 0.0];
/// let vector_b = vec![-1.0, 2.0, 5.0];
/// assert!(lingebra::orthogonal(&vector_a, &vector_b));
/// ```
pub fn orthogonal(a: &[f64], b: &[f64]) -> bool {
    vector_dot_product(a, b) == 0.0
}

/// Check if two vectors are orthogonal to each other
///
/// # Examples
///
/// ```
/// let a = vec![1.0, 0.0, 0.0, 0.0];
/// let b = vec![0.0, 2.0, -1.0, 0.0];
/// let c = vec![0.0, 1.0, 2.0, 0.0];
/// let d = vec![0.0, 0.0, 0.0, 3.0];
/// assert!(lingebra::all_orthogonal(&vec![&a, &b, &c, &d]));
/// ```
pub fn all_orthogonal(vectors: &[&Vec<f64>]) -> bool {
    if vectors.len() <= 1 {
        true
    } else {
        let test_vector = vectors[0];
        vectors.iter().skip(1).all(|x| orthogonal(test_vector, x))
            && all_orthogonal(&vectors[1..].to_vec())
    }
}

pub fn change_base(v: &[f64], base: &[&Vec<f64>]) -> Vec<f64> {
    assert!(
        all_orthogonal(base),
        "The vectors in base are not all orthogonal to each other"
    );
    base.iter().map(|x| scalar_projection(v, x)).collect()
}

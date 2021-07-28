//! # LinGebra
//!
//! `lingebra` is a library that provides simple implementation of objects and operations
//! that are used in linear algebra.
mod matrix;

use std::f64::consts::E;
use std::f64::consts::PI;
pub use matrix::Matrix;

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

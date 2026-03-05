//! Index and IndexMut Traits
//!
//! Implement `[]` indexing for your own types — with any index type, not just integers.

use std::collections::HashMap;
use std::fmt;
use std::ops::{Index, IndexMut};

/// A 2D matrix with row-major storage.
///
/// Supports tuple indexing: `matrix[(row, col)]`
#[derive(Clone, Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    /// Creates a new matrix filled with zeros.
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    /// Creates a matrix from existing data.
    ///
    /// # Panics
    /// Panics if `data.len() != rows * cols`
    pub fn with_data(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(data.len(), rows * cols, "Data size mismatch");
        Matrix { rows, cols, data }
    }

    /// Returns the dimensions as (rows, cols).
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Returns an iterator over all elements in row-major order.
    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.data.iter()
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &f64 {
        assert!(
            row < self.rows && col < self.cols,
            "Matrix index ({}, {}) out of bounds for {}x{} matrix",
            row,
            col,
            self.rows,
            self.cols
        );
        &self.data[row * self.cols + col]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f64 {
        assert!(
            row < self.rows && col < self.cols,
            "Matrix index ({}, {}) out of bounds for {}x{} matrix",
            row,
            col,
            self.rows,
            self.cols
        );
        &mut self.data[row * self.cols + col]
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.cols {
                write!(f, "{:8.2}", self[(r, c)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// A configuration store indexed by string keys.
#[derive(Default, Debug)]
pub struct Config {
    map: HashMap<String, String>,
}

impl Config {
    /// Creates an empty configuration.
    pub fn new() -> Self {
        Config {
            map: HashMap::new(),
        }
    }

    /// Sets a configuration value.
    pub fn set(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_string(), value.to_string());
    }

    /// Gets a value, returning None if not found.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    /// Checks if a key exists.
    pub fn contains(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }
}

impl Index<&str> for Config {
    type Output = String;

    fn index(&self, key: &str) -> &String {
        self.map
            .get(key)
            .unwrap_or_else(|| panic!("Config key not found: {}", key))
    }
}

/// A sparse vector that only stores non-zero values.
#[derive(Default, Debug)]
pub struct SparseVec {
    data: HashMap<usize, f64>,
    len: usize,
}

impl SparseVec {
    /// Creates a new sparse vector of given length.
    pub fn new(len: usize) -> Self {
        SparseVec {
            data: HashMap::new(),
            len,
        }
    }

    /// Sets a value (removes if zero).
    pub fn set(&mut self, index: usize, value: f64) {
        assert!(index < self.len, "Index out of bounds");
        if value == 0.0 {
            self.data.remove(&index);
        } else {
            self.data.insert(index, value);
        }
    }

    /// Returns the number of non-zero elements.
    pub fn nnz(&self) -> usize {
        self.data.len()
    }
}

impl Index<usize> for SparseVec {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        assert!(index < self.len, "Index {} out of bounds", index);
        // Return reference to stored value, or static 0.0
        self.data.get(&index).unwrap_or(&0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_new() {
        let m = Matrix::new(3, 4);
        assert_eq!(m.dimensions(), (3, 4));
        assert_eq!(m[(0, 0)], 0.0);
    }

    #[test]
    fn test_matrix_with_data() {
        let m = Matrix::with_data(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 1)], 2.0);
        assert_eq!(m[(1, 0)], 3.0);
        assert_eq!(m[(1, 1)], 4.0);
    }

    #[test]
    fn test_matrix_index_mut() {
        let mut m = Matrix::new(2, 2);
        m[(0, 1)] = 5.0;
        m[(1, 0)] = 7.0;
        assert_eq!(m[(0, 1)], 5.0);
        assert_eq!(m[(1, 0)], 7.0);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_matrix_out_of_bounds() {
        let m = Matrix::new(2, 2);
        let _ = m[(5, 5)];
    }

    #[test]
    fn test_matrix_iter() {
        let m = Matrix::with_data(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let sum: f64 = m.iter().sum();
        assert_eq!(sum, 10.0);
    }

    #[test]
    fn test_config_set_and_index() {
        let mut cfg = Config::new();
        cfg.set("host", "localhost");
        cfg.set("port", "8080");
        assert_eq!(cfg["host"], "localhost");
        assert_eq!(cfg["port"], "8080");
    }

    #[test]
    fn test_config_get_option() {
        let mut cfg = Config::new();
        cfg.set("key", "value");
        assert_eq!(cfg.get("key"), Some(&"value".to_string()));
        assert_eq!(cfg.get("missing"), None);
    }

    #[test]
    #[should_panic(expected = "not found")]
    fn test_config_missing_key() {
        let cfg = Config::new();
        let _ = cfg["nonexistent"];
    }

    #[test]
    fn test_config_contains() {
        let mut cfg = Config::new();
        cfg.set("exists", "yes");
        assert!(cfg.contains("exists"));
        assert!(!cfg.contains("missing"));
    }

    #[test]
    fn test_sparse_vec_zero_default() {
        let sv = SparseVec::new(100);
        assert_eq!(sv[0], 0.0);
        assert_eq!(sv[50], 0.0);
        assert_eq!(sv[99], 0.0);
    }

    #[test]
    fn test_sparse_vec_set_and_index() {
        let mut sv = SparseVec::new(100);
        sv.set(10, 5.0);
        sv.set(50, 3.0);
        assert_eq!(sv[10], 5.0);
        assert_eq!(sv[50], 3.0);
        assert_eq!(sv[0], 0.0);
        assert_eq!(sv.nnz(), 2);
    }

    #[test]
    fn test_sparse_vec_set_zero_removes() {
        let mut sv = SparseVec::new(10);
        sv.set(5, 10.0);
        assert_eq!(sv.nnz(), 1);
        sv.set(5, 0.0);
        assert_eq!(sv.nnz(), 0);
    }
}

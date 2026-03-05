// Index and IndexMut traits in Rust
use std::ops::{Index, IndexMut};
use std::fmt;

// 2D Matrix with custom indexing
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix { rows, cols, data: vec![0.0; rows * cols] }
    }

    fn with_data(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(data.len(), rows * cols);
        Matrix { rows, cols, data }
    }
}

// Index by (row, col) tuple
impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, (row, col): (usize, usize)) -> &f64 {
        assert!(row < self.rows && col < self.cols, "Matrix index out of bounds");
        &self.data[row * self.cols + col]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f64 {
        assert!(row < self.rows && col < self.cols, "Matrix index out of bounds");
        &mut self.data[row * self.cols + col]
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.cols {
                write!(f, "{:6.1}", self[(r, c)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Custom map with string indexing
use std::collections::HashMap;
struct Config(HashMap<String, String>);

impl Config {
    fn new() -> Self { Config(HashMap::new()) }
    fn set(&mut self, key: &str, val: &str) { self.0.insert(key.to_string(), val.to_string()); }
}

impl Index<&str> for Config {
    type Output = String;
    fn index(&self, key: &str) -> &String {
        self.0.get(key).unwrap_or_else(|| panic!("Key not found: {}", key))
    }
}

fn main() {
    let mut m = Matrix::new(3, 3);
    for r in 0..3 {
        for c in 0..3 {
            m[(r, c)] = (r * 3 + c + 1) as f64;
        }
    }
    println!("Matrix:
{}", m);
    println!("m[1][2] = {}", m[(1, 2)]);

    m[(0, 0)] = 99.0;
    println!("After m[0][0] = 99:
{}", m);

    let mut cfg = Config::new();
    cfg.set("host", "localhost");
    cfg.set("port", "8080");
    println!("Config host: {}", cfg["host"]);
    println!("Config port: {}", cfg["port"]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_index() {
        let m = Matrix::with_data(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(1, 1)], 4.0);
    }

    #[test]
    fn test_matrix_index_mut() {
        let mut m = Matrix::new(2, 2);
        m[(0, 1)] = 5.0;
        assert_eq!(m[(0, 1)], 5.0);
    }

    #[test]
    fn test_config_index() {
        let mut cfg = Config::new();
        cfg.set("key", "value");
        assert_eq!(cfg["key"], "value");
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let m = Matrix::new(2, 2);
        let _ = m[(5, 5)];
    }
}

// 975: Sparse Matrix
// Only store non-zero elements using HashMap<(usize,usize), f64>
// OCaml uses custom Hashtbl.Make; Rust uses std HashMap with tuple keys

use std::collections::HashMap;

pub struct SparseMatrix {
    rows: usize,
    cols: usize,
    data: HashMap<(usize, usize), f64>,
}

impl SparseMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        SparseMatrix {
            rows,
            cols,
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, r: usize, c: usize, v: f64) {
        assert!(r < self.rows && c < self.cols, "index out of bounds");
        if v == 0.0 {
            self.data.remove(&(r, c));
        } else {
            self.data.insert((r, c), v);
        }
    }

    pub fn get(&self, r: usize, c: usize) -> f64 {
        *self.data.get(&(r, c)).unwrap_or(&0.0)
    }

    /// Number of non-zero elements
    pub fn nnz(&self) -> usize {
        self.data.len()
    }

    pub fn rows(&self) -> usize { self.rows }
    pub fn cols(&self) -> usize { self.cols }

    /// Matrix-vector multiply: result[i] = sum_j mat[i,j] * v[j]
    pub fn matvec(&self, v: &[f64]) -> Vec<f64> {
        assert_eq!(v.len(), self.cols, "vector length mismatch");
        let mut result = vec![0.0f64; self.rows];
        for (&(r, c), &val) in &self.data {
            result[r] += val * v[c];
        }
        result
    }

    /// Transpose: returns new SparseMatrix with rows/cols swapped
    pub fn transpose(&self) -> SparseMatrix {
        let mut t = SparseMatrix::new(self.cols, self.rows);
        for (&(r, c), &v) in &self.data {
            t.data.insert((c, r), v);
        }
        t
    }

    /// Element-wise add: returns new matrix
    pub fn add(&self, other: &SparseMatrix) -> SparseMatrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let mut result = SparseMatrix::new(self.rows, self.cols);
        // Copy self
        for (&k, &v) in &self.data {
            result.data.insert(k, v);
        }
        // Add other
        for (&(r, c), &v) in &other.data {
            let entry = result.data.entry((r, c)).or_insert(0.0);
            *entry += v;
            if *entry == 0.0 {
                result.data.remove(&(r, c));
            }
        }
        result
    }

    /// Iterate non-zero entries (sorted for determinism in tests)
    pub fn entries(&self) -> Vec<((usize, usize), f64)> {
        let mut v: Vec<_> = self.data.iter().map(|(&k, &v)| (k, v)).collect();
        v.sort_by_key(|(k, _)| *k);
        v
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn make_matrix() -> SparseMatrix {
        let mut m = SparseMatrix::new(4, 4);
        m.set(0, 0, 1.0);
        m.set(0, 2, 2.0);
        m.set(1, 1, 3.0);
        m.set(2, 0, 4.0);
        m.set(2, 3, 5.0);
        m.set(3, 3, 6.0);
        m
    }

    #[test]
    fn test_get_set() {
        let m = make_matrix();
        assert_eq!(m.nnz(), 6);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(0, 1), 0.0); // zero element
        assert_eq!(m.get(1, 1), 3.0);
    }

    #[test]
    fn test_set_zero_removes() {
        let mut m = make_matrix();
        m.set(1, 1, 0.0);
        assert_eq!(m.nnz(), 5);
        assert_eq!(m.get(1, 1), 0.0);
    }

    #[test]
    fn test_matvec() {
        let mut m = make_matrix();
        m.set(1, 1, 0.0); // remove entry
        let v = vec![1.0, 0.0, 1.0, 0.0];
        let result = m.matvec(&v);
        assert_eq!(result[0], 3.0); // 1*1 + 2*1
        assert_eq!(result[1], 0.0);
        assert_eq!(result[2], 4.0); // 4*1
    }

    #[test]
    fn test_transpose() {
        let m = make_matrix();
        let mt = m.transpose();
        assert_eq!(mt.get(0, 0), 1.0);
        assert_eq!(mt.get(2, 0), 2.0);
        assert_eq!(mt.get(0, 2), 4.0);
        assert_eq!(mt.get(3, 2), 5.0);
        assert_eq!(mt.get(3, 3), 6.0);
        assert_eq!(mt.nnz(), 6);
    }

    #[test]
    fn test_add() {
        let m1 = make_matrix();
        let mut m2 = SparseMatrix::new(4, 4);
        m2.set(0, 0, 1.0);
        m2.set(1, 1, -3.0); // cancels out

        let sum = m1.add(&m2);
        assert_eq!(sum.get(0, 0), 2.0); // 1+1
        assert_eq!(sum.get(1, 1), 0.0); // 3+(-3)=0, removed
        assert_eq!(sum.nnz(), 6); // 6 + 1 - 1 = 6
    }
}

#![allow(dead_code)]

/// Create a 1D vector of `n` copies of `value`.
pub fn make<T: Clone>(n: usize, value: T) -> Vec<T> {
    vec![value; n]
}

/// Create a 2D matrix of rows × cols with every cell set to `value`.
pub fn make_matrix<T: Clone>(rows: usize, cols: usize, value: T) -> Vec<Vec<T>> {
    vec![vec![value; cols]; rows]
}

/// Get a cell from a 2D matrix.
pub fn matrix_get<T>(matrix: &[Vec<T>], row: usize, col: usize) -> Option<&T> {
    matrix.get(row).and_then(|r| r.get(col))
}

/// Create an identity matrix.
pub fn identity_matrix(n: usize) -> Vec<Vec<f64>> {
    let mut m = make_matrix(n, n, 0.0_f64);
    for (i, row) in m.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    m
}

fn main() {
    let zeros = make(5, 0_i32);
    println!("zeros = {:?}", zeros);

    let mut matrix = make_matrix(3, 4, 0.0_f64);
    matrix[1][2] = 42.0;
    println!("matrix after setting [1][2]=42:");
    for row in &matrix {
        println!("  {:?}", row);
    }

    let id = identity_matrix(3);
    println!("3x3 identity:");
    for row in &id {
        println!("  {:?}", row);
    }
}

/* Output:
   zeros = [0, 0, 0, 0, 0]
   matrix after setting [1][2]=42:
     [0.0, 0.0, 0.0, 0.0]
     [0.0, 0.0, 42.0, 0.0]
     [0.0, 0.0, 0.0, 0.0]
   3x3 identity:
     [1.0, 0.0, 0.0]
     [0.0, 1.0, 0.0]
     [0.0, 0.0, 1.0]
*/

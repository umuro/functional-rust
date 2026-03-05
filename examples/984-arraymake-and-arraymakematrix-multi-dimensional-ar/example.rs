/// Creates a 1D vector of `n` elements, each initialized to `val`.
/// Equivalent to OCaml's `Array.make n val`.
pub fn make<T: Clone>(n: usize, val: T) -> Vec<T> {
    vec![val; n]
}

/// Creates a 1D vector using iterator chaining — functional style.
pub fn make_iter<T: Clone>(n: usize, val: T) -> Vec<T> {
    std::iter::repeat_n(val, n).collect()
}

/// Creates a 2D matrix of `rows × cols`, each cell initialized to `val`.
/// Equivalent to OCaml's `Array.make_matrix rows cols val`.
pub fn make_matrix<T: Clone>(rows: usize, cols: usize, val: T) -> Vec<Vec<T>> {
    vec![vec![val; cols]; rows]
}

/// Creates a 2D matrix using explicit iterator mapping — functional style.
pub fn make_matrix_iter<T: Clone>(rows: usize, cols: usize, val: T) -> Vec<Vec<T>> {
    (0..rows)
        .map(|_| (0..cols).map(|_| val.clone()).collect())
        .collect()
}

fn main() {
    // 1D array — idiomatic with vec!
    let zeros = make(5, 0);
    println!("make(5, 0)       = {:?}", zeros);

    // 1D array — iterator style
    let ones = make_iter(4, 1);
    println!("make_iter(4, 1)  = {:?}", ones);

    // 2D matrix — idiomatic
    let mut matrix = make_matrix(3, 4, 0.0_f64);
    matrix[1][2] = 42.0; // mirror of OCaml: matrix.(1).(2) <- 42.0
    println!("\nmake_matrix(3, 4, 0.0) then matrix[1][2] = 42.0:");
    for row in &matrix {
        let formatted: Vec<String> = row.iter().map(|x| format!("{:.0}", x)).collect();
        println!("  [{}]", formatted.join(", "));
    }

    // 2D matrix — functional iterator style
    let matrix2 = make_matrix_iter(2, 3, 7);
    println!("\nmake_matrix_iter(2, 3, 7):");
    for row in &matrix2 {
        println!("  {:?}", row);
    }
}

/* Output:
   make(5, 0)       = [0, 0, 0, 0, 0]
   make_iter(4, 1)  = [1, 1, 1, 1]

   make_matrix(3, 4, 0.0) then matrix[1][2] = 42.0:
     [0, 0, 0, 0]
     [0, 0, 42, 0]
     [0, 0, 0, 0]

   make_matrix_iter(2, 3, 7):
     [7, 7, 7]
     [7, 7, 7]
*/

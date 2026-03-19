#![allow(clippy::all)]
// 1062: N-Queens — Backtracking

// Approach 1: Backtracking with boolean arrays for columns/diagonals
fn solve_n_queens(n: usize) -> Vec<Vec<usize>> {
    let mut solutions = Vec::new();
    let mut cols = vec![false; n];
    let mut diag1 = vec![false; 2 * n - 1]; // row - col + n - 1
    let mut diag2 = vec![false; 2 * n - 1]; // row + col
    let mut board = vec![0usize; n];

    fn place(
        row: usize,
        n: usize,
        board: &mut Vec<usize>,
        cols: &mut Vec<bool>,
        diag1: &mut Vec<bool>,
        diag2: &mut Vec<bool>,
        solutions: &mut Vec<Vec<usize>>,
    ) {
        if row == n {
            solutions.push(board.clone());
            return;
        }
        for col in 0..n {
            let d1 = row + n - 1 - col;
            let d2 = row + col;
            if !cols[col] && !diag1[d1] && !diag2[d2] {
                board[row] = col;
                cols[col] = true;
                diag1[d1] = true;
                diag2[d2] = true;
                place(row + 1, n, board, cols, diag1, diag2, solutions);
                cols[col] = false;
                diag1[d1] = false;
                diag2[d2] = false;
            }
        }
    }

    place(
        0,
        n,
        &mut board,
        &mut cols,
        &mut diag1,
        &mut diag2,
        &mut solutions,
    );
    solutions
}

// Approach 2: Functional style with Vec accumulation
fn solve_n_queens_func(n: usize) -> Vec<Vec<usize>> {
    fn is_safe(queens: &[usize], col: usize) -> bool {
        let row = queens.len();
        queens.iter().enumerate().all(|(i, &c)| {
            c != col
                && (row as i32 - i as i32).unsigned_abs() as usize
                    != (col as i32 - c as i32).unsigned_abs() as usize
        })
    }

    fn solve(queens: &mut Vec<usize>, row: usize, n: usize, results: &mut Vec<Vec<usize>>) {
        if row == n {
            results.push(queens.clone());
            return;
        }
        for col in 0..n {
            if is_safe(queens, col) {
                queens.push(col);
                solve(queens, row + 1, n, results);
                queens.pop();
            }
        }
    }

    let mut results = Vec::new();
    let mut queens = Vec::new();
    solve(&mut queens, 0, n, &mut results);
    results
}

// Approach 3: Bitmask-based (fastest)
fn solve_n_queens_bits(n: usize) -> usize {
    fn count(row: usize, n: usize, cols: u32, diag1: u32, diag2: u32) -> usize {
        if row == n {
            return 1;
        }
        let mut total = 0;
        let available = ((1u32 << n) - 1) & !(cols | diag1 | diag2);
        let mut bits = available;
        while bits > 0 {
            let bit = bits & bits.wrapping_neg(); // lowest set bit
            total += count(
                row + 1,
                n,
                cols | bit,
                (diag1 | bit) << 1,
                (diag2 | bit) >> 1,
            );
            bits &= bits - 1;
        }
        total
    }
    count(0, n, 0, 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_queens_4() {
        assert_eq!(solve_n_queens(4).len(), 2);
    }

    #[test]
    fn test_n_queens_8() {
        assert_eq!(solve_n_queens(8).len(), 92);
    }

    #[test]
    fn test_n_queens_func() {
        assert_eq!(solve_n_queens_func(4).len(), 2);
        assert_eq!(solve_n_queens_func(8).len(), 92);
    }

    #[test]
    fn test_n_queens_bits() {
        assert_eq!(solve_n_queens_bits(4), 2);
        assert_eq!(solve_n_queens_bits(8), 92);
    }
}

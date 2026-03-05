/// Backtracking: Generic Recursive Framework with Pruning.
///
/// Demonstrated with N-Queens and permutation generation.
/// Pattern: try choice → check constraint → recurse → undo (backtrack).

/// Check if placing a queen at (row, col) is safe.
fn is_safe(board: &[usize], row: usize, col: usize) -> bool {
    for r in 0..row {
        let c = board[r];
        if c == col || c.abs_diff(col) == r.abs_diff(row) {
            return false;
        }
    }
    true
}

/// N-Queens: find all solutions. board[row] = column of queen.
fn n_queens(n: usize) -> Vec<Vec<usize>> {
    let mut solutions = Vec::new();
    let mut board = vec![0usize; n];
    n_queens_rec(n, 0, &mut board, &mut solutions);
    solutions
}

fn n_queens_rec(n: usize, row: usize, board: &mut Vec<usize>, solutions: &mut Vec<Vec<usize>>) {
    if row == n {
        solutions.push(board.clone());
        return;
    }
    for col in 0..n {
        if is_safe(board, row, col) {
            board[row] = col;                             // choose
            n_queens_rec(n, row + 1, board, solutions);  // explore
            // board[row] = 0; -- undo (implicit, will be overwritten)
        }
    }
}

/// Generate all permutations of a slice.
fn permutations<T: Clone>(xs: &[T]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; xs.len()];
    permutations_rec(xs, &mut current, &mut used, &mut result);
    result
}

fn permutations_rec<T: Clone>(
    xs: &[T],
    current: &mut Vec<T>,
    used: &mut Vec<bool>,
    result: &mut Vec<Vec<T>>,
) {
    if current.len() == xs.len() {
        result.push(current.clone());
        return;
    }
    for i in 0..xs.len() {
        if !used[i] {
            used[i] = true;
            current.push(xs[i].clone());  // choose
            permutations_rec(xs, current, used, result);
            current.pop();                // undo
            used[i] = false;              // undo
        }
    }
}

/// Subset sum: does any subset of nums sum to target?
fn subset_sum(nums: &[i64], target: i64) -> bool {
    fn bt(nums: &[i64], idx: usize, remaining: i64) -> bool {
        if remaining == 0 { return true; }
        if idx == nums.len() || remaining < 0 { return false; }
        // Include nums[idx] or skip it
        bt(nums, idx + 1, remaining - nums[idx]) || bt(nums, idx + 1, remaining)
    }
    bt(nums, 0, target)
}

fn print_board(board: &[usize]) {
    let n = board.len();
    for &col in board {
        let row: String = (0..n).map(|c| if c == col { 'Q' } else { '.' }).collect();
        println!("{row}");
    }
    println!();
}

fn main() {
    let sols = n_queens(4);
    println!("4-Queens: {} solutions", sols.len());
    print_board(&sols[0]);

    println!("8-Queens: {} solutions", n_queens(8).len()); // 92

    let mut perms = permutations(&[1, 2, 3]);
    perms.sort();
    println!("permutations([1,2,3]): {} (expected 6)", perms.len());
    for p in &perms { println!("  {p:?}"); }

    println!("subset_sum([3,1,4,1,5], 9): {}", subset_sum(&[3, 1, 4, 1, 5], 9));
    println!("subset_sum([3,1,4,1,5], 11): {}", subset_sum(&[3, 1, 4, 1, 5], 11));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4_queens_count() {
        assert_eq!(n_queens(4).len(), 2);
    }

    #[test]
    fn test_8_queens_count() {
        assert_eq!(n_queens(8).len(), 92);
    }

    #[test]
    fn test_queens_valid() {
        for board in n_queens(8) {
            let n = board.len();
            // Every row has exactly one queen
            for row in 0..n {
                for other in 0..n {
                    if row != other {
                        assert_ne!(board[row], board[other], "column clash");
                        assert_ne!(board[row].abs_diff(board[other]),
                                   row.abs_diff(other), "diagonal clash");
                    }
                }
            }
        }
    }

    #[test]
    fn test_permutations_count() {
        assert_eq!(permutations(&[1, 2, 3]).len(), 6);
        assert_eq!(permutations(&[1, 2, 3, 4]).len(), 24);
    }

    #[test]
    fn test_permutations_content() {
        let mut perms = permutations(&[1, 2, 3]);
        perms.sort();
        assert_eq!(perms[0], vec![1, 2, 3]);
        assert_eq!(perms[5], vec![3, 2, 1]);
    }

    #[test]
    fn test_subset_sum() {
        assert!(subset_sum(&[3, 1, 4, 1, 5], 9));    // 3+1+5=9
        assert!(!subset_sum(&[3, 1, 4, 1, 5], 11)); // no subset sums to 11
        assert!(subset_sum(&[3, 1, 4, 1, 5], 5));    // 5 itself
    }
}

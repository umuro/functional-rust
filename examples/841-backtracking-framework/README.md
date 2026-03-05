📖 **[View on hightechmind.io →](https://hightechmind.io/rust/841-backtracking-framework)**

---

# 841: Backtracking — Generic Recursive Framework with Pruning

**Difficulty:** 3  **Level:** Intermediate

Enumerate all valid solutions by building candidates incrementally and abandoning dead-end branches early — the algorithmic engine behind N-queens, Sudoku solvers, and combinatorial optimization.

## The Problem This Solves

Many problems require finding all (or the best) solution in a combinatorial search space: N-queens placement, Sudoku, graph coloring, subset sum, scheduling with constraints. Brute force generates all candidates then filters — O(n! × n) for permutations. Backtracking does better by pruning: if a partial assignment already violates constraints, abandon it immediately without completing it.

This is not just an academic exercise. Sudoku solvers, SAT solvers (DPLL algorithm), constraint satisfaction in AI planning, and automatic theorem provers are all backtracking with increasingly sophisticated pruning strategies. The difference between a 1ms and a 1s Sudoku solve is almost entirely in the quality of the pruning predicate.

The generic framework makes the pattern explicit: try each choice, check the constraint predicate after each extension (not just at leaves), recurse, then undo the choice (backtrack). Implemented in Rust with mutable `Vec` state passed by `&mut` reference — no heap allocation per recursive call.

## The Intuition

Think of it as tree search: each node is a partial solution, branches are choices at the next position. DFS with pruning: at each node, check if the current partial solution is still consistent. If not, don't explore any of its subtrees — that's the prune. If yes, extend to all children. At leaf nodes (full assignment), collect the solution.

The key insight: check constraints at every extension, not just at the end. For N-queens, check whether placing a queen in column `col` at row `row` conflicts with any already-placed queen. This eliminates entire subtrees of size (n-row)! at each prune.

## How It Works in Rust

```rust
// Constraint check: can we place a queen at (row, col)?
fn is_safe(board: &[usize], row: usize, col: usize) -> bool {
    for r in 0..row {
        let c = board[r];
        // Conflicts: same column, or same diagonal
        if c == col || c.abs_diff(col) == r.abs_diff(row) {
            return false;
        }
    }
    true
}

// N-Queens: board[row] = column of queen in that row
fn n_queens(n: usize) -> Vec<Vec<usize>> {
    let mut solutions = Vec::new();
    let mut board = vec![0usize; n];
    n_queens_rec(n, 0, &mut board, &mut solutions);
    solutions
}

fn n_queens_rec(n: usize, row: usize, board: &mut Vec<usize>, solutions: &mut Vec<Vec<usize>>) {
    if row == n {
        solutions.push(board.clone());  // Complete solution: collect
        return;
    }
    for col in 0..n {
        if is_safe(board, row, col) {
            board[row] = col;                              // Choose
            n_queens_rec(n, row + 1, board, solutions);   // Explore
            // Backtrack: board[row] will be overwritten in next iteration
            // (explicit undo only needed if state isn't overwritten)
        }
    }
}

// Permutations: explicit undo via used[] flag
fn permutations_rec<T: Clone>(xs: &[T], current: &mut Vec<T>,
                               used: &mut Vec<bool>, result: &mut Vec<Vec<T>>) {
    if current.len() == xs.len() { result.push(current.clone()); return; }
    for i in 0..xs.len() {
        if !used[i] {
            used[i] = true;          // Choose
            current.push(xs[i].clone());
            permutations_rec(xs, current, used, result);  // Explore
            current.pop();           // Undo choice
            used[i] = false;         // Undo flag — explicit backtrack
        }
    }
}
```

The `&mut Vec` pattern — passing mutable collections by reference rather than returning them — is idiomatic Rust for recursive algorithms that accumulate results. It avoids cloning partial solutions on every recursive call.

## What This Unlocks

- **SAT solvers and constraint programming**: The DPLL algorithm for satisfiability is backtracking with unit propagation as pruning — the core of every modern SAT solver (MiniSAT, Z3).
- **Sudoku and puzzle solving**: Constraint propagation (eliminating candidate values from peers) + backtracking solves any Sudoku; the same pattern handles Nonograms, Kakuro, and logic puzzles.
- **Subset enumeration and knapsack variants**: "Find all subsets with sum exactly k" is backtracking with a sum constraint — used in portfolio optimization, scheduling, and combinatorial auction algorithms.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable accumulator | `ref` list or `Buffer` | `&mut Vec` passed down — explicit |
| Board state | `int list` (functional, no undo needed) | `Vec<usize>` with in-place update |
| Undo step | Functional: no undo (immutable state) | `current.pop(); used[i] = false;` |
| Collect solutions | `results := sol :: !results` | `solutions.push(board.clone())` |
| Constraint check | Pure function on list | Pure function on slice — identical style |

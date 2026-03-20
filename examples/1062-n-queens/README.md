📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1062-n-queens)**

---

# 1062-n-queens — N-Queens

## Problem Statement

The N-queens problem asks how to place N chess queens on an N×N board so that no two queens attack each other — no shared row, column, or diagonal. It is the classic benchmark for backtracking algorithms and constraint satisfaction solvers.

For N=8, there are 92 solutions. For N=14, there are 365,596. The problem scales exponentially with N, making efficient pruning critical. The solution demonstrates how constraint propagation (recording which columns and diagonals are occupied) can dramatically reduce the search space.

## Learning Outcomes

- Implement backtracking with constraint propagation using boolean arrays
- Use column and diagonal constraint vectors to prune the search tree
- Count solutions and collect all valid board configurations
- Understand the O(N!) worst case and why pruning reduces practical run time
- Connect to constraint satisfaction programming (CSP) and SAT solvers

## Rust Application

`src/lib.rs` uses three boolean arrays: `cols` (which columns are occupied), `diag1` (row-col + N-1 diagonal), and `diag2` (row+col diagonal). When placing a queen at `(row, col)`, all three constraints are checked and set — then cleared on backtrack. This is O(1) per placement/backtrack instead of O(N) scan.

The diagonal encoding `row + col` and `row - col + N - 1` is the key optimization: both diagonals can be indexed directly for O(1) lookup rather than scanning the board.

## OCaml Approach

```ocaml
let solve_n_queens n =
  let solutions = ref [] in
  let cols = Array.make n false in
  let diag1 = Array.make (2*n-1) false in
  let diag2 = Array.make (2*n-1) false in
  let board = Array.make n 0 in
  let rec place row =
    if row = n then solutions := Array.copy board :: !solutions
    else
      for col = 0 to n - 1 do
        let d1 = row + n - 1 - col and d2 = row + col in
        if not cols.(col) && not diag1.(d1) && not diag2.(d2) then begin
          board.(row) <- col; cols.(col) <- true; diag1.(d1) <- true; diag2.(d2) <- true;
          place (row + 1);
          cols.(col) <- false; diag1.(d1) <- false; diag2.(d2) <- false
        end
      done
  in
  place 0;
  !solutions
```

Structurally identical. The constraint array indexing is the same in both languages.

## Key Differences

1. **Mutation model**: Rust's `&mut` arrays are passed explicitly; OCaml's mutable arrays are implicitly shared across recursive calls.
2. **Solutions collection**: Rust uses `results: &mut Vec<Vec<usize>>`; OCaml uses a `ref` to a list — both accumulate without returning from each recursive call.
3. **`board.clone()`**: Rust must explicitly clone the board when adding to solutions (`.clone()`); OCaml uses `Array.copy board`.
4. **Bitmask optimization**: A more optimized version uses bitmasks (three integers) instead of three arrays — both languages support this but Rust's `u32` bit operations are slightly more concise.

## Exercises

1. Implement the bitmask optimization using three `u32` values instead of three `Vec<bool>` arrays.
2. Parallelize the solver using `rayon::par_iter` over the first row's column choices.
3. Write a validator `is_valid_placement(queens: &[usize]) -> bool` that checks a given placement without solving.

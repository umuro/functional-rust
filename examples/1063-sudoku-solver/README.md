📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1063-sudoku-solver)**

---

# 1063-sudoku-solver — Sudoku Solver

## Problem Statement

Sudoku is a constraint satisfaction problem: fill a 9×9 grid with digits 1–9 such that each row, column, and 3×3 box contains each digit exactly once. Backtracking with constraint checking is the standard algorithm — try each valid digit at each empty cell, backtrack when no digit works.

Sudoku solvers appear in puzzle generation systems, benchmarks for SAT solvers, and as interview problems testing backtracking proficiency. Peter Norvig's famous Python solver showed that constraint propagation (reducing possibilities without backtracking) handles most real puzzles without any backtracking.

## Learning Outcomes

- Implement Sudoku solving using backtracking with row/column/box constraints
- Use a `is_valid` predicate that checks all three constraint types
- Understand the search tree and why most puzzles solve quickly
- Optimize constraint checking using bitmasks
- Connect to arc consistency in constraint satisfaction problems

## Rust Application

`src/lib.rs` implements `is_valid(board, row, col, num)` that checks the row, column, and 3×3 box. `solve` iterates all 81 cells, finds the first empty cell (value 0), tries digits 1–9, and recurses. If no digit works, it backtracks by restoring 0.

An optimized version uses bitmasks (`row_mask[r]`, `col_mask[c]`, `box_mask[b]`) to track used digits — O(1) valid digit lookup instead of O(9) scanning.

## OCaml Approach

```ocaml
let solve board =
  let is_valid r c num =
    let rec check i =
      if i = 9 then true
      else if board.(r).(i) = num || board.(i).(c) = num then false
      else check (i + 1)
    in
    let br, bc = (r/3)*3, (c/3)*3 in
    check 0 && (* also check 3x3 box *)
    let rec check_box i =
      if i = 9 then true
      else if board.(br + i/3).(bc + i mod 3) = num then false
      else check_box (i + 1)
    in check_box 0
  in
  (* ... backtracking solve *)
```

The logic is identical. OCaml's `board.(r).(c)` vs Rust's `board[r][c]` is the main syntactic difference.

## Key Differences

1. **Mutable 2D array**: Rust uses `&mut [[u8; 9]; 9]`; OCaml uses `int array array` — both support direct index assignment.
2. **Early termination**: Both return `bool` to signal success/failure; Rust's `return true/false` is explicit while OCaml uses the last expression.
3. **Bitmask optimization**: Both can use three arrays of `u16` (bit per digit) for O(1) constraint checks; the bitmask logic is identical.
4. **Constraint propagation**: Norvig's algorithm extends this with naked pairs/triples elimination — available in both languages with similar code complexity.

## Exercises

1. Add bitmask optimization: use `row_mask: [u16; 9]`, `col_mask: [u16; 9]`, `box_mask: [u16; 9]` to replace the O(9) validity checks with O(1) bit operations.
2. Implement a Sudoku generator that creates valid puzzles by solving from scratch and then removing cells while maintaining uniqueness.
3. Count the total number of valid solutions for a given puzzle (most puzzles should have exactly one).

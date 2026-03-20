# Example 1184: Array.make and Array.make_matrix — Multi-dimensional Arrays

**Difficulty:** ⭐⭐
**Category:** Arrays & Mutable State
**OCaml Source:** `Array.make`, `Array.make_matrix`

## Problem Statement

Create and manipulate one-dimensional and two-dimensional mutable arrays. `Array.make n v` allocates a flat array of `n` elements all initialized to `v`; `Array.make_matrix rows cols v` allocates a 2D grid initialized uniformly. This example covers the primary mutable collection primitives in OCaml and their idiomatic Rust equivalents, including how to read and write individual cells and how to iterate over rows and columns with higher-order functions.

## Learning Outcomes

- How OCaml's `Array.make 5 0` maps to Rust's `vec![0; 5]` — both create a flat initialized array but express it through different syntax
- How OCaml's `Array.make_matrix 3 4 0.0` maps to Rust's `vec![vec![0.0f64; 4]; 3]` — a `Vec<Vec<T>>` is the idiomatic 2D structure in safe Rust
- How mutable cell access differs: OCaml uses `matrix.(row).(col) <- value`; Rust uses `matrix[row][col] = value` after binding with `let mut`
- Why Rust requires explicit `mut` to allow any mutation, while OCaml arrays are always mutable by construction
- How `Array.iter` and `Array.iteri` map to `.iter()` and `.iter().enumerate()` in Rust, and how row-by-row iteration over a 2D structure looks in both languages

## OCaml Approach

OCaml provides `Array.make : int -> 'a -> 'a array` to allocate a 1D array filled with a single initial value. For 2D grids, `Array.make_matrix : int -> int -> 'a -> 'a array array` creates an array of arrays — each row is an independent array, so rows do not share storage. Cell mutation uses the dedicated update operator: `matrix.(1).(2) <- 42.0` sets row 1, column 2 to 42.0. All OCaml arrays are mutable by default; no `mutable` annotation is needed. Iteration uses `Array.iter (fun row -> ...) matrix` for rows, nested with another `Array.iter` for cells, mapping naturally to the Rust nested iterator pattern.

## Rust Application

In Rust, a 1D initialized array is `vec![0; 5]` — the macro syntax `[initial_value; count]` mirrors OCaml's argument order of `(count, value)` in reverse. A 2D array is `vec![vec![0.0f64; 4]; 3]`, producing a `Vec<Vec<f64>>` with 3 rows of 4 columns each. The binding must be declared with `let mut` before any cell can be written: `matrix[1][2] = 42.0`. Row-by-row iteration uses `matrix.iter()` returning references to each inner `Vec`; cell iteration nests another `.iter()` inside. For index-aware iteration, `.iter().enumerate()` replaces OCaml's `Array.iteri`. Because each inner `Vec` is independently heap-allocated, the 2D layout is not contiguous in memory — the same tradeoff as OCaml's `array array`.

## Key Differences

1. **Allocation syntax:** OCaml: `Array.make n v` (function call, count then value); Rust: `vec![v; n]` (macro, value then count) — the argument order is reversed, which is a common source of off-by-one confusion when translating.
2. **Mutability declaration:** OCaml arrays are always mutable; Rust variables must be declared `let mut` before the first write, and the compiler rejects any attempted mutation of a non-`mut` binding at compile time.
3. **Cell update syntax:** OCaml uses the `.<- ` operator `matrix.(r).(c) <- v`; Rust uses index notation with assignment `matrix[r][c] = v` after the binding is declared `mut`.
4. **Memory layout:** Both languages represent a 2D array as an array of pointers to independent row arrays (`'a array array` / `Vec<Vec<T>>`), so rows are not contiguous in memory. For performance-critical numeric work, a flat `Vec<T>` with manual index arithmetic (`row * cols + col`) is preferred in both languages.

## Exercises

1. Implement `make_identity(n: usize) -> Vec<Vec<f64>>` that creates an `n×n` identity matrix (1.0 on the diagonal, 0.0 everywhere else) using `vec![0.0; n]` rows and then writing the diagonal cells.
2. Implement `transpose(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>>` that returns a new matrix where rows and columns are swapped. Use nested iteration and verify that `transpose(transpose(m)) == m` for a non-square matrix.
3. Rewrite the 2D matrix using a flat `Vec<f64>` of length `rows * cols` and implement `get(row: usize, col: usize)` and `set(row: usize, col: usize, val: f64)` accessors that compute the linear index internally. Benchmark it against the `Vec<Vec<f64>>` version for a large matrix to observe cache effects.

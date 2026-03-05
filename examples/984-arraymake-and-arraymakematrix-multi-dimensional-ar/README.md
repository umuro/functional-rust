# Example 984: Array.make and Array.make_matrix — Multi-dimensional Arrays

**Difficulty:** ⭐
**Category:** stdlib-array
**OCaml Source:** OCaml Standard Library — `Array.make`, `Array.make_matrix`

## Problem Statement

Create 1D and 2D arrays pre-filled with a default value, then mutate individual cells. This covers the most common array initialisation patterns in OCaml's `Array` module.

## Learning Outcomes

- How `vec![val; n]` in Rust maps directly to `Array.make n val` in OCaml
- Why `Vec<Vec<T>>` is the idiomatic Rust representation of a 2D matrix
- How `std::iter::repeat(val).take(n)` mirrors the infinite-list intuition in functional programming
- How OCaml's row-independence guarantee (`Array.make_matrix` allocates each row separately) is preserved by `vec![vec![val; cols]; rows]` in Rust

## OCaml Approach

OCaml provides `Array.make n v` for 1D arrays and `Array.make_matrix r c v` for 2D arrays. Both return mutable arrays, and `Array.make_matrix` always allocates each row independently so mutation of one row cannot alias another. Indexing uses `arr.(i)` syntax and mutation uses `arr.(i) <- v`.

## Rust Approach

Rust uses `Vec<T>` for heap-allocated, growable arrays. The `vec![val; n]` macro is the idiomatic initialisation form, equivalent in meaning to `Array.make`. A 2D matrix is represented as `Vec<Vec<T>>`; `vec![vec![val; cols]; rows]` ensures each row is an independent allocation, mirroring OCaml's guarantee. An alternative functional approach uses `std::iter::repeat` and iterator maps, making the repetition explicit.

## Key Differences

1. **Mutability:** OCaml arrays are mutable by default; Rust `Vec` requires `mut` binding and uses `v[i] = x` indexing syntax.
2. **2D representation:** OCaml has a native `Array.make_matrix` returning `'a array array`; Rust uses `Vec<Vec<T>>`, a nested heap allocation with the same semantics.
3. **Initialisation macro:** `vec![val; n]` is Rust's equivalent of `Array.make n val`, both filling n slots with copies of val.
4. **Functional alternative:** `std::iter::repeat(val).take(n).collect()` captures the infinite-stream-then-truncate idiom familiar from lazy functional languages.

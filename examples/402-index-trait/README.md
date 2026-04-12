📖 **[View on hightechmind.io →](https://hightechmind.io/rust/402-index-trait)**

---

# 402: Index and IndexMut Traits
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Mathematical and scientific code benefits from natural indexing syntax. A matrix should support `m[(row, col)]`, a graph adjacency structure `graph[(u, v)]`, and a typed map `inventory["sword"]`. Rust's `Index` and `IndexMut` traits operator-overload the `[]` subscript operator for any combination of container and index type — not just integer indexing. This enables domain-specific types to feel as natural as built-in arrays while maintaining Rust's safety guarantees.

The `Index` trait powers `Vec<T>`'s `v[i]`, `HashMap`'s `map["key"]`, `String`'s `s[range]`, and any custom data structure that benefits from subscript notation.

## Learning Outcomes

- Understand how `std::ops::Index` and `IndexMut` operator-overload `[]` indexing
- Learn how the `Output` associated type determines what `container[idx]` returns
- See how tuple index types like `(usize, usize)` enable 2D matrix access
- Understand the difference between `Index` (immutable borrow) and `IndexMut` (mutable borrow)
- Learn how `HashMap` uses `Index` with arbitrary key types

## Rust Application

In `src/lib.rs`, `Matrix` stores elements in a row-major `Vec<f64>` and implements `Index<(usize, usize)>` with `Output = f64`. The `index` method computes `row * cols + col` and returns a reference to the element. `IndexMut` returns `&mut f64`, enabling `matrix[(1, 2)] = 3.14`. A `HashMap`-backed typed map demonstrates `Index` with string keys. All indexing is bounds-checked, panicking rather than producing undefined behavior.

## OCaml Approach

OCaml uses `.(i)` syntax for arrays (`arr.(i) <- val`), which is built into the language for `'a array`. For custom data structures, OCaml defines regular functions (`Matrix.get m i j`) rather than operator overloading. OCaml does support custom operators via `let (.![]) m (r,c) = ...` syntax (infix operator definitions), making matrix access via `m.![r,c]` possible but uncommon.

## Key Differences

1. **Operator syntax**: Rust uses `[]` via traits; OCaml uses `.(i)` for arrays and `.[i]` for bytes, with custom operators rarely used.
2. **Panic vs. exception**: Rust's `Index` panics on out-of-bounds (no `Result`); OCaml's `arr.(i)` raises `Invalid_argument` exception.
3. **Index types**: Rust allows any type as index (tuples, strings, ranges); OCaml's built-in array syntax is integer-only.
4. **Mutable indexing**: Rust has separate `IndexMut` trait for `lhs[i] = val`; OCaml's `arr.(i) <- val` is a distinct syntax form.

## Exercises

1. **Sparse matrix**: Implement `SparseMatrix` using `HashMap<(usize, usize), f64>` with `Index<(usize, usize)>` that returns 0.0 for missing entries and `IndexMut` that inserts when set to non-zero.
2. **Ring buffer**: Create `RingBuffer<T>` with a fixed capacity and implement `Index<usize>` that wraps around (index `n` maps to `n % capacity`). Add a `push` method and verify that old entries are accessible via wrap-around index.
3. **Typed map**: Implement `TypedMap` using `HashMap<TypeId, Box<dyn Any>>` with `Index` and `IndexMut` that accept type witnesses and return typed references, combining the type-map pattern from example 385 with operator overloading.

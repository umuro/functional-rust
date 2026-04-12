📖 **[View on hightechmind.io →](https://hightechmind.io/rust/126-const-generics)**

---

# Const Generics
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Before const generics, Rust arrays `[T; N]` were a special case — their size was part of the type, but you could not write generic code over arbitrary sizes without macros. Libraries had to implement traits separately for arrays of size 0 through 32. Const generics allow `N` to appear as a type-level constant, enabling truly generic fixed-size arrays, matrices, and buffers where dimension mismatches become compile errors rather than runtime panics.

## Learning Outcomes

- Understand how `const N: usize` in a type parameter encodes array size in the type system
- Learn to write generic functions and methods that work for any compile-time size
- See how `FixedArray<f64, 3>` and `FixedArray<f64, 4>` are genuinely incompatible types
- Understand the use of const generics in embedded systems and numerical computing

## Rust Application

`FixedArray<T, const N: usize>` wraps `[T; N]` with safe accessors. The `map` method returns `FixedArray<U, N>` — the same size, a potentially different element type. Matrix addition `add_matrices<const R: usize, const C: usize>` takes two `[[f64; C]; R]` arrays and returns the same shape; passing arrays of different sizes is rejected at compile time. No bounds checks are needed inside `map` because the compiler knows both arrays have exactly `N` elements.

## OCaml Approach

OCaml does not have const generics. Fixed-size arrays are represented as `'a array` with runtime length, or as tuples for small fixed sizes. Libraries like `Owl` use runtime dimension checking with exceptions. The type system cannot express "a matrix of exactly 3 rows and 4 columns" without GADTs and type-level naturals (example 129), which is considerably more verbose.

## Key Differences

1. **Dimension encoding**: Rust encodes array sizes in the type directly with `const N: usize`; OCaml uses runtime values or verbose type-level natural encodings.
2. **Error timing**: Rust dimension mismatches are compile errors; OCaml raises exceptions at runtime (or panics in unsafe code).
3. **No overhead**: `FixedArray<T, N>` has identical memory layout to `[T; N]` — the `N` parameter disappears entirely at runtime.
4. **Ergonomics**: Rust const generics are relatively ergonomic (stable since 1.51); OCaml's GAT-based type-level naturals are research-level complexity.

## Exercises

1. Add a `zip` method to `FixedArray` that takes another `FixedArray<U, N>` and returns `FixedArray<(T, U), N>`.
2. Implement a type-safe dot product `dot<const N: usize>(a: &FixedArray<f64, N>, b: &FixedArray<f64, N>) -> f64`.
3. Write a 2D `Matrix<T, const R: usize, const C: usize>` type with a `transpose` method returning `Matrix<T, C, R>`.

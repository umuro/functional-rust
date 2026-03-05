📖 **[View on hightechmind.io →](https://hightechmind.io/rust/780-const-generic-struct)**

---

# 780: Generic Structs Parameterised by const

**Difficulty:** 3  **Level:** Intermediate

`struct Matrix<const R: usize, const C: usize>` — encode dimensions in the type so mismatched multiplication is a compile error.

## The Problem This Solves

Matrix multiplication has a dimension constraint: you can multiply an `R×N` matrix by an `N×C` matrix, but not an `R×N` by an `M×C` unless `N == M`. Without const generics, you'd check this at runtime and panic or return `Err`. With const generics, mismatched dimensions are a **type error** — the code won't compile.

This is the core promise of const generics: properties that are normally runtime-checked (array lengths, buffer sizes, state machine alphabet sizes) become part of the type. The compiler enforces them. No runtime cost, no runtime failure modes. `Matrix<2, 3>` and `Matrix<3, 2>` are different types; multiplying them in the wrong order is caught during compilation.

Const generic structs appear in embedded Rust (fixed-size ring buffers, fixed-dimension matrices for robotics), crypto (fixed-length hash digests), and any domain where "this collection has exactly N elements" is a semantic invariant worth encoding in the type system.

## The Intuition

`const N: usize` in a struct definition makes `N` part of the type, not an instance variable. `Matrix<2, 3>` and `Matrix<2, 4>` are as different as `u32` and `u64`. Methods can be restricted to specific const parameter values — `identity()` only makes sense when `R == N`, so it's implemented only for `Matrix<N, N>`. Matrix multiplication's type signature `fn mat_mul<R, N, C>(a: &Matrix<R, N>, b: &Matrix<N, C>) -> Matrix<R, C>` expresses the dimension constraint directly.

## How It Works in Rust

```rust
// R rows, C columns — encoded in the type
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<const R: usize, const C: usize> {
    data: [[f64; C]; R],   // stack-allocated fixed-size array
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn zero() -> Self { Self { data: [[0.0; C]; R] } }
    pub fn rows() -> usize { R }   // compile-time constant — no runtime cost
    pub fn cols() -> usize { C }
}

// Only square matrices have an identity
impl<const N: usize> Matrix<N, N> {
    pub fn identity() -> Self {
        let mut m = Self::zero();
        for i in 0..N { m.data[i][i] = 1.0; }
        m
    }
}

// Dimension constraint in the type signature:
// Matrix<R,N> × Matrix<N,C> → Matrix<R,C>
// If N doesn't match, the call won't compile
pub fn mat_mul<const R: usize, const N: usize, const C: usize>(
    a: &Matrix<R, N>,
    b: &Matrix<N, C>,
) -> Matrix<R, C> {
    let mut out = Matrix::<R, C>::zero();
    for r in 0..R {
        for c in 0..C {
            let mut sum = 0.0;
            for k in 0..N { sum += a.data[r][k] * b.data[k][c]; }
            out.data[r][c] = sum;
        }
    }
    out
}

// Usage: dimensions checked at compile time
let a: Matrix<2, 3> = /* ... */;
let b: Matrix<3, 4> = /* ... */;
let c: Matrix<2, 4> = mat_mul(&a, &b);   // ok: inner dims both 3
// let bad = mat_mul(&a, &a);            // compile error: Matrix<2,3> × Matrix<2,3> — 3 ≠ 2
```

`[[f64; C]; R]` is a stack-allocated fixed-size 2D array. For large matrices, you'd use heap storage (`Vec<f64>` with manual indexing), but const generics still encode the logical dimensions in the type.

## What This Unlocks

- **Dimension-safe linear algebra** — matrix types where `mat_mul(a: &Matrix<R,N>, b: &Matrix<N,C>)` won't compile for mismatched inner dimensions; the nalgebra crate uses exactly this pattern.
- **Impl specialization via const constraints** — `impl<const N: usize> Matrix<N, N>` restricts methods to square matrices only; the same technique applies to power-of-two constraints, zero-length guards, etc.
- **Zero-overhead fixed-size structures** — `[[T; C]; R]` is allocated on the stack with no heap overhead, and the size is known at compile time for efficient copy/comparison.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type-level naturals | GADTs with `zero`/`succ` types | `const N: usize` — direct integer in type param |
| Fixed-size arrays | Bigarray with dimensions | `[[f64; C]; R]` — stack-allocated, bounds-checked |
| Method on subset | No direct equivalent | `impl<const N: usize> Matrix<N, N>` restricts to square |
| Dimension mismatch | Runtime error | Compile error — wrong `N` is a type mismatch |

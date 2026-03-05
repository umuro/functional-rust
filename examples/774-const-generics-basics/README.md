📖 **[View on hightechmind.io →](https://hightechmind.io/rust/774-const-generics-basics)**

---

# 774: Const Generics: fn<const N: usize> Fundamentals

**Difficulty:** 3  **Level:** Intermediate

Parameterize types and functions with compile-time integers — `Vector<3>` and `Vector<4>` are different types, and the compiler catches dimension mismatches before your code runs.

## The Problem This Solves

Before const generics (stable since Rust 1.51), fixed-size arrays were awkward in generic code. You couldn't write `fn sum<const N: usize>(arr: &[i32; N]) -> i32` — you had to use slices and lose the compile-time size information, or write separate implementations for each size.

This matters for numerical code, embedded systems, and any domain where array dimensions have semantic meaning. A `Matrix<3, 4>` (3 rows, 4 columns) and a `Matrix<4, 3>` are fundamentally different. Multiplying two matrices requires the inner dimensions to match: `Matrix<A, B> * Matrix<B, C>` is valid; `Matrix<A, B> * Matrix<A, B>` is not (unless `A == B`). With const generics, wrong dimensions are a compile error, not a runtime panic.

The same principle applies to fixed-size buffers in embedded systems, type-safe packet structures in networking, and statically-allocated data structures where heap allocation is forbidden.

## The Intuition

Think of const generics as type parameters that are integers instead of types. `Vec<i32>` has a type parameter; `[i32; 8]` has a *const* parameter. Const generics generalize this: `fn zeros<const N: usize>() -> [i32; N]` is a single function that works for any array size, with the size checked at compile time.

In C++, this is `template<size_t N>`. In Haskell and TypeScript, it's type-level natural numbers with more complexity. Rust makes it straightforward: `const N: usize` in angle brackets, used wherever a `usize` constant is expected.

## How It Works in Rust

```rust
// A function that works on arrays of any compile-time-known size
fn sum<const N: usize>(arr: &[i32; N]) -> i32 {
    arr.iter().sum()
}

// Dot product — both arrays must have the same N, enforced by the type
fn dot<const N: usize>(a: &[f64; N], b: &[f64; N]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

// A type parameterized by size — Vector<3> and Vector<4> are different types
#[derive(Debug, Clone, PartialEq)]
pub struct Vector<const N: usize> {
    data: [f64; N],
}

impl<const N: usize> Vector<N> {
    pub fn new(data: [f64; N]) -> Self { Self { data } }
    pub fn zeros() -> Self { Self { data: [0.0; N] } }

    pub fn dot(&self, other: &Self) -> f64 {
        // `other: &Self` means other must also be Vector<N> — same size!
        self.data.iter().zip(other.data.iter()).map(|(a, b)| a * b).sum()
    }
}

// These all compile — N is inferred from the argument
let a = [1, 2, 3];
let b = [4, 5, 6];
println!("{}", sum(&a));     // N = 3, inferred
println!("{}", dot(&a.map(|x| x as f64), &b.map(|x| x as f64)));

// These are DIFFERENT types — the compiler won't mix them up
let v3: Vector<3> = Vector::new([1.0, 2.0, 3.0]);
let v4: Vector<4> = Vector::new([1.0, 2.0, 3.0, 4.0]);
// v3.dot(&v4)  ← COMPILE ERROR: expected Vector<3>, found Vector<4>

// Reverse a fixed-size array — returns the same type
fn reversed<T: Copy + Default, const N: usize>(arr: &[T; N]) -> [T; N] {
    let mut out = [T::default(); N];
    for i in 0..N {
        out[i] = arr[N - 1 - i];
    }
    out
}
```

Key points:
- `const N: usize` goes in the same angle brackets as type parameters: `fn f<T, const N: usize>`
- The compiler infers `N` from array literals: `sum(&[1, 2, 3])` infers `N = 3`
- `[T::default(); N]` creates a zero-initialized array of const size — `T: Default` is required
- Two `Vector<N>` with different `N` are incompatible types — the compiler enforces this
- Available on stable Rust since 1.51 (2021)

## What This Unlocks

- **Dimension-safe math**: `Matrix<M, N>` multiplication that requires matching dimensions at compile time — the same guarantee that Haskell's `Vec` and dependent types provide
- **Stack-allocated fixed-size structures**: ring buffers, queues, and sliding windows with no heap allocation and no runtime bounds checks on the size
- **Zero-overhead abstractions**: a `Vector<N>` compiles to the same machine code as a raw `[f64; N]` — the type parameter exists only at compile time

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Const generics | No direct equivalent (GADTs approximate) | `fn f<const N: usize>` — stable since 1.51 |
| Fixed array in generic | `'a array` — dynamic size, no static guarantee | `[T; N]` — size is part of the type |
| Type-level integers | Peano encoding with GADTs | `const N: usize` — plain integer |
| Dimension mismatch | Runtime check | Compile error — different types |
| Type inference | Hindley-Milner infers universally | `N` inferred from array argument |
| Multiple const params | N/A | `struct Matrix<const R: usize, const C: usize>` |

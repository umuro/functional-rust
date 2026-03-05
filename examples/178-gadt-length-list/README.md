📖 **[View on hightechmind.io →](https://hightechmind.io/rust/178-gadt-length-list)**

---

# 178: Length-Indexed Lists

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Track a list's length in its type so that `head` on an empty list and `zip` on mismatched lengths are compile-time errors, not runtime panics.

## The Problem This Solves

`Vec::first()` returns `Option<T>` because the compiler doesn't know if the vec is empty. This is honest but verbose: you unwrap at every call site, scatter `expect("this can't be empty")` through your code, and trust that the invariant actually holds. If you're wrong, you panic at runtime. The type system shrugs.

The deeper issue arises with operations like `zip`. If you zip two lists of different lengths, what do you get? Different languages make different choices (truncate, panic, error), but all of them make the choice at runtime. A length-indexed list catches the mismatch at compile time — the type `[T; 3]` can only zip with another `[U; 3]`, full stop.

OCaml's GADT approach uses Peano-encoded naturals at the type level: `type zero`, `type 'n succ`, and `('a, 'n succ) vec` is a list with at least one element. Rust's const generics offer a cleaner modern alternative: `Vec2<T, const N: usize>` carries the length directly in the type parameter. Either way, the length becomes part of the type, and the compiler enforces it.

## The Intuition

Think of differently-sized bags. A `Bag<3>` holds exactly 3 items. You can zip `Bag<3>` with another `Bag<3>` — both have the same type parameter. You can't zip `Bag<3>` with `Bag<5>` — the compiler sees different types. And `head` works on any `Bag<N>` where the compiler can prove `N ≥ 1`, which you encode via trait bounds or const constraints.

Rust const generics make this ergonomic: `struct Vec2<T, const N: usize>` is a real compile-time feature. The length `N` is part of the type signature, visible in documentation, and checked by the compiler. The Peano approach (`Zero`, `Succ<N>`) is more academic but mirrors OCaml exactly — useful for understanding the theory.

## How It Works in Rust

```rust
// Approach 1: const generics — most ergonomic in modern Rust
#[derive(Debug, Clone)]
struct Vec2<T, const N: usize> {
    data: [T; N],
}

// zip only compiles when BOTH arrays have the same length N
fn zip_vec<T: Copy, U: Copy, const N: usize>(
    a: &Vec2<T, N>,
    b: &Vec2<U, N>,
) -> Vec2<(T, U), N> {
    let mut result = [(a.data[0], b.data[0]); N]; // simplified
    for i in 0..N { result[i] = (a.data[i], b.data[i]); }
    Vec2 { data: result }
}

// Usage:
let xs = Vec2 { data: [1, 2, 3] };      // Vec2<i32, 3>
let ys = Vec2 { data: [4, 5, 6] };      // Vec2<i32, 3>
let zipped = zip_vec(&xs, &ys);          // Vec2<(i32,i32), 3> ✓

// This fails to compile — different lengths:
// let ys2 = Vec2 { data: [4, 5] };     // Vec2<i32, 2>
// zip_vec(&xs, &ys2);                  // N=3 ≠ N=2

// Approach 2: Peano types — mirrors OCaml GADTs
struct Zero;
struct Succ<N>(std::marker::PhantomData<N>);

enum TypeVec<T, N> {
    VNil,                                // only valid when N = Zero
    VCons(T, Box<TypeVec<T, N>>),        // only valid when N = Succ<M>
}
// OCaml: ('a, zero) vec = VNil | ('a, 'n succ) vec = VCons
```

The const generic approach is what you'd use in practice. The Peano approach demonstrates the theoretical connection to type-level natural number arithmetic.

## What This Unlocks

- **Matrix operations** — `Matrix<T, R, C>` where matrix multiplication `Matrix<T, M, N> × Matrix<T, N, P> -> Matrix<T, M, P>` checks dimension compatibility at compile time.
- **Fixed-size protocol buffers** — network messages with a guaranteed field count; serialization/deserialization functions can be written without bounds checks.
- **SIMD/vectorized code** — wrapping aligned chunks of a known fixed size, ensuring operations only apply to correctly-sized vectors.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Length encoding | Peano types: `type zero`, `type 'n succ` | Const generics: `const N: usize` (or manual Peano structs) |
| Non-empty guarantee | `(a, 'n succ) vec` — type-level proof of ≥1 element | `const N: usize` alone can't express `N > 0`; use `NonEmptyVec<T>` pattern instead |
| Zip | `('a, 'n) vec -> ('b, 'n) vec` — same `'n` forces equal length | `Vec2<T, N>` and `Vec2<U, N>` — same `N` enforces equal length |
| Ergonomics | Natural with GADTs but verbose Peano | Const generics are clean; Peano is verbose in Rust |
| Arithmetic on lengths | Type-level addition possible with GADTs | Const generics support `const { N + M }` in nightly; stable is limited |

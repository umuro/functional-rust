# 126: Const Generics

**Difficulty:** ⭐⭐  **Level:** Intermediate

Encode array sizes and matrix dimensions as type parameters so dimension mismatches become compile errors, not runtime panics.

## The Problem This Solves

Imagine you're writing a linear algebra library. You have a `Matrix` type, and you want to multiply two matrices together. Matrix multiplication requires that the number of columns in the first matrix equals the number of rows in the second — a `2×3` times a `3×4` gives a `2×4`, but `2×3` times `4×3` is illegal. In a typical Rust struct with `Vec<Vec<f64>>` inside, nothing stops you from passing the wrong shapes at runtime. You find out when your program panics — or worse, silently produces wrong results.

The same problem shows up with fixed-size arrays. A function that computes a dot product over two 3D vectors should reject two 4D vectors. Without const generics you'd either check at runtime (and handle the error) or use separate types `Vec3`, `Vec4`, etc. — which doesn't scale.

Const generics let you express "this struct holds exactly N elements" or "this matrix is M rows × N columns" directly in the type. The compiler checks dimensions at every call site. Pass a `Matrix<3,4>` where a `Matrix<4,3>` is expected? Compile error. No runtime cost, no boilerplate size-checking code.

## The Intuition

Think of const generics as type parameters that carry numbers instead of types. Just as `Vec<T>` is generic over an element type `T`, `FixedArray<T, const N: usize>` is generic over *both* the element type *and* the length. `FixedArray<f64, 3>` and `FixedArray<f64, 4>` are completely different types — you can't mix them up.

Matrix multiplication makes this shine. The rule is: `Matrix<M, N>` × `Matrix<N, P>` = `Matrix<M, P>`. The inner dimension `N` must match. In Rust you express this as a method signature: `fn mul<const P: usize>(&self, other: &Matrix<T, N, P>) -> Matrix<T, M, P>`. The compiler sees that `other` must share the same `N` as `self`, and enforces it — no runtime check required.

## How It Works in Rust

```rust
// A fixed-size array where N is part of the type
#[derive(Debug, Clone)]
struct FixedArray<T, const N: usize> {
    data: [T; N],          // N is a const value, not a type — but it lives in the type signature
}

impl<T: Default + Copy, const N: usize> FixedArray<T, N> {
    fn new(default: T) -> Self {
        FixedArray { data: [default; N] }  // [default; N] works because N is known at compile time
    }

    fn len(&self) -> usize {
        N  // the length is part of the type — no need to store it
    }
}

// Dot product only compiles if both arrays have the same length N
impl<const N: usize> FixedArray<f64, N> {
    fn dot(&self, other: &Self) -> f64 {  // &Self means &FixedArray<f64, N> — same N
        self.data.iter().zip(other.data.iter()).map(|(a, b)| a * b).sum()
    }
}

// Matrix: both dimensions are const generics
struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

// Multiplication: compile-time dimension check
// self is M×N, other must be N×P, result is M×P
impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where T: Default + Copy + std::ops::Add<Output = T> + std::ops::Mul<Output = T>
{
    fn mul<const P: usize>(&self, other: &Matrix<T, N, P>) -> Matrix<T, M, P> {
        // N in `self` must equal N in `other` — the compiler verifies this
        let mut result = Matrix::<T, M, P>::new(T::default());
        // ...
        result
    }
}
```

Usage:
```rust
let m1: Matrix<f64, 2, 3> = Matrix::new(0.0);
let m2: Matrix<f64, 3, 2> = Matrix::new(0.0);
let m3: Matrix<f64, 2, 2> = m1.mul(&m2);  // compiles — 3 matches 3

// let bad = m1.mul(&m1);   // compile error! Matrix<2,3> × Matrix<2,3> — 3 ≠ 2
```

## What This Unlocks

- **Dimension-safe linear algebra** — `nalgebra` uses const generics so `Vector3<f64>` and `Vector4<f64>` are distinct types; wrong-size operations are compile errors.
- **Protocol buffers / message framing** — encode expected payload sizes in types to prevent buffer overreads without runtime bounds checks.
- **Stack-allocated collections** — `ArrayVec<T, N>` stores up to N items on the stack; the capacity limit is a compile-time guarantee, not a runtime field.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Size as type param | Functor `FixedArray(Size)` using a module | `FixedArray<T, const N: usize>` — value directly in the type |
| Matrix dims | Runtime check `failwith "dimension mismatch"` | Compile-time: `Matrix<M,N>` × `Matrix<N,P>` enforced by the type checker |
| Const evaluation | Module-level `let` computed at startup | `const` items evaluated during compilation, embedded in binary |
| Syntax | Module system + functors | Inline `const N: usize` parameter, same syntax as type generics |

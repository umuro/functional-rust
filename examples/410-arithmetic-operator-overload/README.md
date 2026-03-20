📖 **[View on hightechmind.io →](https://hightechmind.io/rust/410-arithmetic-operator-overload)**

---

# 410: Arithmetic Operator Overloading

## Problem Statement

Mathematical types — vectors, matrices, complex numbers, rational numbers, polynomials — are naturally expressed with arithmetic notation: `v1 + v2`, `m * v`, `-c`. Without operator overloading, these become `v1.add(&v2)`, making code verbose and harder to read than the underlying mathematics. Rust's `std::ops` module provides traits for every arithmetic operator: `Add`, `Sub`, `Mul`, `Div`, `Neg`, `Rem`, and their `*Assign` variants. Implementing these makes custom types feel like first-class numeric citizens.

Operator overloading powers Rust's numeric libraries: `nalgebra` (linear algebra), `num` (arbitrary precision), `ndarray` (array computing), and embedded DSP/signal processing libraries.

## Learning Outcomes

- Understand how `std::ops::Add`, `Sub`, `Mul`, `Div`, `Neg` enable arithmetic operators for custom types
- Learn the associated type `Output` that determines the result type of each operation
- See how `AddAssign` enables `+=` in addition to `+`
- Understand how scalar multiplication (`Vec2 * f64`) requires separate impl from vector multiplication
- Learn how `Copy` types benefit from consuming `self` in operator implementations

## Rust Application

In `src/lib.rs`, `Vec2` implements `Add`, `Sub`, `Mul<f64>` (scalar), `Neg`, and `AddAssign`. Each impl takes `self` by value (since `Vec2: Copy`) and returns a new `Vec2`. `Mul<Vec2> for f64` enables `3.0 * v` in addition to `v * 3.0`. The `Output = Vec2` associated type declares the result type. `AddAssign` enables `v += other` in-place mutation. `fmt::Display` formats as `(x, y)` for readable output.

## OCaml Approach

OCaml supports operator overloading through module local redefinition. `let (+) = Vec2.add` redefined locally within a module shadow the built-in `+` for the scope. This is more fragile than Rust's trait system — the shadows apply to the entire module and cannot be scope-restricted to specific types. The `Base` library uses module-local operator overloading extensively for its numeric types.

## Key Differences

1. **Type dispatch**: Rust's operator overloading is type-directed — the compiler selects the right `Add` impl based on operand types; OCaml's operator shadows apply lexically.
2. **Mixed-type ops**: Rust can implement `Add<f64> for Vec2` and `Add<Vec2> for f64` separately; OCaml requires either uniform types or explicit conversions.
3. **No global pollution**: Rust's overloaded operators only affect the specific types involved; OCaml's `let (+) = Vec2.add` shadows the integer `+` for the whole module scope.
4. **Commutativity**: Rust requires separate impls for `a + b` and `b + a` when types differ; OCaml's `+` is always a single function.

## Exercises

1. **Complex number**: Implement `Complex { re: f64, im: f64 }` with all four arithmetic operators plus `Neg`. Ensure `i * i = -1` passes as a test. Implement `Display` as `a + bi`.
2. **Matrix multiplication**: Create a `Matrix2x2([[f64; 2]; 2])` and implement `Mul<Matrix2x2>` (matrix multiplication), `Mul<Vec2>` (matrix-vector product), and `Add<Matrix2x2>`. Verify with the identity matrix.
3. **Polynomial arithmetic**: Implement `Polynomial(Vec<f64>)` where `coeffs[i]` is the coefficient of `x^i`. Implement `Add` (zip coefficients), `Sub`, `Mul` (polynomial multiplication using convolution), and `Display` (`3x^2 + 2x + 1`).

📖 **[View on hightechmind.io →](https://hightechmind.io/rust/783-const-type-arithmetic)**

---

# 783-const-type-arithmetic — Const Type Arithmetic

## Problem Statement

Type-level arithmetic allows the type system to reason about sizes and dimensions. `Add<3, 4>::VALUE == 7` and `Mul<3, 4>::VALUE == 12` are computed by the compiler. This enables types like `Matrix<f64, 3, 4>` and `Matrix<f64, 4, 5>` to multiply only when the inner dimensions match — a matrix multiplication type error becomes a compile error. Used in linear algebra libraries (`nalgebra`) and tensor libraries that enforce dimension compatibility.

## Learning Outcomes

- Implement `Add<A, B>` and `Mul<A, B>` structs with `const VALUE: usize`
- Create `Vec3<const N: usize>` with typed length and slice access
- Implement `concat_vec` that combines two vectors and returns a `Vec<f64>` (not `[f64; A+B]` — limited by stable Rust)
- Understand why `Matrix<ROWS, COLS>` multiplication checks inner dimensions at compile time
- See how nalgebra uses this for dimension-safe linear algebra operations

## Rust Application

`Add<A, B>` provides `const VALUE: usize = A + B`. `Mul<A, B>` provides `const VALUE: usize = A * B`. `Vec3<N>` wraps `[f64; N]` with typed length. `Matrix<const R: usize, const C: usize>` wraps `[[f64; C]; R]` with `mul` that only accepts `Matrix<C, D>`. `concat_vec` returns `Vec<f64>` because stable Rust cannot use `{A+B}` as a const generic expression directly. Comments explain the nightly approach.

## OCaml Approach

OCaml achieves type-level dimension checking via GADTs and phantom type arithmetic. Libraries like `Tensorflow_ocaml` use shape-indexed tensors. `tensor-ocaml` uses `Z.t Succ.t` for natural number type encoding (Peano arithmetic). While more verbose than Rust's const generics, OCaml's GADT approach can express more complex invariants. `linalg` and `owl` libraries sacrifice type-level safety for practical usability.

## Key Differences

1. **Ergonomics**: Rust's `Matrix<3, 4>` is concise; OCaml's Peano encoding (`Succ (Succ (Succ Zero))`) is verbose and impractical for large dimensions.
2. **Expression restriction**: Stable Rust cannot write `[f64; A + B]` as a generic array size; OCaml's GADTs don't have this restriction since all arrays are heap-allocated.
3. **Library support**: `nalgebra` (Rust) uses const generics for dimension-safe matrices with excellent ergonomics; OCaml lacks an equivalent mature library.
4. **Nightly features**: Rust nightly allows `{ A + B }` in const generic expressions; this enables `concat_arr<A, B>() -> [f64; A+B]` without `Vec`.

## Exercises

1. Implement `Matrix<R, C>::transpose() -> Matrix<C, R>` that reverses the dimensions in the type signature.
2. Write a `dot_product<const N: usize>(a: &Vec3<N>, b: &Vec3<N>) -> f64` that computes the inner product — the same `N` constraint prevents mismatched lengths.
3. Implement `outer_product<const M: usize, const N: usize>(a: &Vec3<M>, b: &Vec3<N>) -> Matrix<M, N>` that computes the outer product with correct dimension types.

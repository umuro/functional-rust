📖 **[View on hightechmind.io →](https://hightechmind.io/rust/774-const-generics-basics)**

---

# 774-const-generics-basics — Const Generics Basics
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Generic programming traditionally parameterizes types by other types. Const generics extend this to values known at compile time — primarily integer constants. This enables `Array<i32, 8>` and `Array<i32, 16>` to be different types with their sizes embedded in the type system, eliminating runtime bounds checks and enabling stack allocation. Stabilized in Rust 1.51, const generics are used in `ndarray`, embedded-hal, and the standard library's `[T; N]` arrays.

## Learning Outcomes

- Declare a struct with a `const N: usize` type parameter
- Implement `const fn len(&self) -> usize { N }` — a compile-time constant method
- Understand why `Array<T, 8>` and `Array<T, 16>` are different types at compile time
- Use `[T::default(); N]` for zero-initialization with const generic size
- See how const generics enable stack-allocated containers without heap allocation

## Rust Application

`Array<T, const N: usize>` wraps `data: [T; N]`. `new()` fills with `T::default()`. `len()` and `is_empty()` are `const fn` returning `N`. `get` and `set` provide safe indexed access. `Default` is derived by requiring `T: Default + Copy`. The `size_of::<Array<u8, 8>>()` test verifies the type is exactly 8 bytes — no heap, no length field. Tests verify that `Array<i32, 4>` and `Array<i32, 8>` are genuinely different types.

## OCaml Approach

OCaml has no direct equivalent of const generics. Array sizes are runtime values: `Array.make n value`. The `Bigarray` module allows specifying element kinds but not sizes in the type. OCaml 5's effect system doesn't address this gap. For fixed-size types, OCaml uses phantom type parameters with module-level constants: `module Fixed8 : sig type t val size : int end = struct type t = int array let size = 8 end`.

## Key Differences

1. **Type-level sizes**: Rust's `Array<T, N>` encodes `N` in the type; OCaml arrays have runtime-only sizes.
2. **Compile-time verification**: Rust can reject mismatched sizes at compile time (e.g., function expecting `[u8; 32]` getting `[u8; 16]`); OCaml needs runtime checks.
3. **Stack allocation**: Rust's `[T; N]` is always stack-allocated when N is small; OCaml arrays are always heap-allocated.
4. **GADTs**: OCaml's GADTs can encode some size relationships at the type level, but it requires more boilerplate than Rust's const generics.

## Exercises

1. Implement `zip<const N: usize>(a: Array<A, N>, b: Array<B, N>) -> Array<(A,B), N>` that pairs elements — the compiler ensures both arrays have the same size.
2. Add `fn concat<const A: usize, const B: usize>(left: Array<T, A>, right: Array<T, B>) -> [T; A+B]` — note this requires nightly's const arithmetic; discuss the limitation.
3. Implement a `Matrix<T, const ROWS: usize, const COLS: usize>` with `mul` that only compiles when the inner dimensions match.

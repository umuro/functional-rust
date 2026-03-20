📖 **[View on hightechmind.io →](https://hightechmind.io/rust/781-const-where-bounds)**

---

# 781-const-where-bounds — Const Where Bounds
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Const generics can accept any `usize` value, but many types have validity constraints: a buffer must be non-empty, a size must be a power of two, a dimension must be positive. On stable Rust, these constraints are enforced via runtime assertions in `new()`. On nightly, `where [(); N - 1]: Sized` and similar tricks enforce constraints at compile time. This example shows both approaches and explains why the nightly technique is not yet stable.

## Learning Outcomes

- Enforce `N >= 1` at runtime in `new()` for a `NonEmptyArray<T, N>`
- Enforce power-of-two `SIZE` for `PowerOfTwoBuffer<SIZE>` using a runtime assert
- Understand why `where [(); N - 1]:` compiles on nightly but not stable
- Use `const_assert!` in constructors to provide early, clear error messages
- See how these patterns are used in embedded HAL crates for register sizes

## Rust Application

`NonEmptyArray<T, N>` panics in `new()` if `N == 0`, providing a clear error message. `first()` and `last()` are infallible because the non-empty invariant is checked at construction. `PowerOfTwoBuffer<SIZE>` asserts `SIZE > 0 && (SIZE & (SIZE - 1)) == 0`. `AlignedBuffer<const ALIGN: usize>` checks alignment is a power of two. The example discusses the nightly `where [(); N]:` trick in comments.

## OCaml Approach

OCaml enforces constraints at the module functor level: `module Make(N: sig val n: int end) : sig ... end = struct let () = assert (N.n >= 1) ... end`. This makes the assertion happen at module creation time, similar to Rust's `new()` assert. GADTs allow type-level encoding of some constraints: `type 'n positive = Positive : positive_int -> positive positive` using phantom types.

## Key Differences

1. **Compile vs runtime**: Nightly Rust can enforce const bounds at compile time; stable Rust and OCaml both use runtime assertions in constructors.
2. **Error location**: Rust's compile-time bounds produce errors at the point of type instantiation; runtime asserts fail at `new()` call, which may be distant from the invalid literal.
3. **Functor approach**: OCaml's functor + module `Make(N)` is analogous to Rust's `NonEmptyArray<T, N>::new()` — both check `N` at construction.
4. **Stability**: Rust's nightly const-bound mechanism (`where [(); EXPR]:`) is unstable; use runtime asserts in production code.

## Exercises

1. Implement `WindowBuffer<T, const WINDOW: usize, const STEP: usize>` that asserts `STEP <= WINDOW` and provides a `slide()` method.
2. Add a `MinMaxBuffer<T, const MIN_CAP: usize, const MAX_CAP: usize>` that asserts `MIN_CAP <= MAX_CAP` and stores between `MIN_CAP` and `MAX_CAP` elements.
3. Experiment with the nightly `where [(); N - 1]:` trick in a nightly build and document the compile error that it produces for `N = 0`.

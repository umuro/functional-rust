📖 **[View on hightechmind.io →](https://hightechmind.io/rust/778-const-fibonacci)**

---

# 778-const-fibonacci — Const Fibonacci
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Fibonacci numbers appear in algorithm analysis, financial modeling, and data structure design (Fibonacci heaps). Computing them at compile time eliminates startup overhead for lookup tables and serves as a benchmark for `const fn` capability. This example implements three approaches — naive recursion, iterative, and matrix exponentiation — and compares their feasibility and efficiency in `const` contexts, demonstrating how `const fn` restrictions shape algorithm choices.

## Learning Outcomes

- Implement `const fn fib_recursive(n: u64)` — exponential but simple, limited by recursion depth limits
- Implement `const fn fib_iterative(n: u64)` using `while` loops — O(n), works for large n
- Implement `const fn fib_matrix(n: u64)` using matrix exponentiation — O(log n) in const context
- Compute `const FIB_30: u64 = fib_iterative(30)` to verify compile-time evaluation
- Understand why recursive const functions hit depth limits for large inputs

## Rust Application

`fib_recursive` uses a match expression and double recursion — exponential time, limited to ~n=30 in const due to recursion depth. `fib_iterative` uses `while` loop variables, O(n), works for n up to ~90 before u64 overflow. `fib_matrix` implements matrix exponentiation by squaring in const, achieving O(log n) even at compile time. All three are verified by `const FIBx: u64 = fib_*(n)` constants that would fail to compile if incorrect.

## OCaml Approach

OCaml cannot compute Fibonacci at compile time directly. Module-level `let` bindings are evaluated at program startup, not during compilation. A common approach is code generation: a script generates `let fib_table = [| 0; 1; 1; 2; 3; 5; ... |]` as a source file. For small n, the `[@@unrolled]` attribute can help the compiler specialize, but this is not true compile-time computation.

## Key Differences

1. **True compile time**: Rust's `const fn fib(n)` evaluates during `rustc` compilation; OCaml's equivalent runs at program startup.
2. **Recursion depth**: Rust limits `const fn` recursion depth (configurable via `-Zunleash-the-miri-inside-of-you`); OCaml's startup-time evaluation has no such limit.
3. **Loop support**: Rust's `while` loop in `const fn` (since Rust 1.46) enables efficient iterative algorithms; OCaml has no equivalent.
4. **Build scripts**: Both languages can generate lookup tables via build scripts (`build.rs` in Rust, Dune rules in OCaml), avoiding language-level const computation.

## Exercises

1. Use `const fn fib_iterative` to generate a complete Fibonacci lookup table: `const FIB_TABLE: [u64; 93] = generate_fib_table()`.
2. Implement `const fn fib_last_digit(n: u64) -> u8` that computes `fib(n) % 10` without overflow, using the Pisano period property that the last digit repeats with period 60.
3. Write a `const fn golden_ratio_approx(n: u32) -> (u64, u64)` that returns `(fib(n+1), fib(n))` as a rational approximation of the golden ratio.

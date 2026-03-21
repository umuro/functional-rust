📖 **[View on hightechmind.io →](https://hightechmind.io/rust/127-const-functions)**

---

# Const Functions — Compile-Time Computation
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Lookup tables, mathematical constants, and protocol values are often computed once and used throughout a program's lifetime. Computing them at runtime wastes startup time; hand-coding them as literals is error-prone and hard to maintain. Rust's `const fn` evaluates functions at compile time — the result is baked into the binary as a constant, with zero runtime cost and full verification that the computation is correct at build time.

## Learning Outcomes

- Understand what `const fn` is and the restrictions it operates under (no heap allocation, limited control flow)
- Learn how to compute Fibonacci numbers, powers, and lookup tables at compile time
- See how `const` items computed by `const fn` appear in binaries as read-only data
- Understand the practical use cases: cryptographic S-boxes, CRC tables, sine lookup tables

## Rust Application

`fibonacci(n: u64)` uses an iterative loop (allowed in `const fn` since Rust 1.46) instead of recursion to avoid stack growth concerns. `FIB_10`, `FIB_20`, `FIB_30` are computed entirely by the compiler — LLVM sees them as immediate constants. `build_square_table()` produces a `[u32; 256]` array in the binary's read-only section. Compile-time evaluation means the runtime function is never called for these constants.

## OCaml Approach

OCaml evaluates module-level expressions at program startup (runtime initialization), not at compile time. `let fib_10 = fibonacci 10` runs when the module is first loaded, not when the binary is built. OCaml's `ppx_const` or meta-programming via `camlp4` can move some computation to compile time, but these are third-party tools rather than a built-in language feature.

## Key Differences

1. **Evaluation timing**: Rust `const fn` results are embedded in the binary at compile time; OCaml module-level expressions run at program startup.
2. **Restrictions**: Rust `const fn` cannot allocate heap memory, call non-const functions, or use floating point (in stable const contexts); OCaml has no such restrictions on startup code.
3. **Verification**: Rust compile-time evaluation catches panics (overflow, divide-by-zero) as compile errors; OCaml startup panics only at runtime.
4. **Lookup tables**: Rust can build `[T; N]` tables in `const fn`; OCaml requires runtime initialization of arrays.

## Exercises

1. Write a `const fn` that computes the nth triangular number and define `TRIANGULAR_100` as a compile-time constant.
2. Build a CRC-32 lookup table as a `const [u32; 256]` array using `const fn`.
3. Verify that `size_of_val(&SQUARE_TABLE)` equals 1024 bytes (256 × 4) and that it lives in the binary's read-only section using `cargo objdump`.

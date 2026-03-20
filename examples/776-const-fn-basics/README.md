📖 **[View on hightechmind.io →](https://hightechmind.io/rust/776-const-fn-basics)**

---

# 776-const-fn-basics — Const Fn Basics

## Problem Statement

`const fn` allows computations to run at compile time, moving work from program startup to compilation. This eliminates initialization overhead for lookup tables, mathematical constants, and hash seeds. Stabilized incrementally since Rust 1.31, `const fn` now supports loops, conditional expressions, and recursive functions. It is used in `phf` (perfect hash functions computed at compile time), `const-oid`, and array-init crates.

## Learning Outcomes

- Write `const fn factorial(n: u64) -> u64` using recursive `const fn`
- Implement `const fn gcd(a, b)` and `const fn pow(base, exp)` for compile-time math
- Use `const fn is_prime(n: u64)` with a loop (available in `const` since Rust 1.46)
- Verify results with `const X: u64 = factorial(10)` — computed at compile time
- Understand which operations are forbidden in `const fn` (heap allocation, `dyn Trait`, etc.)

## Rust Application

`factorial` is recursive `const fn` (computes `10! = 3628800` at compile time). `gcd` uses Euclid's algorithm with a recursive `const fn`. `pow` uses exponentiation by squaring. `is_prime` uses a `while` loop — valid in `const` context. Constants are declared as `const FACT_10: u64 = factorial(10)`, proving compile-time evaluation. The example also shows the limit: `const fn` cannot call `println!` or allocate.

## OCaml Approach

OCaml has no direct equivalent of `const fn`. Values that can be computed at compile time are limited to constant literals and simple arithmetic in module-level `let` bindings that the compiler evaluates. For lookup tables, OCaml uses `[@@unrolled]` loops or preprocessor-based code generation. `ppx_const` provides limited compile-time conditionals. Jane Street's `ppx_sexp_conv` generates code at compile time but doesn't compute arbitrary values.

## Key Differences

1. **Scope**: Rust's `const fn` can compute arbitrary values at compile time; OCaml's compile-time computation is limited to simple constant folding.
2. **Lookup tables**: Rust computes full lookup tables in `const fn`; OCaml must either hardcode them or compute them at runtime initialization.
3. **Recursion**: Rust's `const fn` supports recursion; OCaml's constant expressions do not.
4. **Restrictions**: Rust's `const fn` cannot use heap allocation, `dyn Trait`, or floating-point `sin`/`cos` (they're not yet stable `const`); OCaml's limitations are different.

## Exercises

1. Write `const fn is_power_of_two(n: u64) -> bool` and use it in a `const_assert!` to validate a buffer size at compile time.
2. Implement `const fn nth_fibonacci(n: u32) -> u64` using the iterative approach (more efficient than recursive for const contexts) and compute `const FIB_50: u64`.
3. Write `const fn count_bits_set(n: u64) -> u32` (popcount) and verify `count_bits_set(0b1010_1010) == 4` at compile time.

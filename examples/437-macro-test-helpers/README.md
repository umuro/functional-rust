📖 **[View on hightechmind.io →](https://hightechmind.io/rust/437-macro-test-helpers)**

---

# 437: Test Helper Macros
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Test code benefits from higher-level assertions than `assert_eq!`. Floating-point tests need approximate equality, panic testing needs `catch_unwind` boilerplate, and table-driven tests need repetition over input/output pairs. Test helper macros eliminate this repetition: `assert_approx!(a, b)`, `assert_panics!(expr)`, `test_cases!(fn, input => expected, ...)`. These make tests more readable, reduce boilerplate, and ensure consistent error messages when assertions fail.

Test helper macros are widespread: `approx` crate for float comparison, `proptest` for property-based testing, `rstest` for parametrized tests, and countless internal test utilities.

## Learning Outcomes

- Understand how test macros reduce boilerplate in test suites
- Learn how `assert_approx!` uses a default epsilon with override capability
- See how `std::panic::catch_unwind` enables testing that code panics
- Understand `test_cases!` as a lightweight parametrized test approach
- Learn how test macros improve assertion failure messages with context

## Rust Application

In `src/lib.rs`, `assert_approx!($left, $right)` defaults to epsilon `1e-6` and delegates to the two-argument form. `assert_panics!($body)` uses `std::panic::catch_unwind(|| $body)` and asserts the result is `Err` (a panic was caught). `test_cases!($func, $input => $expected, ...)` expands to individual `assert_eq!` calls for each pair. All macros include descriptive failure messages using `stringify!` or formatted values.

## OCaml Approach

OCaml's `alcotest` library provides `check float "msg" expected actual` with epsilon support. `OUnit2.assert_raises` tests for exceptions. `QCheck` provides property-based testing. OCaml's `ppx_expect` generates inline snapshot tests. The test ecosystem is library-based rather than macro-based, but achieves the same goals. `ppx_inline_test` with `let%test_unit "name" = ...` syntax provides test helpers via PPX.

## Key Differences

1. **Panic vs. exception**: Rust's `assert_panics!` uses `catch_unwind`; OCaml's `assert_raises` is a function that catches exceptions — both serve the same purpose.
2. **Float precision**: Rust needs macros for custom epsilon; OCaml's `alcotest` has built-in float equality with configurable precision.
3. **Table tests**: Rust's `test_cases!` expands to multiple `assert_eq!` calls; OCaml's `List.iter` provides the same with cleaner syntax.
4. **Framework integration**: Rust test macros work within Rust's built-in test framework; OCaml test helpers require a testing library (`alcotest`, `ounit`, `qcheck`).

## Exercises

1. **Matrix assertion**: Implement `assert_matrix_eq!(m1, m2, epsilon)` that compares two 2D `Vec<Vec<f64>>` element-wise within epsilon, printing the differing element indices and values when they differ.
2. **Property test macro**: Create `test_property!(fn_name: |x: i32| { /* property */ }, count: 1000)` that generates 1000 random `i32` values and asserts the property holds for each, printing the failing input on assertion failure.
3. **Benchmark macro**: Implement `bench!(name, warmup: 100, runs: 1000, { /* code */ })` that runs the code `warmup` times, then `runs` times, computing mean and stddev of the timing, printing a summary. Use `Instant::now()` in the expansion.

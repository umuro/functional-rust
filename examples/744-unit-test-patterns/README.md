📖 **[View on hightechmind.io →](https://hightechmind.io/rust/744-unit-test-patterns)**

---

# 744-unit-test-patterns — Unit Test Patterns
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Good unit tests follow a consistent structure, isolate concerns, and fail with informative messages. The Arrange-Act-Assert (AAA) pattern makes test intent clear. Grouping tests into `mod` blocks by feature reduces noise in output. Custom assertion helpers (`assert_approx_eq`, `assert_sorted`) reduce repetition and improve failure messages. These patterns are standard practice in Rust codebases at companies like Mozilla, AWS, and Cloudflare.

## Learning Outcomes

- Organize tests in `#[cfg(test)] mod tests` submodules by feature area
- Apply the Arrange-Act-Assert pattern for clear, readable tests
- Write custom assertion helpers that produce actionable failure messages
- Use `#[should_panic(expected = "...")]` for testing panic behavior
- Understand the difference between `assert_eq!` (requires `PartialEq + Debug`) and custom comparators

## Rust Application

The example provides `clamp`, `divide_checked`, `is_palindrome`, and `fizzbuzz` as functions under test. Tests are grouped into `mod tests_clamp`, `mod tests_divide`, etc. The `helpers` module (also `#[cfg(test)]`) provides `assert_approx_eq` for floating-point comparisons and `assert_sorted` for slice ordering. Each test follows AAA: data setup, function call, assertion. Edge cases (boundaries, zero divisors, empty strings) have dedicated tests.

## OCaml Approach

OCaml uses the `OUnit2` or `Alcotest` framework for unit testing. Tests are organized in `suite` lists and run via `Alcotest.run`. `Alcotest` provides typed testable values (`Alcotest.int`, `Alcotest.string`) for structured failure messages. Unlike Rust's built-in `#[test]`, OCaml requires an explicit test runner executable. Jane Street uses `ppx_inline_test` for inline tests in source files, similar to Rust's `#[test]` in source files.

## Key Differences

1. **Built-in vs library**: Rust's test framework is built into `cargo test`; OCaml requires `OUnit2`, `Alcotest`, or `ppx_inline_test` as external dependencies.
2. **Isolation**: Rust tests run in parallel by default (each in a thread); OCaml's `Alcotest` runs tests sequentially unless using its parallel mode.
3. **Inline tests**: Rust tests live in the same file as the code under test; OCaml typically separates tests into a `test/` directory (unless using `ppx_inline_test`).
4. **Custom assertions**: Rust's macro system makes `assert_approx_eq!` macros ergonomic; OCaml uses first-class testable values with `Alcotest.testable`.

## Exercises

1. Add a `mod tests_fizzbuzz` that tests every class: pure numbers, multiples of 3, multiples of 5, and multiples of both. Use a table-driven test with `[(input, expected)]` pairs.
2. Write a `assert_no_duplicates<T: Eq + Hash>` test helper that checks a slice contains no repeated elements and prints the duplicate on failure.
3. Add property-based tests for `is_palindrome` that verify the law `is_palindrome(s) == is_palindrome(reverse(s))` for randomly generated strings.

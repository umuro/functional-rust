📖 **[View on hightechmind.io →](https://hightechmind.io/rust/439-macro-assert-variants)**

---

# 439: Assert Variant Macros
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Testing enum variants is verbose: `assert!(matches!(result, Ok(_)))` requires knowing the `matches!` macro, wrapping in `assert!`, and loses the ability to extract the inner value. `unwrap_variant!(result, Ok(v) => v)` provides a cleaner pattern: assert that the value matches a variant and extract the inner data in one operation. For test suites heavily using `Result` and enum-heavy domain models, these macros significantly reduce test boilerplate while providing better error messages.

Variant assertion macros appear in `assert_matches!` (stabilized in Rust 1.73), `unwrap_variant`, testing framework helper crates, and any codebase testing enum-heavy APIs.

## Learning Outcomes

- Understand how `matches!($val, $pattern)` provides a boolean pattern check
- Learn how `assert_matches!` combines `matches!` with assertion and error message
- See how `unwrap_variant!` uses `match` with a wildcard arm to extract or panic
- Understand the `stringify!($pattern)` technique for showing the pattern in error messages
- Learn that `assert_matches!` was stabilized in `std` in Rust 1.73

## Rust Application

In `src/lib.rs`, `assert_matches!($value, $pattern)` wraps `matches!($value, $pattern)` in `assert!` with a descriptive message including both the actual value (`{:?}`) and the expected pattern text (via `stringify!`). `unwrap_variant!($value, $pattern => $extracted)` uses a match expression where only the expected variant succeeds and any other variant panics. `Message` and the custom `Result` enum demonstrate usage.

## OCaml Approach

OCaml's `alcotest` provides `check (module Message) "msg" expected actual` with custom testable modules. `assert_failure "message"` in OUnit2 handles failure. For pattern-based tests, OCaml's `match ... with _ -> assert false` is the equivalent of `unwrap_variant!`. OCaml's pattern matching is a language feature, so tests are often just `let Message.Text s = msg in assert_string s`.

## Key Differences

1. **Std stabilization**: Rust's `assert_matches!` is now in `std` (1.73+); OCaml's test assertions are always library-based.
2. **Pattern syntax**: Rust's `assert_matches!($v, Ok(x) if x > 0)` supports guard conditions; OCaml's inline pattern matching does too.
3. **Error messages**: Rust's custom `assert_matches!` can include the actual value; OCaml's pattern match failures show the match expression location.
4. **Extraction**: Rust's `unwrap_variant!` extracts the inner value; OCaml's `let Pattern x = val` achieves the same with more concise syntax.

## Exercises

1. **Nested variant assertion**: Extend `assert_matches!` to support nested patterns: `assert_matches!(response, Response::Ok(Body::Json(json)) if json.contains("id"))`. Verify it produces clear failure messages showing the actual value.
2. **Result helpers**: Create `assert_ok!(result)` and `assert_err!(result)` macros that assert the appropriate variant and return the inner value. Also create `assert_ok_eq!(result, expected)` that combines the assertion with value equality checking.
3. **Collection variant test**: Implement `assert_all_match!(items, $pattern)` that asserts every element in a `Vec` matches the pattern, reporting the index of the first non-matching element.

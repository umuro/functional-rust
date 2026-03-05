📖 **[View on hightechmind.io →](https://hightechmind.io/rust/755-testing-error-paths)**

---

# 755: Testing Error Cases and Unwrap Discipline

**Difficulty:** 2  **Level:** Intermediate

Test every error variant explicitly — not just `is_err()`, but *which* error and *what values* it carries.

## The Problem This Solves

Most developers test the happy path thoroughly and add one or two `assert!(result.is_err())` checks for errors. This leaves the error logic essentially untested. Did you return the *right* error? Does it contain the right position, the right value, the right message? You won't know until a user reports a confusing error message in production.

Rich error types carry data. `ParseError::InvalidChar { ch: 'x', pos: 2 }` is far more useful than `ParseError::Invalid`. But only if you test that `ch` is `'x'` and `pos` is `2` — not just that an error occurred. If you refactor the parser and the position becomes `3` instead of `2`, your tests should catch it.

There's also `unwrap()` discipline to consider. Using `.unwrap()` in tests is fine when the value *should* be `Ok` — a failing `.unwrap()` gives you a clear panic with the actual value. But use `.expect("descriptive message")` for better diagnostics. Avoid using `.unwrap()` in business logic; prefer `.expect()` or `?`.

## The Intuition

In Python, you'd use `pytest.raises(ValueError) as exc_info` and then inspect `exc_info.value.args`. In Rust, you pattern-match on the error variant and destructure its fields — the compiler ensures you check all the data the error carries.

The `match` pattern `Err(ParseError::TooLong { len: 11, max: 10 }) => {}` reads naturally: "I expect a TooLong error where len is 11 and max is 10". If the implementation changes the max to 9, the test fails.

## How It Works in Rust

```rust
// Rich error enum with data in each variant
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    Empty,
    TooLong { len: usize, max: usize },
    InvalidChar { ch: char, pos: usize },
    OutOfRange { value: i64, min: i64, max: i64 },
}

// Testing happy path — use expect() for clear failure messages
#[test]
fn parse_valid_number() {
    let n = parse_positive("42").expect("'42' is a valid positive integer");
    assert_eq!(n, 42);
}

// Testing error variant and its fields
#[test]
fn parse_too_long_returns_correct_lengths() {
    let long = "1".repeat(11);
    match parse_positive(&long) {
        Err(ParseError::TooLong { len: 11, max: 10 }) => {}  // exactly right
        other => panic!("expected TooLong(11, 10), got {:?}", other),
    }
}

// Testing error position — char at index 2 is invalid
#[test]
fn parse_invalid_char_reports_position() {
    match parse_positive("12x45") {
        Err(ParseError::InvalidChar { ch: 'x', pos: 2 }) => {}
        other => panic!("expected InvalidChar('x', 2), got {:?}", other),
    }
}

// Using `..` to match only fields you care about
#[test]
fn parse_zero_is_out_of_range() {
    match parse_positive("0") {
        Err(ParseError::OutOfRange { value: 0, .. }) => {}
        other => panic!("expected OutOfRange(0), got {:?}", other),
    }
}

// assert_eq! works when error type derives PartialEq
#[test]
fn empty_input_is_empty_error() {
    assert_eq!(parse_positive(""), Err(ParseError::Empty));
}

// #[should_panic] for testing that unwrap panics
#[test]
#[should_panic(expected = "called `Result::unwrap()`")]
fn unwrap_on_err_panics() {
    parse_positive("").unwrap();
}

// Graceful defaults in production code
#[test]
fn unwrap_or_else_for_defaults() {
    let n = parse_positive("bad").unwrap_or_else(|_| 0);
    assert_eq!(n, 0);
}
```

Key points:
- Derive `PartialEq` on error types to use `assert_eq!` directly
- Use `match` + `panic!` to check specific field values with a helpful message
- `..` in patterns ignores fields you don't need to check
- `.expect("message")` in tests is better than `.unwrap()` — shows what you expected
- Test *every* error variant, not just `is_err()` — otherwise refactors can silently change error types

## What This Unlocks

- **Confident error API**: every error variant is covered by at least one test — users get accurate error messages
- **Safe refactoring**: changing an error variant causes test failures, not silent behavior changes
- **Better error design feedback**: if your errors are hard to test, they're probably hard for callers to handle too

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Error type | Polymorphic variants or exceptions | `enum` with data per variant |
| Matching errors in tests | `match result with | Error X -> ...` | `match result { Err(MyError::X { field }) => ... }` |
| Equality check | `(=)` operator or custom | Derive `PartialEq` on error enum |
| Expected panic | `assert_raises` | `#[should_panic(expected = "...")]` |
| Unwrap in tests | `Option.get` / `Result.get_ok` | `.unwrap()` or `.expect("msg")` |
| Partial field match | N/A | `..` pattern ignores remaining fields |

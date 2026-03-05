📖 **[View on hightechmind.io →](https://hightechmind.io/rust/319-error-in-tests)**

---

# 319: Error Handling in Tests

**Difficulty:** 1  **Level:** Beginner

Write clean, readable tests for fallible code — including the failure cases.

## The Problem This Solves

You have a function that returns `Result<T, E>`. Testing both the success and failure paths requires different approaches: success tests want to extract the value and check it, failure tests want to assert on the error variant, and some tests need to verify that code panics. Writing all of these with manual `match` expressions is verbose and obscures what's actually being tested.

Rust's test framework handles this well — but you need to know the idioms. Result-returning test functions (`fn test() -> Result<(), E>`) let you use `?` freely inside tests. `assert_eq!` on `Result` values checks equality. `#[should_panic]` marks tests that are expected to panic. And `unwrap_err()` extracts the error for inspection without a verbose match.

Testing error paths thoroughly is what separates production-quality code from prototype code. Error handling that's never been tested is error handling that doesn't work when you need it.

## The Intuition

Test functions can return `Result<(), E>` — use `?` freely for the happy path, and `unwrap_err()` / `assert_eq!` for the error path.

## How It Works in Rust

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Pattern 1: Result-returning test — use ? freely
    #[test]
    fn div_ok() -> Result<(), MathError> {
        assert_eq!(safe_div(10, 2)?, 5);  // ? fails the test if Err
        Ok(())  // must return Ok(()) at the end
    }

    // Pattern 2: Assert on specific error value
    #[test]
    fn div_zero() {
        assert_eq!(safe_div(5, 0), Err(MathError::DivisionByZero));
    }

    // Pattern 3: Check error without equality — matches! macro
    #[test]
    fn div_zero_variant() {
        assert!(matches!(safe_div(5, 0), Err(MathError::DivisionByZero)));
    }

    // Pattern 4: Inspect the error value
    #[test]
    fn sqrt_neg_message() {
        let err = safe_sqrt(-9).unwrap_err();  // unwrap_err() panics if Ok
        assert_eq!(err, MathError::NegativeInput(-9));
    }

    // Pattern 5: Test that a function panics
    #[test]
    #[should_panic]           // test passes if the body panics
    fn panics_on_unwrap() {
        safe_div(1, 0).unwrap();  // this panics
    }

    // Pattern 6: #[should_panic(expected = "...")] — verify the panic message
    #[test]
    #[should_panic(expected = "division by zero")]
    fn panics_with_message() {
        panic!("division by zero");
    }
}
```

The `?` operator in tests converts an error into a test failure with the error's `Debug` output. It's cleaner than `unwrap()` because the test failure message tells you *which* error occurred, not just "called unwrap on an Err value."

## What This Unlocks

- **Clean happy-path tests** — `fn test() -> Result<(), E>` with `?` reads like production code, not test boilerplate
- **Complete error coverage** — `#[should_panic]`, `unwrap_err()`, and `assert_eq!` on `Err` cover all the ways things can go wrong
- **Documented panic contracts** — `#[should_panic]` tests serve as documentation: "calling this with X is a programmer error that panics"

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Assert equality | `assert_equal` (OUnit) | `assert_eq!(actual, expected)` |
| Expected error | `assert_raises` | `assert_eq!(f(), Err(e))` or `#[should_panic]` |
| Result in tests | Manual `match` | `fn test() -> Result<(), E>` — use `?` directly |
| Inspect error | Manual `match` | `result.unwrap_err()` — panics if `Ok` |

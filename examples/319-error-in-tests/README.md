📖 **[View on hightechmind.io →](https://hightechmind.io/rust/319-error-in-tests)**

---

# 319: Error Handling in Tests
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Tests that call fallible functions traditionally use `unwrap()`, which panics with an unhelpful message on failure. Rust test functions can return `Result<(), E>`, enabling `?` to propagate errors with full context. Additionally, `#[should_panic(expected = "...")]` attributes test that specific panics occur — completing the testing toolkit for both `Result`-returning and panic-producing code.

## Learning Outcomes

- Write test functions returning `Result<(), E>` to use `?` for clean error propagation
- Use `#[should_panic(expected = "message")]` to test expected panic behavior
- Use `assert_eq!` / `assert!` inside `Result`-returning tests for mixed assertions
- Recognize that returning `Err` from a test function causes a clean test failure with the error message

## Rust Application

Tests returning `Result` get clean error messages; `#[should_panic]` tests expected panics:

```rust
#[test]
fn test_safe_div_result() -> Result<(), MathError> {
    let result = safe_div(10, 2)?;  // ? propagates MathError on failure
    assert_eq!(result, 5);
    Ok(())
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_div_panics() {
    let _ = 1 / 0;  // panics with "attempt to divide by zero"
}

#[test]
fn test_error_variant() {
    let result = safe_div(10, 0);
    assert_eq!(result, Err(MathError::DivisionByZero));
}
```

## OCaml Approach

OCaml testing with `Alcotest` uses `Alcotest.check` for assertions and `Alcotest.check_raises` for expected exceptions. Test functions return `unit` and raise `Alcotest.Test_error` on failure:

```ocaml
let test_safe_div () =
  Alcotest.(check int) "five" 5 (safe_div 10 2);
  Alcotest.check_raises "div by zero" Division_by_zero (fun () -> safe_div 1 0)
```

## Key Differences

1. **Test return type**: Rust test functions can return `Result<(), E>` — the `?` operator works naturally inside them; OCaml tests return `unit`.
2. **Failure message**: Rust test failure from `Err(e)` displays `format!("{:?}", e)`; OCaml's Alcotest shows the exception message.
3. **Expected panic**: `#[should_panic]` is a compile-time annotation; OCaml's `check_raises` is a runtime assertion.
4. **Integration**: `Result`-returning tests integrate with `?` and all `Result` combinators — tests read like production code.

## Exercises

1. Write a test that uses `?` to call three fallible operations in sequence, failing with a descriptive error if any step fails.
2. Add `#[should_panic(expected = "invariant violated")]` tests for functions that use `assert!` to enforce preconditions.
3. Write a test that captures the `Err` value from a failing operation and uses `assert_eq!` on the error variant — verifying both that it failed and how it failed.

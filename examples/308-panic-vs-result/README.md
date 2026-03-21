📖 **[View on hightechmind.io →](https://hightechmind.io/rust/308-panic-vs-result)**

---

# 308: When to Panic vs Return Result
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

The choice between `panic!()` and returning `Result` is architectural. Panic for programming errors (bugs the developer should fix); return `Result` for operational errors (bad user input, network failures, missing files). Getting this wrong creates brittle APIs that crash on recoverable failures, or obscure programmer errors behind error types that callers don't know how to handle. This distinction maps to OCaml's choice between exceptions and `Result`.

## Learning Outcomes

- Distinguish recoverable errors (return `Result`) from programming bugs (use `panic!` or `assert!`)
- Use `assert!()` and `assert_eq!()` to document and enforce invariants
- Return `Result` from library functions that receive user-controlled input
- Panic for impossible states that indicate a bug: violated post-conditions, index out of bounds on internal data

## Rust Application

The rule: `Result` for callers who can handle failures, `panic` for bugs that should never happen:

```rust
// Library function: user provides input — use Result
pub fn parse_age(s: &str) -> Result<u8, String> {
    let n: i32 = s.parse().map_err(|_| format!("'{}' is not a number", s))?;
    if n < 0 || n > 150 {
        return Err(format!("age {} is out of range [0, 150]", n));
    }
    Ok(n as u8)
}

// Internal invariant that must always hold — panic is appropriate
pub fn divide(a: i32, b: i32) -> i32 {
    assert!(b != 0, "divide: b must not be zero");
    a / b
}
```

## OCaml Approach

OCaml uses exceptions for panic-equivalent cases and `Result`/`Option` for expected failures:

```ocaml
(* User input: use Result *)
let parse_age s =
  match int_of_string_opt s with
  | None -> Error (Printf.sprintf "'%s' is not a number" s)
  | Some n when n < 0 || n > 150 -> Error "age out of range"
  | Some n -> Ok n

(* Programmer invariant: raise Invalid_argument *)
let divide a b =
  if b = 0 then raise (Invalid_argument "divide by zero")
  else a / b
```

## Key Differences

1. **Panic = unrecoverable**: Rust's `panic!` unwinds the thread; OCaml exceptions are catchable with `try/with` — Rust panics can be caught with `catch_unwind` but this is uncommon.
2. **API contract**: `panic` documents "this is a bug" in the library contract; `Result` documents "this can fail at runtime".
3. **Testing**: `#[should_panic]` tests that code panics on contract violations; `assert_eq!` tests that Results contain expected values.
4. **Infallible**: `std::convert::Infallible` and the `!` type represent operations that cannot fail — the type system enforces it.

## Exercises

1. Write a `safe_sqrt(x: f64) -> Result<f64, String>` and a `sqrt_positive(x: f64) -> f64` (panics on negative) — document when each is appropriate.
2. Refactor a function that currently panics on invalid input to return `Result`, and show that callers must now handle the error.
3. Add `assert!` precondition checks to an internal function, then write a test with `#[should_panic]` that verifies the assertion fires on invalid input.

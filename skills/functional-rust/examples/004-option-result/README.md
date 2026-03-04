# 004: Option and Result

**Difficulty:** ⭐⭐  **Level:** Intermediate

Replace `null` crashes and uncaught exceptions with types the compiler forces you to handle.

## The Problem This Solves

Every programmer has seen `NullPointerException`, `TypeError: Cannot read property of undefined`, or `AttributeError: 'NoneType' object`. These happen when code assumes a value exists, but it doesn't. The fix is always the same: check first. But it's easy to forget, the check is often far from the code that fails, and tests don't always catch it.

Rust has no `null`. Instead, it has two types:
- **`Option<T>`** — for values that might or might not exist (`Some(x)` or `None`)
- **`Result<T, E>`** — for operations that might fail (`Ok(x)` or `Err(e)`)

The key difference from other languages: you *can't use* an `Option<T>` as if it were a `T`. The compiler refuses. You must handle the `None` case explicitly before you can access the value inside.

## The Intuition

```python
# Python — None is a valid value anywhere, crashes at runtime
def safe_div(a, b):
    if b == 0:
        return None     # caller might forget to check this!
    return a / b

result = safe_div(10, 0)
print(result + 1)    # AttributeError: NoneType — runtime crash
```

```rust
// Rust — Option forces you to check at compile time
fn safe_div(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

let result = safe_div(10.0, 0.0);
println!("{}", result + 1.0);   // COMPILE ERROR: can't use Option<f64> as f64
```

The Rust version catches the bug before the program ever runs.

## How It Works in Rust

**Option — for values that might not exist:**

```rust
// Safe division
fn safe_div(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

// Using Option:
match safe_div(10.0, 2.0) {
    Some(result) => println!("{}", result),   // 5.0
    None         => println!("Division by zero"),
}

// Chaining with .map() — transform the value if it exists
let doubled = safe_div(10.0, 2.0).map(|x| x * 2.0);  // Some(10.0)
let nothing = safe_div(10.0, 0.0).map(|x| x * 2.0);  // None — map skips

// Default value
let result = safe_div(10.0, 0.0).unwrap_or(0.0);  // 0.0
```

**Result — for operations that fail with a reason:**

```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

fn checked_div(a: i64, b: i64) -> Result<i64, MathError> {
    if b == 0 { Err(MathError::DivisionByZero) } else { Ok(a / b) }
}

fn checked_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 { Err(MathError::NegativeSquareRoot) } else { Ok(x.sqrt()) }
}
```

**The `?` operator — chain fallible operations cleanly:**

```rust
// Compute sqrt(a / b) — two things that can fail
fn sqrt_of_division(a: f64, b: f64) -> Result<f64, MathError> {
    let quotient = safe_div(a, b).ok_or(MathError::DivisionByZero)?;
    //                                                               ^
    //             If this is Err, return immediately with that error
    checked_sqrt(quotient)
    // If this is Err, return that error too
}
```

`?` is Rust's "propagate error upward" operator. It replaces chains of `if err != nil { return err }` in Go, or try/catch in Java. Errors flow up automatically, and the code reads like the happy path.

## What This Unlocks

- **No null pointer exceptions** — ever. If a value might be absent, the type says so.
- **Explicit error handling** — every failure mode is named and handled; no silent swallowing
- **The `?` operator in practice** — most real Rust functions that do I/O, parsing, or network calls return `Result` and chain with `?`

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| "Maybe a value" | `'a option` | `Option<T>` |
| Success/failure | `('a, 'b) result` | `Result<T, E>` |
| Chaining Options | `Option.map`, `Option.bind` | `.map()`, `.and_then()` |
| Chaining Results | `Result.bind`, `\|>` | `.map()`, `.and_then()`, `?` |
| Propagate error | Manual match or `Result.bind` | `?` operator |
| Force unwrap | `Option.get` (raises) | `.unwrap()` (panics) |

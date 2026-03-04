# 045: Result Basics

**Difficulty:** 1  **Level:** Foundations

Understand `Result<T, E>` — Rust's way of making errors visible, explicit, and impossible to ignore.

## The Problem This Solves

You're writing a function that divides two numbers. In Python you'd write `a / b` and maybe wrap it in `try/except ZeroDivisionError`. But here's the thing: nothing in Python's type system tells you that function can fail. A caller reading your code has no way to know — they have to read the docs, or discover it at runtime when the exception bubbles up.

This is the "hidden failure" problem. In large codebases with many layers, exceptions silently skip the call stack. A Java method marked `throws SomeException` is closer to the truth, but still opt-in — the compiler won't stop you from ignoring it. You can always just… not catch it.

Rust takes a different approach. A function that can fail returns `Result<T, E>`. If it returns `Ok(value)`, the call succeeded. If it returns `Err(error)`, it failed. The caller **must** decide what to do — the compiler enforces it. You literally cannot accidentally ignore an error. This isn't a style preference; it's baked into the type system.

## The Intuition

Think about Python's `try/except`. It works, but the failure mode is invisible: you call a function, have no idea if it throws, and find out at runtime. JavaScript's Promises are better — `.then()` makes the async path explicit — but regular functions still throw silently.

In Rust, `Result<i64, MathError>` in the return type is a **contract**: "this function either gives you an `i64`, or it gives you a `MathError`. Handle both." The key insight: **in Rust, the type signature tells you whether a function can fail — no surprises.**

You match on it just like a Python `if/else`, but the compiler won't let you forget the error case:

```rust
match safe_div(10, 0) {
    Ok(value) => println!("Got: {}", value),
    Err(e)    => println!("Failed: {:?}", e),
}
```

## How It Works in Rust

```rust
// Define what errors your function can produce
#[derive(Debug, PartialEq)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

// Return type says: "this can fail with MathError"
fn safe_div(a: i64, b: i64) -> Result<i64, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)   // wrap error in Err(...)
    } else {
        Ok(a / b)                         // wrap success in Ok(...)
    }
}

// Pattern match to handle both cases
match safe_div(10, 2) {
    Ok(v)  => println!("{}", v),          // prints 5
    Err(e) => println!("{:?}", e),
}

// Utility methods for quick access
safe_div(10, 2).unwrap_or(0);            // returns 5, or 0 if Err
safe_div(10, 0).is_err();                // true
safe_div(10, 2).ok();                    // Some(5) — converts to Option
```

## What This Unlocks

- **Safe parsing:** `"42".parse::<i64>()` returns `Result<i64, ParseIntError>` — parse failures are handled, not hoped for.
- **I/O operations:** File reads, network calls, database queries — all return `Result` in Rust, so failures are always explicit.
- **Library design:** When you publish a library, callers see exactly which functions can fail from the type signatures alone — no documentation required.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Success value | `Ok value` | `Ok(value)` |
| Failure value | `Error e` | `Err(e)` |
| Pattern match | `match r with Ok v -> ... \| Error e -> ...` | `match r { Ok(v) => ..., Err(e) => ... }` |
| Error type | Any type, often polymorphic `('a, 'b) result` | Must be concrete: `Result<T, E>` |
| Ignoring errors | Compiler warns, but possible | Compiler **error** (unused `Result` triggers warning that's hard to ignore) |
| Convert to Option | `Result.to_option` | `.ok()` |
| Default on error | `Result.value ~default` | `.unwrap_or(default)` |

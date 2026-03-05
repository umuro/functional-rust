# 291: Result Combinators

**Difficulty:** 2  **Level:** Intermediate

Transform, chain, and recover from errors using `.map()`, `.and_then()`, and `.or_else()` — without writing `match` everywhere.

## The Problem This Solves

You've just learned that `Result<T, E>` is how Rust handles errors. You write your first error-handling code and it looks like this:

```rust
match parse_int("10") {
    Ok(n) => match divide(n, 2) {
        Ok(result) => match something_else(result) {
            Ok(final) => println!("{}", final),
            Err(e)    => println!("Error: {}", e),
        },
        Err(e) => println!("Error: {}", e),
    },
    Err(e) => println!("Error: {}", e),
}
```

This is called "pyramid of doom" — three operations, three levels of nesting. Every new step doubles the indentation. In real code with five or six steps, this becomes unreadable.

The solution is Result's built-in combinators: methods that let you work *inside* a `Result` without unwrapping it. They short-circuit automatically — the first `Err` stops the chain and passes the error to the end. Your happy path reads like a straight line.

## The Intuition

If you know JavaScript promises, this will feel familiar:
```js
fetch(url)
  .then(response => response.json())   // .map() or .and_then()
  .then(data => process(data))
  .catch(err => defaultValue)           // .unwrap_or()
```

In Python, you might use a try/except block for every step. In Rust, you use the combinators:

- **`.map(f)`** — if `Ok(x)`, run `f(x)` and return `Ok(result)`; if `Err`, skip and pass the error through
- **`.and_then(f)`** — like `.map()` but `f` itself returns a `Result`; used when the next step can also fail
- **`.map_err(f)`** — transform the *error* value (e.g., add context to error messages)
- **`.or_else(f)`** — if `Err`, try `f` as a fallback; if `Ok`, skip `f`

## How It Works in Rust

```rust
fn parse_int(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| format!("parse error: {}", e))
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err("division by zero".to_string()) }
    else { Ok(a / b) }
}

// .map(): transform the Ok value — if Err, it passes through unchanged
let doubled: Result<i32, String> = Ok(5).map(|x| x * 2);
// Ok(10)

// .and_then(): chain a fallible operation
// (if parse_int fails, divide never runs)
let result = parse_int("10").and_then(|n| divide(n, 2));
// Ok(5)

let short = parse_int("abc").and_then(|n| divide(n, 2));
// Err("parse error: ...") — divide never called

// .map_err(): rewrite the error message
let rich = "bad".parse::<i32>()
    .map_err(|e| format!("Validation failed: {}", e));
// Err("Validation failed: invalid digit found in string")

// .or_else(): provide a fallback when there's an error
let recovered = parse_int("bad").or_else(|_| Ok(42));
// Ok(42)

// Full pipeline — reads like a recipe
let result = parse_int("20")
    .and_then(|n| divide(n, 4))   // 20 / 4 = 5
    .map(|n| n + 1)                // 5 + 1 = 6
    .map_err(|e| format!("Pipeline failed: {}", e));
// Ok(6)
```

Think of `and_then` as "and if that worked, then do this." If any step returns `Err`, the rest of the chain is skipped — just like a short-circuit `&&` for operations.

## What This Unlocks

- **HTTP request pipelines** — parse response → validate status → deserialize JSON → extract field, all in one clean chain that stops at the first failure
- **Form validation** — parse string → check range → apply business rules, returning a specific error for each failure mode
- **Configuration loading** — try environment variable → try config file → fall back to default, using `.or_else()` at each step

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Map Ok value | `Result.map f r` | `r.map(f)` |
| Chain fallible ops | `Result.bind r f` | `r.and_then(f)` |
| Map error value | `Result.map_error f r` | `r.map_err(f)` |
| Fallback on error | Custom `match` | `r.or_else(f)` |
| Unwrap with default | `Result.value ~default r` | `r.unwrap_or(default)` |

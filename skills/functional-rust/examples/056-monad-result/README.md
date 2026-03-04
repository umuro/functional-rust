# 056: Result Monad

**Difficulty:** ⭐⭐  **Level:** Intermediate

Chain fallible operations on `Result` — getting typed error information at every failure point, with zero nesting.

## The Problem This Solves

You have a sequence of steps where each can fail, and you care *why* it failed, not just *that* it failed. Maybe you're parsing user input, then validating it, then processing it. Each step uses `Result<T, E>` to carry an error message or error type when it goes wrong.

The naive version is a cascade of matches:

```rust
// Without and_then: nesting grows with each step
fn validate_input(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Err(_) => Err(format!("Not an integer: {}", s)),
        Ok(n) => {
            if n <= 0 {
                Err(format!("Not positive: {}", n))
            } else {
                match n % 2 {
                    1 => Err(format!("Not even: {}", n)),
                    _ => Ok(n),
                }
            }
        }
    }
}
```

Every step adds another level of indentation. The happy path runs down the rightmost column. Error handling dominates the structure. If you add a fourth validation step, you nest again.

The real pain: `Err` propagates identically through every step — if any step fails, the whole chain stops and carries that error forward. You're writing the same pattern repeatedly to express a simple idea: "do this, then this, then this, and if anything fails, stop and tell me why."

The Result monad exists to solve exactly that pain.

## The Intuition

Think of a quality control line in a factory. Raw material goes in, finished product (or rejection slip) comes out. Each inspector either passes the item along or stamps it with a specific reason for rejection and sends it to the reject bin. Once rejected, no further inspectors touch it — the rejection slip travels all the way to the end unchanged.

`Result` is that quality control line. `Ok(value)` is "passes inspection." `Err(reason)` is the rejection slip. Once you get `Err`, every subsequent step is skipped and that exact error comes out the end.

`and_then` connects inspectors: "if the item passed the previous station, pass it to this function; otherwise, let the rejection slip through unchanged."

```rust
// Three inspectors, connected
parse_int(s)              // Station 1: is it a number?
    .and_then(check_positive)  // Station 2: is it positive?
    .and_then(check_even)      // Station 3: is it even?
```

If station 1 fails with `Err("Not an integer: foo")`, stations 2 and 3 never run. The error arrives at the end exactly as produced. This is a **monad**: a pattern for chaining operations that carry failure context, without nesting. And again — `?` is the same thing written as early return.

## How It Works in Rust

**Three validators, each returning Result**

```rust
fn parse_int(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|_| format!("Not an integer: {}", s))
    // map_err converts parse's error type into our String error
}

fn check_positive(n: i32) -> Result<i32, String> {
    if n > 0 { Ok(n) } else { Err(format!("Not positive: {}", n)) }
}

fn check_even(n: i32) -> Result<i32, String> {
    if n % 2 == 0 { Ok(n) } else { Err(format!("Not even: {}", n)) }
}
```

**Chain them with `and_then`**

```rust
fn validate_input(s: &str) -> Result<i32, String> {
    parse_int(s)
        .and_then(check_positive)  // only runs if parse_int returned Ok
        .and_then(check_even)      // only runs if check_positive returned Ok
}

validate_input("42")     // Ok(42)
validate_input("hello")  // Err("Not an integer: hello")
validate_input("-4")     // Err("Not positive: -4")
validate_input("7")      // Err("Not even: 7")
```

**The same chain with `?`**

```rust
fn validate_input(s: &str) -> Result<i32, String> {
    let n = parse_int(s)?;      // returns Err early if this fails
    let n = check_positive(n)?; // returns Err early if this fails
    let n = check_even(n)?;     // returns Err early if this fails
    Ok(n)
}
```

Identical behavior, different style. `?` is cleaner when steps have names; `.and_then()` is cleaner for one-liners or lambdas.

**Typed errors (the idiomatic Rust upgrade)**

When you want the caller to distinguish error cases at compile time, use an enum instead of `String`:

```rust
#[derive(Debug, PartialEq)]
enum ValidationError {
    ParseError(String),
    NotPositive(i32),
    NotEven(i32),
}

fn validate_typed(s: &str) -> Result<i32, ValidationError> {
    let n = parse_int_typed(s)?;       // Err(ValidationError::ParseError(...))
    let n = check_positive_typed(n)?;  // Err(ValidationError::NotPositive(...))
    check_even_typed(n)                // Err(ValidationError::NotEven(...))
}
```

Now the caller can `match` on the exact variant — no string parsing needed to figure out what went wrong.

## What This Unlocks

- **Error information without nesting.** Every failure carries a typed reason, but you write the logic as a flat chain. The structure reflects your business logic, not your error handling.
- **The `?` operator everywhere.** Understanding `and_then` as monadic bind explains why `?` composes so well: every `?` is just an `and_then` step that returns early. You can stack them freely and the types guide you.
- **Custom error types with `From`.** When your chain mixes different error types, Rust's `From` trait auto-converts them via `?`. The monad chain stays flat even as error diversity grows.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Monadic bind | `Result.bind` / `>>=` operator | `Result::and_then` |
| Do-notation sugar | Not built in | `?` operator |
| Error types | Strings or polymorphic variants common | Custom error enums idiomatic |
| Error conversion | Explicit mapping required | `From` trait + `?` auto-converts |
| Short-circuit | `Err` propagates through bind | `Err` propagates through `and_then` / `?` |

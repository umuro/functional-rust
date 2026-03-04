# 259: Result Monad — Error Chaining

**Difficulty:** 2  **Level:** Intermediate

Chain validation steps so the first failure short-circuits with a descriptive error — railway-oriented programming.

## The Problem This Solves

Input validation often has multiple steps: parse the string as an integer, check it's positive, check it's even. Each step can succeed or fail with a different error message. Nesting `match` expressions for each step creates a pyramid of indentation and obscures the happy path.

Railway-oriented programming offers a cleaner mental model: there are two tracks, the success track and the error track. Each validation step is a switch. If the value on the success track passes the check, it continues forward. If it fails, it's diverted to the error track. Once on the error track, all remaining steps are skipped automatically — the error just travels to the end.

`Result<T, E>` is Rust's railway: `Ok(T)` is on the success track, `Err(E)` is on the error track. `and_then` is the switch. The `?` operator is syntactic sugar for the same switching, letting you write the happy path sequentially.

## The Intuition

Think of `Result` as a train on two parallel tracks. `Ok(value)` means the train is on the success track carrying `value`. `Err(msg)` means the train has switched to the error track carrying `msg`.

`and_then` says: "if on success track, apply this function; if on error track, stay there and skip". Chain three `and_then` calls and you get three switches — each one either continues the journey or terminates it with an error.

The `?` operator makes this invisible: `f()?` means "if `Err`, return it immediately from this function; if `Ok(x)`, give me `x`". Writing three `?`-terminated calls in sequence gives you the railway structure with imperative readability.

## How It Works in Rust

```rust
pub fn parse_int(s: &str) -> Result<i64, String> {
    s.parse::<i64>().map_err(|_| format!("Not an integer: {s}"))
}

pub fn check_positive(n: i64) -> Result<i64, String> {
    if n > 0 { Ok(n) } else { Err("Must be positive".to_string()) }
}

pub fn check_even(n: i64) -> Result<i64, String> {
    if n % 2 == 0 { Ok(n) } else { Err("Must be even".to_string()) }
}

// Style 1: and_then chain — mirrors OCaml's >>= operator exactly
pub fn validate(s: &str) -> Result<i64, String> {
    parse_int(s).and_then(check_positive).and_then(check_even)
}

// Style 2: ? operator — monadic sequencing with imperative style
pub fn validate_q(s: &str) -> Result<i64, String> {
    let n = parse_int(s)?;       // Err propagates immediately
    let n = check_positive(n)?;  // skipped if previous returned Err
    let n = check_even(n)?;
    Ok(n)
}

// Style 3: explicit bind — shows what and_then desugars to
fn bind<T, U, E>(r: Result<T, E>, f: impl FnOnce(T) -> Result<U, E>) -> Result<U, E> {
    match r {
        Err(e) => Err(e),  // error track: skip f, propagate error
        Ok(v) => f(v),     // success track: apply f
    }
}
```

`map_err` converts the `ParseIntError` from `str::parse` into our uniform `String` error type — a necessary step when combining functions with different error types.

## What This Unlocks

- **Multi-step form validation** — parse, sanitise, range-check, business-rule-check as a chain; first failure returns immediately with its message.
- **Database query pipelines** — parse → lookup → authorise → transform as `Result` chain; the `?` makes it read like sequential code but handles errors automatically.
- **Error type conversion** — `map_err` + `?` together form the standard pattern for converting between error types across library boundaries.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Bind operator | Custom `>>=` infix on `result` | `.and_then()` method — no custom operator |
| `?` equivalent | None | `?` desugars to early `return Err(e)` |
| Error conversion | String concatenation | `map_err(\|_\| ...)` converts error types |
| Error type | `string` for all steps | Generic `E` — same type across the chain |
| Syntax sugar | `let*` (bind notation, OCaml 4.08+) | `?` — idiomatic in all Rust code |

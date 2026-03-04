# 047: Result Bind

**Difficulty:** 1  **Level:** Foundations

Chain multiple fallible operations cleanly — stop at the first failure, with zero boilerplate.

## The Problem This Solves

Real programs rarely do one thing. You parse input, divide it, check a range, verify a constraint — four steps, any of which can fail. In Python you'd write four `try/except` blocks or deeply nested `if err != None` checks. The signal (your actual business logic) gets buried in the noise (error handling).

In JavaScript, Promise chaining solves this for async code: `.then().then().then()` with a single `.catch()` at the end. Each step gets the previous step's value; if any step rejects, the rest are skipped. It's clean.

`and_then` (also called `bind` or `flatMap` in functional programming) brings that same clarity to synchronous Rust code. Each step in the chain only runs if the previous step succeeded. The first `Err` short-circuits everything. Your business logic reads top-to-bottom without nesting.

## The Intuition

`and_then` is the answer to: "what if the *next* operation can also fail?" Compare:

- `map(f)` — f is `T → U`, cannot fail
- `and_then(f)` — f is `T → Result<U, E>`, can fail

Think of `and_then` as "if Ok, try this next thing." If it also succeeds, keep going. If it fails, stop and return that error. The chain collapses the moment anything goes wrong — no nested match, no intermediate variable checking.

The `?` operator is syntactic sugar for `and_then` + early return. Both styles produce identical behavior; `?` just reads more like imperative code.

## How It Works in Rust

```rust
fn parse_int(s: &str) -> Result<i64, Error> { ... }
fn safe_div(a: i64, b: i64) -> Result<i64, Error> { ... }
fn check_positive(x: i64) -> Result<i64, Error> { ... }
fn check_small(x: i64) -> Result<i64, Error> { ... }

// Style 1: and_then chain
// Each step only runs if the previous returned Ok
fn pipeline(input: &str, divisor: i64) -> Result<i64, Error> {
    parse_int(input)
        .and_then(|x| safe_div(x, divisor))  // only runs if parse succeeded
        .and_then(check_positive)              // only runs if div succeeded
        .and_then(check_small)                 // only runs if positive check passed
}

// Style 2: ? operator — equivalent, reads like normal imperative code
fn pipeline_question(input: &str, divisor: i64) -> Result<i64, Error> {
    let x = parse_int(input)?;    // if Err: return that Err immediately
    let y = safe_div(x, divisor)?;
    let z = check_positive(y)?;
    check_small(z)                 // last step: return its Result directly
}

// Both styles produce IDENTICAL results:
pipeline("100", 5)    // Ok(20) — 100/5=20, positive, small
pipeline("100", 0)    // Err(DivByZero) — stops at step 2
pipeline("-100", 5)   // Err(Negative) — stops at step 3
pipeline("5000", 1)   // Err(TooLarge) — stops at step 4
pipeline("abc", 2)    // Err(Parse(...)) — stops at step 1
```

## What This Unlocks

- **Validation pipelines:** Chain parse → sanitize → validate → save, all as a linear flow. The first failure wins; no need to track error state manually.
- **Database lookup chains:** Look up a user, then their department, then their manager — each lookup can fail with "not found," and `and_then` chains them cleanly.
- **Protocol parsing:** Read a header, then a body, then verify a checksum — structured sequential work where order matters and any step can fail.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Bind operator | `>>=` (infix: `r >>= f`) | `.and_then(f)` (method) |
| ? equivalent | `let* x = r in ...` (OCaml 4.08+) | `let x = r?;` |
| Early exit | `match` or `let*` | `?` returns from current function |
| Error threading | Same error type throughout | Same error type, or use `From` for conversion |
| Function signature | `'a result -> ('a -> 'b result) -> 'b result` | `Result<T,E>` + closure `T -> Result<U,E>` |
| Method vs function | `Result.bind r f` | `r.and_then(f)` |

# 048: Error Propagation

**Difficulty:** 1  **Level:** Foundations

The `?` operator — the most important operator in Rust — propagates errors up the call stack with a single character.

## The Problem This Solves

You're writing a function that calls three other functions, each of which can fail. Without any special syntax, you'd write:

```rust
let a = match parse_int(a_str) {
    Ok(v) => v,
    Err(e) => return Err(e),
};
let b = match parse_int(b_str) {
    Ok(v) => v,
    Err(e) => return Err(e),
};
```

That's 10 lines of boilerplate for two fallible calls. With five steps, you've written 25 lines of match arms that all do the same thing: "if error, return it." The actual logic — what you're computing — is drowned out.

Java's checked exceptions were an attempt to solve this, but callers still have to write `throws` declarations everywhere or wrap everything in `RuntimeException`. Python's exceptions propagate automatically, but that means they're also *invisible* — you can't tell from reading code which calls might throw. Rust finds the middle ground: errors propagate explicitly with `?`, but the propagation is opt-in, visible, and checked by the compiler.

## The Intuition

The `?` operator does exactly one thing: "if this is `Err`, return it from the current function; if it's `Ok`, give me the value." It's not magic — it desugars to a match + early return. But it collapses 5-line match blocks into one character.

Think of it like Python's exception propagation, but *you choose* where it propagates. The `?` is visible in the source code. Every `?` is a place where a function can exit early. You can read a function and know exactly how many ways it can fail.

The function must return `Result` (or `Option`) for `?` to work. That's the contract: if you want to propagate errors, your caller also needs to handle them. The type system enforces the chain.

## How It Works in Rust

```rust
// Implementing From lets ? auto-convert error types
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::Parse(e) }
}

fn parse_int(s: &str) -> Result<i64, AppError> {
    // The inner parse returns ParseIntError; ? converts it to AppError via From
    Ok(s.trim().parse::<i64>()?)
}

// Multi-step computation using ? at each step
fn compute(a_str: &str, b_str: &str) -> Result<i64, AppError> {
    let a = parse_int(a_str)?;        // if Err(AppError::Parse), return immediately
    let b = parse_int(b_str)?;        // same
    let quotient = safe_div(a, b)?;   // if Err(AppError::DivByZero), return immediately
    let validated = validate_range(quotient)?;  // same
    Ok(validated * 2)                 // only reached if all steps succeeded
}

// Compare: WITHOUT ? operator (5 steps, 25 lines)
fn compute_verbose(a_str: &str, b_str: &str) -> Result<i64, AppError> {
    let a = match parse_int(a_str) { Ok(v) => v, Err(e) => return Err(e) };
    let b = match parse_int(b_str) { Ok(v) => v, Err(e) => return Err(e) };
    // ... three more of these
}
// With ?: 5 lines. Without?: 15+ lines. Same behavior.
```

## What This Unlocks

- **Layered error handling:** Write deep call stacks where each layer propagates errors upward. Only the top level (e.g., `main` or an HTTP handler) needs to decide what to *do* with the error.
- **Error enrichment:** At each `?`, you can `.map_err(|e| enrich(e))` to add context before propagating — like Python's `raise X from Y` for exception chaining.
- **Clean library code:** Functions in a library can return specific errors; the application that calls them decides whether to log, retry, or surface them to the user.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Propagate error | `let* x = r in ...` (binding operator) | `let x = r?;` |
| Manual early return | `match r with Error e -> Error e \| Ok v -> ...` | `match r { Err(e) => return Err(e), Ok(v) => ... }` |
| Error type conversion | Manual wrapping or `Result.map_error` | Automatic via `From<E>` trait + `?` |
| Where it works | Any expression | Only in functions returning `Result` or `Option` |
| `main` function | Returns `unit`, handle errors manually | `fn main() -> Result<(), E>` — `?` works in main too |
| Chained context | `Result.map_error (fun e -> ...)` | `.map_err(\|e\| ...)` before `?` |

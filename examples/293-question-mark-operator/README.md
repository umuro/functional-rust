📖 **[View on hightechmind.io →](https://hightechmind.io/rust/293-question-mark-operator)**

---

# 293: The ? Operator

**Difficulty:** 2  **Level:** Intermediate

The single most important operator for readable error handling in Rust — it says "if this failed, return the error immediately."

## The Problem This Solves

You're writing a function that calls three other functions, each of which might fail. Without `?`, you'd write this:

```rust
fn compute(a_str: &str, b_str: &str) -> Result<u32, AppError> {
    match parse_positive(a_str) {
        Ok(a) => match parse_positive(b_str) {
            Ok(b) => match safe_div(a, b) {
                Ok(result) => Ok(result * 2),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
```

This is six lines of boilerplate that all say the same thing: "if it failed, pass the error up." Multiply that by every function in a real application and you have hundreds of lines of noise hiding your actual logic.

The `?` operator collapses all that into a single character. Place `?` after any expression that returns `Result` or `Option`, and Rust does the match for you: if it's `Ok(x)`, unwrap to `x` and continue; if it's `Err(e)`, return `Err(e.into())` immediately. Your function reads like a happy-path-only description of what it does.

## The Intuition

If you've written async JavaScript with `async/await`, you've seen this idea: instead of `.then().then().catch()`, you write straight-line code and errors bubble up automatically via `try/catch`. The `?` operator is Rust's equivalent — but checked at compile time, with no exceptions thrown at runtime.

Think of `?` as a supercharged early return: "try to get this value — if it fails, bail out of the whole function with that error." It's not magic, it's just very convenient shorthand.

## How It Works in Rust

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    DivByZero,
    NegativeInput,
}

// This impl lets ? automatically convert ParseIntError into AppError
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::Parse(e) }
}

fn parse_positive(s: &str) -> Result<u32, AppError> {
    let n: i32 = s.parse()?;  // ? here: if parse fails, return Err(AppError::Parse(...))
                               // The From impl does the conversion automatically
    if n < 0 {
        Err(AppError::NegativeInput)
    } else {
        Ok(n as u32)
    }
}

fn safe_div(a: u32, b: u32) -> Result<u32, AppError> {
    if b == 0 { Err(AppError::DivByZero) } else { Ok(a / b) }
}

// Three ? operators, three potential early returns — reads like straight-line code
fn compute(a_str: &str, b_str: &str) -> Result<u32, AppError> {
    let a = parse_positive(a_str)?;  // if this fails, return immediately
    let b = parse_positive(b_str)?;  // if this fails, return immediately
    let result = safe_div(a, b)?;    // if this fails, return immediately
    Ok(result * 2)                   // only get here if all three succeeded
}

// ? works on Option too
fn find_double(v: &[i32], target: i32) -> Option<i32> {
    let idx = v.iter().position(|&x| x == target)?;  // None if not found
    let val = v.get(idx)?;                             // None if out of bounds
    Some(val * 2)
}
```

What `?` desugars to under the hood:
```rust
// `expr?` expands to roughly:
match expr {
    Ok(val)  => val,
    Err(e)   => return Err(e.into()),  // .into() uses the From trait
}
```

The `.into()` is what makes `?` so flexible: as long as you implement `From<SourceError> for YourError`, `?` handles the conversion automatically.

## What This Unlocks

- **File I/O** — open file, read contents, parse lines, validate format: each step uses `?` and errors float up to the caller naturally
- **HTTP clients** — `let resp = client.get(url).send()?.json::<MyType>()?;` — two fallible operations, two `?`, one line
- **CLI tools** — propagate all errors up to `main()` with `fn main() -> Result<(), Box<dyn Error>>` and use `?` throughout

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Error propagation | `let* x = result in ...` (ppx_let) | `let x = result?;` |
| Desugars to | `Result.bind` | `match + return Err(e.into())` |
| Type conversion | Manual | Automatic via `From` trait |
| On Option | Manual `match` or `Option.bind` | `option?` — returns `None` early |
| Works in closures | Yes | Limited — `?` only works in functions returning `Result`/`Option` |

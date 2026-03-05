📖 **[View on hightechmind.io →](https://hightechmind.io/rust/298-anyhow-pattern)**

---

# 298: The anyhow Pattern — Boxed Errors

**Difficulty:** 3  **Level:** Advanced

Box any error type into a single container — trade type precision for ergonomics.

## The Problem This Solves

You're writing a CLI or application binary. It calls ten different libraries, each with its own error type. Writing a unified enum with `From` impls for all of them is theoretically correct — but also twenty minutes of boilerplate for code you'll never match on. You just want to propagate the error, log it, and exit. The type system is fighting you.

`Box<dyn Error + Send + Sync>` solves this. Any type that implements `Error` can be boxed into it. The `?` operator will do the boxing automatically — no `From` impl needed. You lose the ability to match on specific error variants, but for application code that's often the right trade-off: you don't want to handle a parse error differently from a network error; you want to log both and stop.

The `anyhow` crate packages this pattern with a `Result<T>` type alias, a `.context()` method for adding human-readable context, and a pretty error reporter. This example shows the same pattern using only `std` — so you understand what `anyhow` is actually doing under the hood.

## The Intuition

`Box<dyn Error + Send + Sync>` is a universal error container: any error that implements `Error` can go in, `?` does the boxing, and you get a clean propagation path without writing a single `From` impl.

## How It Works in Rust

```rust
// Type alias — this is essentially what anyhow::Result<T> is
type AnyResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

fn parse_port(s: &str) -> AnyResult<u16> {
    let n: u16 = s.parse()?;  // ParseIntError gets boxed automatically — no From impl needed
    if n == 0 { return Err("port cannot be zero".into()); }  // &str -> Box<dyn Error> via .into()
    Ok(n)
}

// Adding context: wrap the box in another box with a message
fn load_config(port_str: &str) -> AnyResult<String> {
    let port = parse_port(port_str)
        .map_err(|e| format!("invalid port: {}", e))?;  // contextual message wraps the box
    Ok(format!("localhost:{}", port))
}

// main() can return Box<dyn Error> too — Rust prints it on failure
fn main() -> Result<(), Box<dyn Error>> {
    let addr = load_config("8080")?;
    println!("{}", addr);
    Ok(())
}
```

The `Send + Sync` bounds matter: without them, you can't send the error across threads, which kills async code. Always use `Box<dyn Error + Send + Sync>`.

## What This Unlocks

- **Zero-boilerplate error propagation** — any `?` in the function body works, regardless of the error type
- **`main()` as an error handler** — return `Result<(), Box<dyn Error>>` from `main` and Rust prints the error on failure
- **String literals as errors** — `"something went wrong".into()` is a valid `Box<dyn Error>` for quick prototyping

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Untyped error | `exn` — exceptions are polymorphic by default | `Box<dyn Error>` — explicit type erasure |
| Any error | Raise any exception | Any type implementing `Error` can be boxed |
| Context | Wrap in new exception | Wrap with `map_err` or `.context()` extension |
| Library vs app | Same either way | Library: typed enum; App: `Box<dyn Error>` |
| Matching on variants | Pattern match exceptions | Not possible after boxing — use typed errors for libraries |

# 295: Implementing std::error::Error

**Difficulty:** 3  **Level:** Advanced

Make your custom error type a first-class citizen in the Rust error ecosystem.

## The Problem This Solves

You've written a custom error enum — but `?` won't convert it, you can't store it in a `Vec<Box<dyn Error>>`, and error-reporting tools ignore your type completely. That's because none of those things work without the `Error` trait.

The `std::error::Error` trait is the contract that makes your type interoperable with everything else. Without it, your error is a dead end: callers can't chain it, log it generically, or propagate it across crate boundaries. Every production Rust codebase needs errors that speak the ecosystem's language.

The three-part contract is simple: implement `Display` (human-readable message), `Debug` (derived for free), and optionally override `source()` to point to the underlying cause. That's it — but it unlocks everything.

## The Intuition

`std::error::Error` is the ID card that lets your error type work with `Box<dyn Error>`, error reporters, and the `?` operator's automatic conversion machinery.

## How It Works in Rust

```rust
use std::error::Error;
use std::fmt;

// Step 1: Implement Display (required by Error)
#[derive(Debug)]  // Debug is also required — derive it for free
struct ParseError { input: String, reason: String }

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse '{}': {}", self.input, self.reason)
    }
}

// Step 2: Implement the Error trait (empty body = source() returns None)
impl Error for ParseError {}

// Step 3: Wrap with source() for error chaining
#[derive(Debug)]
struct ValidationError {
    field: String,
    source: Box<dyn Error + Send + Sync>,  // boxed so it can be any error type
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "validation failed for field '{}'", self.field)
    }
}

impl Error for ValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())  // points to the underlying cause
    }
}

// Now both work with Box<dyn Error> and the ? operator
```

The key insight about `source()`: it creates a singly-linked list of errors. Walk it with `e.source()` in a loop to reconstruct the full causal chain — invaluable in logs.

## What This Unlocks

- **`Box<dyn Error>` storage** — collect heterogeneous errors in `Vec<Box<dyn Error>>` or return them from `main()`
- **Error chaining** — `source()` lets tooling (and your own code) walk the full causal chain for debugging
- **`?` across crate boundaries** — combined with `From` impls, your error type can be the target of automatic conversions from any other error

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Error trait | No standard trait — use polymorphic variants or exceptions | `std::error::Error` (requires `Display + Debug`) |
| Error chaining | Manual cause field | `source()` method — standard linked list |
| Dynamic dispatch | Exceptions are already polymorphic | `Box<dyn Error>` — explicit opt-in |
| Root cause | Traverse manually | Walk `e.source()` in a loop |

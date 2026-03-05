📖 **[View on hightechmind.io →](https://hightechmind.io/rust/312-error-downcasting)**

---

# 312: Error Downcasting

**Difficulty:** 4  **Level:** Expert

Recover the concrete error type from `Box<dyn Error>` at runtime using `.downcast_ref()` or `.downcast()`.

## The Problem This Solves

`Box<dyn Error>` is the idiomatic "I don't know what error type this is" container. It's perfect for library boundaries, generic error handling, and collecting errors from different subsystems into a single `Vec<Box<dyn Error>>`. Type erasure makes all of this composable.

But sometimes you're on the receiving end of that erased error and you *do* care about the specific type. Maybe you want to retry on `NetworkError::Timeout` but propagate `IoError::PermissionDenied`. With the concrete type erased, you're stuck treating all errors the same — unless you downcast.

Downcasting is Rust's controlled escape from the type system. It asks at runtime: "is this actually a `NetworkError` in disguise?" and gives you either a typed reference or `None`. This is explicit, opt-in dynamic typing — safe because it returns `Option`/`Result` rather than panicking on type mismatch.

## The Intuition

Think of `Box<dyn Error>` as a sealed envelope. Downcasting opens it and checks the label. If the label matches what you expected, you get the contents. If not, you get the envelope back unchanged (with `downcast`) or just `None` (with `downcast_ref`). No surprises, no undefined behavior.

## How It Works in Rust

```rust
use std::error::Error;

// downcast_ref: borrow the concrete type (non-consuming)
fn handle(e: &dyn Error) {
    if let Some(net_err) = e.downcast_ref::<NetworkError>() {
        if net_err.code == 503 { retry(); return; }
    }
    if let Some(parse_err) = e.downcast_ref::<ParseError>() {
        eprintln!("Bad input: {}", parse_err.input);
        return;
    }
    eprintln!("Unhandled error: {}", e); // generic fallback
}

// downcast: consume the Box, get ownership or the Box back
let boxed: Box<dyn Error> = produce_error();
match boxed.downcast::<ParseError>() {
    Ok(pe) => println!("Got ParseError: {:?}", pe), // owned ParseError
    Err(original) => println!("Not a ParseError: {}", original),
}
```

The `TypeId` system powers downcasting under the hood. Each `'static` type has a unique ID; the cast checks IDs at runtime and is always safe.

## What This Unlocks

- **Error type inspection at boundaries** — libraries can use `Box<dyn Error>` externally while internal handlers recover specific types for fine-grained recovery logic
- **Plugin/handler architectures** — route errors to specialized handlers based on their runtime type without a central discriminant enum
- **Interop with anyhow/eyre** — both libraries expose downcasting APIs (`downcast_ref::<T>()`) built on the same mechanism

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Dynamic error type | `exn` (extensible exception type) | `Box<dyn Error>` |
| Type inspection | Pattern match on `exn` variants | `downcast_ref::<T>()` |
| Failure mode | Match failure (unhandled exn) | Returns `Option<&T>` (safe) |
| Ownership recovery | N/A (GC) | `Box::downcast::<T>()` → `Result<Box<T>, Box<dyn Error>>` |
| Runtime cost | Pattern match overhead | Single `TypeId` comparison |

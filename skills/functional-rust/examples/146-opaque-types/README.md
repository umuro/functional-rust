# 146: Opaque Types — Hiding Implementation Details

**Difficulty:** 3  **Level:** Intermediate

Expose an interface without revealing how it's built — callers get the what, not the how.

## The Problem This Solves

Good API design separates interface from implementation. If your `Stack<T>` is internally a `Vec<T>`, clients should not be able to call `.sort()` on it or depend on vector-specific behavior. You want them to use only the operations you've designed: `push`, `pop`, `peek`.

Exposing internals creates maintenance traps: every time you change the implementation, you risk breaking clients. Hiding them gives you freedom to refactor, swap data structures, or add caching — without touching callers.

Rust has two distinct mechanisms: private fields (module-level) for hiding the *type representation*, and `impl Trait` return types for hiding the *concrete type of a return value*.

## The Intuition

**Private fields:** A struct with a private inner `Vec<T>` forces all access through the public methods you define. Outside the module, there's no way to access the underlying Vec — not even for reading. The type is opaque.

**`impl Trait` returns:** When a function returns `impl Iterator<Item = i32>`, the caller knows it's *some* iterator that yields `i32` values. They don't know if it's a `Map`, a `Filter`, a `Chain`, or a custom type. This freedom is valuable: you can change the return type without changing the function signature.

## How It Works in Rust

```rust
pub mod stack {
    /// Callers see Stack<T> as an opaque type.
    /// The inner Vec is private — no direct access outside this module.
    #[derive(Debug, Clone)]
    pub struct Stack<T>(Vec<T>);  // tuple struct: field is private by default

    impl<T> Stack<T> {
        pub fn empty() -> Self { Stack(Vec::new()) }

        pub fn push(mut self, x: T) -> Self {
            self.0.push(x);
            self
        }

        pub fn pop(mut self) -> (Option<T>, Self) {
            let top = self.0.pop();
            (top, self)
        }

        pub fn peek(&self) -> Option<&T> { self.0.last() }
        pub fn size(&self) -> usize { self.0.len() }
    }
    // Outside this module: s.0 would be a compile error
}
```

`impl Trait` opaque return type:

```rust
// Caller sees "some iterator of i32" — not the concrete type
fn make_counter(start: i32, step: i32) -> impl Iterator<Item = i32> {
    (0..).map(move |i| start + i * step)
    // Could be changed to a different iterator type without breaking callers
}

let counter: Vec<i32> = make_counter(10, 5).take(4).collect();
// [10, 15, 20, 25]
```

Validated opaque type — can only be constructed via a checked constructor:

```rust
pub struct SecretKey(Vec<u8>);  // field is private

impl SecretKey {
    pub fn new(key: &[u8]) -> Result<Self, &'static str> {
        if key.len() >= 16 {
            Ok(SecretKey(key.to_vec()))  // only valid keys can be created
        } else {
            Err("key must be at least 16 bytes")
        }
    }
    // No way to construct SecretKey without going through `new`
}
```

## What This Unlocks

- **API stability** — change the internal representation freely; callers never knew it
- **Invariant enforcement** — opaque constructors guarantee the value is always valid
- **Iterator composition** — return complex iterator chains behind a simple `impl Iterator` without exposing intermediate types

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Hide type representation | Module signature with abstract type: `type t` | Private fields in struct (module-level privacy) |
| Opaque return type | Not directly expressible | `impl Trait` in function return position |
| Constructor validation | Module function, abstract type prevents bypass | Private field + public constructor |
| Sealing a trait | First-class modules / private signature | Private supertrait (sealed trait pattern) |
| Scope of privacy | Signature/module boundary | `pub(crate)`, `pub(super)`, or fully private |

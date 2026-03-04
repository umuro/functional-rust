# 387: Sealed Trait Pattern

**Difficulty:** 3  **Level:** Advanced

Prevent external crates from implementing your trait — lock down your abstraction boundary.

## The Problem This Solves

You design a trait that must only be implemented by types in your crate. Maybe it's a token trait (`Sealed`) used in your public API, and external implementations would break your invariants. Maybe it's a state machine trait where only your crate's types are valid states. Maybe you want the freedom to add methods to the trait later without breaking semver — which you can't do with a public trait anyone might implement.

Rust doesn't have a built-in `sealed` keyword, but the pattern is easy to implement: put a hidden supertrait in a private module. External crates can see your public trait but can't implement the private supertrait, so they can't implement the public trait either.

This pattern is widely used in the Rust ecosystem: `tokio`, `futures`, and `serde` all use it to mark traits as not-meant-for-external-implementation.

## The Intuition

The trick is that `impl MyPublicTrait for ExternalType` requires `impl private::Sealed for ExternalType` (because `MyPublicTrait: private::Sealed`). But `private::Sealed` is not accessible outside your crate — the module is private. So the impl block can't compile. The user sees your public trait and can call its methods, but they can't add new implementations.

This is Rust's module system doing the work, not a language feature. It's a convention, not a hard keyword — but it's effective and idiomatic.

## How It Works in Rust

```rust
// In your crate: lib.rs

mod private {
    // Not pub — invisible outside this crate
    pub trait Sealed {}
}

// Public trait — requires the private supertrait
pub trait Validated: private::Sealed {
    fn value(&self) -> &str;
}

// Your crate's types can implement Sealed (they're in the same crate)
pub struct Email(String);
pub struct Url(String);

impl private::Sealed for Email {}
impl private::Sealed for Url {}

impl Validated for Email {
    fn value(&self) -> &str { &self.0 }
}

impl Validated for Url {
    fn value(&self) -> &str { &self.0 }
}

// External crate trying to implement Validated:
// impl Validated for ExternalType { ... }
// ERROR: trait `private::Sealed` is not accessible
```

Users of your crate can write `fn process(v: &dyn Validated)` and call `v.value()` — they just can't add new implementations.

## What This Unlocks

- **Stable extension points** — add methods to a sealed trait without breaking semver (no external impls to break).
- **Invariant enforcement** — ensure only your carefully-constructed types satisfy the trait.
- **State machine types** — seal the state trait so only your crate's states are valid.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Module-private type | `module M : sig type t end` (abstract type) | `mod private { pub trait Sealed {} }` |
| Prevent external impl | Abstract module signatures | Sealed trait via private supertrait |
| Closed set of types | Variant types (closed by design) | Sealed trait (open syntax, closed semantics) |
| Adding methods safely | Adding to module signature | Add to sealed trait — no external impls to break |

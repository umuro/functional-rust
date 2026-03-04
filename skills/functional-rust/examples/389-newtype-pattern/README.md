# 389: Newtype Pattern

**Difficulty:** 2  **Level:** Intermediate

Wrap a type to give it distinct identity and semantics — preventing accidental misuse at zero runtime cost.

## The Problem This Solves

You have a function that takes two `f64` arguments: `fn set_position(x: f64, y: f64)`. Nothing stops a caller from accidentally swapping them: `set_position(y_coord, x_coord)`. It compiles. It runs. It's wrong. A `String` for an email address and a `String` for a username are type-identical but semantically incompatible — the compiler won't catch it if you pass one where the other is expected.

The newtype pattern solves this by wrapping each primitive in a distinct struct: `struct Email(String)`. Now `Email` and `Username` are different types — passing one where the other is expected is a compile error. You get type-level documentation and compiler enforcement for free, with zero runtime overhead (the wrapper is erased).

This is one of Rust's most practically useful patterns, and it scales from simple `struct Meters(f64)` to complex `struct UserId(Uuid)` in production systems.

## The Intuition

`struct Email(String)` is a tuple struct with one field. At runtime, it's identical to `String` — no extra memory, no extra indirection. At compile time, it's a completely different type. The Rust compiler cannot convert between `Email` and `Username` without an explicit `.0` field access.

The pattern also lets you implement traits on the wrapper that you can't implement on the inner type (the orphan rule prevents `impl Display for String` in your crate, but `impl Display for Email` is fine). This is the standard workaround for the orphan rule.

## How It Works in Rust

```rust
// Distinct types for distinct concepts
struct Email(String);
struct Username(String);
struct Meters(f64);
struct Feet(f64);

impl Email {
    fn new(s: &str) -> Result<Self, &'static str> {
        if s.contains('@') { Ok(Email(s.to_string())) }
        else { Err("invalid email") }
    }

    fn as_str(&self) -> &str { &self.0 }
}

fn send_welcome(email: &Email, name: &Username) {
    println!("Welcome {}, sending to {}", name.0, email.as_str());
}

let email = Email::new("alice@example.com").unwrap();
let name  = Username("Alice".to_string());

send_welcome(&email, &name);       // OK
// send_welcome(&name, &email);    // compile error: wrong types

// Implement traits on the wrapper (orphan rule workaround)
use std::fmt;
impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Zero-cost: same memory layout as inner type
assert_eq!(std::mem::size_of::<Email>(), std::mem::size_of::<String>());
```

Use `#[repr(transparent)]` when FFI requires the wrapper to have identical ABI to the inner type.

## What This Unlocks

- **Type-safe units** — `Meters` and `Feet` can't be accidentally added; conversion is explicit.
- **Validated constructors** — `Email::new()` validates on construction; valid-by-construction invariants.
- **Orphan rule workaround** — implement external traits on foreign types by wrapping them.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Distinct type alias | `type email = string` (alias, not distinct!) | `struct Email(String)` (distinct type) |
| Abstract type | `module M : sig type t end` (fully abstract) | `struct Email(String)` with private field |
| Zero-cost wrapper | N/A — types are structural in OCaml | `#[repr(transparent)]` guarantees same layout |
| Orphan rule workaround | Module system | Newtype wrapper enables trait impls |

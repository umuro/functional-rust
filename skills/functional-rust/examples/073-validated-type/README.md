# 073: Validated Type — Smart Constructors

**Difficulty:** 3  **Level:** Intermediate

Parse at the boundary, validate once, never check again — opaque newtypes that enforce invariants at construction time.

## The Problem This Solves

Functions often receive strings or numbers that must meet certain criteria: an email must contain `@`, a port must be between 1–65535, a price must be positive. The naive approach validates at every use site — scattered `if` checks that are easy to forget and inconsistent to audit.

Smart constructors flip this: validation happens once, at the moment you create the value. If construction succeeds, you hold a type that *by construction* satisfies the invariant — forever, everywhere it travels in your code. No function that receives `Email` needs to re-check it's valid. The compiler enforces the guarantee.

This is the "parse, don't validate" principle: transform unstructured input into a typed proof of validity.

## The Intuition

In Python or JavaScript, you'd pass a raw string around and check `is_valid_email(s)` at each use site. Bugs emerge when someone skips the check.

Rust's private fields make the newtype pattern enforceable: if the inner field is private, the *only* way to construct an `Email` is through `Email::parse()`. The type system becomes your validator — once the value exists, it's always valid.

Compare to OCaml's `type email = private string` — Rust achieves the same with a tuple struct and `pub`/private field control.

## How It Works in Rust

```rust
// The inner field is private — only constructible via ::create()
#[derive(Debug, Clone, PartialEq)]
pub struct NonEmptyString(String); // private field!

impl NonEmptyString {
    // The one true entrance: validates, then wraps
    pub fn create(s: &str) -> Result<Self, String> {
        if !s.is_empty() {
            Ok(NonEmptyString(s.to_string()))
        } else {
            Err("string must be non-empty".to_string())
        }
    }

    // Read-only access — no way to get a mutable reference to the inner String
    pub fn value(&self) -> &str { &self.0 }

    // Operations on the validated type stay valid — concat of two non-empty strings is non-empty
    pub fn concat(&self, other: &NonEmptyString) -> NonEmptyString {
        NonEmptyString(format!("{}{}", self.0, other.0))
    }
}

// Positive integer: validated at construction, always > 0 after that
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PositiveInt(u64);

impl PositiveInt {
    pub fn new(n: u64) -> Result<Self, String> {
        if n > 0 { Ok(PositiveInt(n)) }
        else { Err("must be > 0".to_string()) }
    }

    pub fn value(self) -> u64 { self.0 }

    // PositiveInt + PositiveInt is always positive — no re-validation needed
    pub fn add(self, other: PositiveInt) -> PositiveInt {
        PositiveInt(self.0 + other.0) // safe: sum of positives is positive
    }
}
```

The key: make the inner field private, expose only a `parse`/`create`/`new` method returning `Result`. Downstream code never deals with invalid states.

## What This Unlocks

- **Fearless APIs**: function signatures express their preconditions — `fn send(to: Email)` can't receive garbage.
- **Composable validity**: operations on valid types can often skip re-validation (concat of non-empty strings is non-empty).
- **Audit-friendly code**: validation logic lives in one place — the constructor. To find all validation rules, look only there.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Opaque type | `type t = private string` | Tuple struct with private field |
| Constructor | `let create s = if ... then Ok (wrap s) else Error ...` | `pub fn create(s: &str) -> Result<Self, E>` |
| Field access | Module-controlled | `pub fn value(&self) -> &T` |
| Derive traits | `[@@deriving eq, show]` | `#[derive(Debug, Clone, PartialEq)]` |
| Invalid states | Prevented by private type | Prevented by private field + Result constructor |

# 741: Parse-Don't-Validate: Rich Types Over Runtime Checks

**Difficulty:** 4  **Level:** Expert

Parse once at the boundary into a type that can only hold valid values — all code that receives the type knows it's valid without checking.

## The Problem This Solves

Here's a common pattern: a function receives a `String` representing an email address, calls `is_valid_email(&email)` at the start to check it, and proceeds. That check must be repeated in every function that handles email strings — or it gets forgotten. If any call site skips the check, bugs slip through silently.

The "parse, don't validate" principle (coined by Alexis King) says: validate *once* at the input boundary, and return a type that proves validity. An `Email` newtype can only be constructed by `Email::parse()`, which validates the format. Any function that receives an `Email` knows it's valid — the type is the proof. No defensive checks needed at every call site.

This is the Rust translation of making impossible states unrepresentable. `String` can hold `"not-an-email"`. `Email` cannot. The type system does the documentation and enforcement simultaneously.

## The Intuition

In Python, you might create a `@dataclass` with `__post_init__` validation, or use `pydantic`. In TypeScript, you'd use a branded type: `type Email = string & { _brand: 'Email' }`. In Rust, the newtype pattern with a private field is the idiomatic approach.

The key is *private fields* + *a single parse constructor*. The field being private means `Email("bad@")` doesn't compile from outside the module — callers must go through `Email::parse()`. The parse function validates and returns `Result<Email, ParseError>`, forcing callers to handle the error at the boundary.

Once you have an `Email`, you never check validity again. Functions that need an email take `Email`, not `String`. The type carries its own proof of validity through the entire program.

## How It Works in Rust

```rust
// Private field — cannot construct Email directly from outside this module
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);  // private field

impl Email {
    /// The only way to create an Email — validates at the boundary
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        let at = s.find('@').ok_or_else(|| ParseError::InvalidEmail(s.to_owned()))?;
        let (local, domain) = s.split_at(at);
        let domain = &domain[1..]; // skip '@'
        if local.is_empty() || !domain.contains('.') || domain.starts_with('.') {
            return Err(ParseError::InvalidEmail(s.to_owned()));
        }
        Ok(Email(s.to_ascii_lowercase()))
    }

    pub fn as_str(&self) -> &str { &self.0 }
    pub fn domain(&self) -> &str { self.0.split('@').nth(1).unwrap() }
}

// Before parse-don't-validate (the bad way):
fn send_email_bad(address: &str) {
    if !is_valid_email(address) { panic!("invalid email"); }  // forgotten at half the call sites
    // ...
}

// After parse-don't-validate (the good way):
fn send_email(to: &Email, subject: &str) {  // can only receive a valid Email
    // No check needed — the type is the proof
    println!("Sending to {}", to.as_str());
}

// Parse at the boundary — once
let email = Email::parse("Alice@Example.COM")
    .expect("invalid email address");
// email is now Email("alice@example.com") — lowercase-normalized, validated
send_email(&email, "Hello!");

// Bounded integer — can only hold values in [1, 100]
pub struct Percentage(u8);

impl Percentage {
    pub fn parse(n: i64) -> Result<Self, ParseError> {
        if n < 1 || n > 100 {
            return Err(ParseError::OutOfRange { value: n, lo: 1, hi: 100 });
        }
        Ok(Percentage(n as u8))
    }
    pub fn value(&self) -> u8 { self.0 }
}

// Function that requires a valid percentage — no range check needed
fn set_volume(level: Percentage) {
    // level.value() is guaranteed [1, 100] — we know without checking
}
```

Key points:
- Private field `Email(String)` — outside code can't write `Email("bad")`, only `Email::parse(...)`
- `Email::parse` returns `Result<Email, ParseError>` — forces callers to handle invalid input
- Normalization in the constructor (`to_ascii_lowercase`) means all `Email` values have a canonical form
- `Hash + Eq` derived means `Email` can be used in `HashSet<Email>` and `HashMap<Email, _>`
- Functions that receive `Email` instead of `&str` are easier to read: the type documents the precondition

## What This Unlocks

- **Eliminate defensive programming**: every function that takes `Email`, `NonEmptyString`, or `Percentage` can drop its validation checks — the type already guarantees validity
- **Canonical representation**: the parse constructor normalizes inputs (lowercase email, trimmed whitespace, etc.) — no inconsistency between `"Alice@Example.COM"` and `"alice@example.com"`
- **Domain modeling**: `UserId(u64)`, `OrderId(u64)`, `CustomerId(u64)` prevent accidentally passing the wrong ID to a function — newtypes make distinct concepts distinct types

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Opaque type | Abstract type in `.mli` signature | Struct with private field |
| Smart constructor | Function returning `result` type | `fn parse(s: &str) -> Result<Self, ParseError>` |
| Private field | Module-level privacy via `.mli` | `pub struct Email(String)` — field private by default |
| Canonical form | Constructor applies normalization | Same — `to_ascii_lowercase()` in `parse()` |
| Hash/equality | Structural equality by default | Derived `Hash + Eq` |
| Bounded integer | Variant type with constraint | Newtype struct with range check in `parse()` |

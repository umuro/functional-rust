# 484: Cow<str> for Flexible Strings

**Difficulty:** 2  **Level:** Intermediate

Avoid unnecessary heap allocations by holding either a borrowed `&str` or an owned `String` — decided at runtime.

## The Problem This Solves

Imagine a function that sanitizes a string: if it's already clean, return it as-is; if it needs modification, return the modified version. In Python or JavaScript, you'd return a new string either way — even if nothing changed, you allocate. In Rust, returning a `String` means you always allocate a copy. Returning `&str` means you can never allocate. Neither is right.

`Cow<'a, str>` ("Clone on Write") solves this. It's an enum that holds *either* a borrowed `&str` *or* an owned `String`. If your data needs no modification, you return `Cow::Borrowed(s)` — zero allocation, the original `&str` passed through. If modification is needed, you return `Cow::Owned(new_string)` — allocated only when necessary.

This pattern appears throughout the standard library. `String::from_utf8_lossy()` returns `Cow<str>` — if the input is valid UTF-8, you get a borrowed view; if replacement was needed, you get an owned `String`. Many serialization and normalization functions use the same approach.

## The Intuition

`Cow<str>` is Rust's way of saying "I might need to own this, I might not — decide at runtime." The mental model:

- `Cow::Borrowed(&str)` — "I'm just looking, not touching."
- `Cow::Owned(String)` — "I made a copy and modified it."

Both variants implement `Deref<Target = str>`, so you can call any `str` method on a `Cow<str>` without knowing which variant it is. `cow.trim()`, `cow.contains("x")`, `cow.len()` — all work transparently.

There's no direct OCaml equivalent. OCaml strings are immutable, so the question of "borrow or own" doesn't arise the same way. The closest analogy in any language is Go's `[]byte` vs `string` distinction, or Scala's `lazy val`.

## How It Works in Rust

```rust
use std::borrow::Cow;

// Returns borrowed if no spaces, owned if spaces found
fn ensure_no_spaces(s: &str) -> Cow<str> {
    if !s.contains(' ') {
        Cow::Borrowed(s)               // zero allocation — just a view
    } else {
        Cow::Owned(s.replace(' ', "_")) // allocates only when needed
    }
}

let clean = ensure_no_spaces("hello_world");
let dirty = ensure_no_spaces("hello world");

// Both work the same — Cow deref's to &str transparently
println!("{}", clean);  // "hello_world"  (no allocation)
println!("{}", dirty);  // "hello_world"  (allocated)

// Check which variant at runtime
matches!(clean, Cow::Borrowed(_))  // true
matches!(dirty, Cow::Owned(_))     // true

// Accept both &str and String in one parameter type
fn process(input: Cow<str>) -> String {
    format!("processed: {}", input.trim())  // works on both variants
}

process(Cow::Borrowed("  hello  "));
process(Cow::Owned(String::from("  world  ")));

// Cow::into_owned() — get a String regardless of variant
let s: String = clean.into_owned();  // copies if Borrowed, moves if Owned

// from_utf8_lossy returns Cow<str>
let bytes = b"hello world";
let cow = String::from_utf8_lossy(bytes);  // Borrowed (valid UTF-8, no copy)
```

## What This Unlocks

- **Zero-cost sanitization functions** — return borrowed input unchanged, allocate only on modification.
- **Flexible API design** — functions that accept `Cow<str>` work with both `&str` literals and owned `String` values.
- **Efficient lossy UTF-8 conversion** — `from_utf8_lossy()` avoids allocation for valid input.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Borrow or own | No distinction (strings immutable) | `Cow<str>` — explicit borrowed vs owned |
| Conditional allocation | Always allocates on transform | `Cow::Borrowed` avoids allocation |
| Uniform API | N/A | `Deref<Target=str>` — same methods on both |
| Runtime check | N/A | `matches!(cow, Cow::Borrowed(_))` |
| Force owned | N/A | `.into_owned()` → `String` |
| Closest analog | `type cow_str = Borrowed of string \| Owned of Buffer.t` | `Cow<'a, str>` — built into std |

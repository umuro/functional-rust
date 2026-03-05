📖 **[View on hightechmind.io →](https://hightechmind.io/rust/403-display-debug-traits)**

---

# 403: Display, Debug, and Formatting

**Difficulty:** 2  **Level:** Intermediate

Two separate formatting traits for two audiences — users and developers — with format specifiers that call each one.

## The Problem This Solves

Every type eventually needs to be printed. But "printing for users" and "printing for debugging" are different needs. A `Duration` displayed to a user should say "2 hours 15 minutes." Displayed to a developer it should show `Duration { secs: 8100, nanos: 0 }`. Conflating these creates user-facing output full of struct internals, or debug sessions where you can't see what's actually stored.

Rust separates these concerns at the trait level. `Display` (invoked by `{}`) is for human-readable, user-facing output that you design deliberately. `Debug` (invoked by `{:?}`) is for developers — show everything, make it inspectable. You implement `Display` manually when you care about the presentation. You derive `Debug` automatically in almost every case.

A third need: zero-allocation formatting — `format_args!` captures format arguments lazily without immediately allocating a `String`. This matters in hot paths, logging infrastructure, and embedded systems.

## The Intuition

`{}` calls `Display` (write this for humans), `{:?}` calls `Debug` (write this for developers) — two audiences, two traits, one type.

## How It Works in Rust

```rust
use std::fmt;

struct Temperature { celsius: f64 }

// Display: user-facing, deliberate design
impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.celsius)
    }
}

// Debug: auto-derived for developer inspection
#[derive(Debug)]
struct Temperature { celsius: f64 }

// Usage
let t = Temperature { celsius: 23.5 };
println!("{}", t);    // "23.5°C"     — Display
println!("{:?}", t);  // "Temperature { celsius: 23.5 }" — Debug
println!("{:#?}", t); // pretty-printed Debug

// Format specifiers
println!("{:>10}", t);  // right-align in 10 chars
println!("{:0>5}", 42); // "00042" — zero-pad

// Custom Debug (when derived doesn't suit)
impl fmt::Debug for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Temperature")
            .field("celsius", &self.celsius)
            .field("fahrenheit", &(self.celsius * 1.8 + 32.0))
            .finish()
    }
}
```

1. Derive `Debug` on almost every type — it's free and invaluable.
2. Implement `Display` when the type has a natural user-facing representation.
3. `write!(f, ...)` in the `fmt` method — same syntax as `println!` but writes to a formatter.

## What This Unlocks

- **Free debugging**: `#[derive(Debug)]` + `{:?}` = inspectable types with zero effort.
- **Error messages**: Implement `Display` on error types for human-readable error reporting.
- **Custom format specifiers**: Implement `fmt::LowerHex`, `fmt::Binary`, etc. for `{:x}`, `{:b}` support.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| User-facing print | `Printf.printf` / `Format.printf` | `impl Display` + `{}` |
| Debug/developer print | `#show_type` or manual | `#[derive(Debug)]` + `{:?}` |
| Format strings | `%d`, `%s` type-checked at compile time | `{}`, `{:?}` trait-dispatched |
| Custom formatters | `Format.pp_print_*` | `impl fmt::Display` / `impl fmt::Debug` |
| Pretty print | `Format` module with boxes | `{:#?}` for pretty Debug, or custom `Display` |

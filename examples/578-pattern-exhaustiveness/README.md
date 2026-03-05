# 578: Exhaustive Matching with _

**Difficulty:** 2  **Level:** Beginner

Understand Rust's exhaustiveness guarantee — every possible value is handled, and the compiler proves it.

## The Problem This Solves

The "unhandled case" bug is one of the most common runtime errors in languages without exhaustive matching. You add a new status code, a new message type, a new direction to an enum — and somewhere, a switch statement silently falls through to a default that does nothing, or returns a wrong value. The bug ships. It's found in production.

In Java, C, and Python, adding an enum variant requires you to manually audit every switch/match in the codebase. Miss one and it compiles fine. In most languages, the `default` or `_` branch is a cop-out that hides future variants.

Rust makes exhaustiveness a compile-time guarantee. Add `Dir::Northeast` to your direction enum and every `match d { ... }` that doesn't cover it — or doesn't have a `_` catch-all — stops compiling. The compiler lists every location that needs updating. You fix them before the code runs.

## The Intuition

A Rust `match` must be exhaustive: the patterns must cover every possible value of the scrutinee type. For a four-variant enum, that means four arms (or fewer arms plus a `_` wildcard). Leave one out and the compiler tells you which variant is unhandled.

The `_` wildcard is your "everything else" — it matches anything not caught by earlier arms. Use it deliberately, not reflexively. A `match` with no `_` that covers all variants is *stronger* than one with `_`: if you add a variant, the compiler notices. With `_`, the new variant silently hits the catch-all.

`#[non_exhaustive]` is the library author's tool: marking a public enum with it tells consumers they *must* have a `_` arm, because future versions of the library may add variants. Your match stays correct as the library evolves.

## How It Works in Rust

```rust
// No _ needed — compiler verifies all 4 variants covered
fn describe(d: &Dir) -> &'static str {
    match d {
        Dir::N => "north",
        Dir::S => "south",
        Dir::E => "east",
        Dir::W => "west",
        // Try commenting one out — the compiler will name the missing variant
    }
}

// _ as explicit catch-all
fn horizontal(d: &Dir) -> bool {
    match d { Dir::E | Dir::W => true, _ => false }
}

// #[non_exhaustive] — library forces consumers to have a catch-all
#[non_exhaustive]
enum StatusCode { Ok, NotFound, Unauthorized, ServerError }

fn status_text(c: &StatusCode) -> &'static str {
    match c {
        StatusCode::Ok           => "OK",
        StatusCode::NotFound     => "Not Found",
        StatusCode::Unauthorized => "Unauthorized",
        StatusCode::ServerError  => "Internal Server Error",
        _ => "Unknown",  // required — new variants may be added
    }
}

// Full integer coverage with ranges — no _ needed, compiler verifies
fn sign(n: i32) -> &'static str {
    match n {
        i32::MIN..=-1 => "negative",
        0             => "zero",
        1..=i32::MAX  => "positive",
    }
}
```

## What This Unlocks

- **Compiler-guided refactoring** — add an enum variant and the compiler shows every match that needs updating.
- **Zero runtime "unhandled case" errors** — every path through a match has an explicit arm.
- **Deliberate vs. accidental wildcards** — use `_` when you mean "everything else," not as a lazy default.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Missing arm | Warning (warning 8) | Hard compile error |
| Wildcard | `_` | `_` |
| Or patterns | `| A | B ->` | `A \| B =>` |
| Non-exhaustive | No direct equivalent | `#[non_exhaustive]` attribute |
| Integer exhaustiveness | Guards — compiler can't verify | Range patterns — compiler verifies |

# 422: Derive Macros: Concept and Usage

**Difficulty:** 3  **Level:** Advanced

Demystify `#[derive(Debug)]` by showing what code it actually generates — and understand when to use derive vs write implementations by hand.

## The Problem This Solves

Every Rust beginner writes `#[derive(Debug, Clone, PartialEq)]` and it magically works. But when something goes wrong — a custom type doesn't derive, a newtype wraps a non-`Hash` type, an enum variant breaks `Ord` ordering — you're stuck. You can't debug what you don't understand.

Beyond debugging, derive macros are the entry point to a deeper capability: procedural macros. `#[derive(Serialize)]` from serde, `#[derive(Error)]` from thiserror, `#[derive(Component)]` from Bevy — all of these are proc macros that generate substantial code from a struct definition. Understanding what `#[derive(Debug)]` actually emits teaches you to reason about what any derive macro might emit.

The manual equivalents also matter in practice. You'll hand-write `PartialEq` when two structs are equal only when a subset of fields match, or `Debug` when you want to redact a password field. Knowing what derive would have generated makes writing the custom version straightforward.

## The Intuition

`#[derive(Debug)]` is a code generator. Before compilation, the Rust compiler hands the macro your struct definition and says "generate the `Debug` impl." The macro produces something like what you'd write yourself — `f.debug_struct("Point").field("x", &self.x).field("y", &self.y).finish()` — and that generated code is compiled alongside your own.

All standard derive traits follow the same pattern: they inspect each field or variant and compose the implementation recursively. `#[derive(Clone)]` calls `.clone()` on each field. `#[derive(PartialEq)]` compares fields pairwise. `#[derive(Ord)]` compares fields left to right, using the first non-equal result.

## How It Works in Rust

```rust
// What you write:
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Point { x: i32, y: i32 }

// What #[derive(Debug)] generates (simplified):
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)   // calls Debug on each field
            .field("y", &self.y)
            .finish()
    }
}

// What #[derive(PartialEq)] generates:
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// What #[derive(Default)] generates:
impl Default for Point {
    fn default() -> Self {
        Point { x: i32::default(), y: i32::default() }
    }
}
```

**Practical outcomes of derived traits:**
- `Debug` → `{:?}` formatting and `dbg!()` macro
- `Hash + Eq` → use as `HashMap` key
- `Ord` → `.sort()` on `Vec<T>`
- `Clone` → `.clone()` method
- `Default` → `..Default::default()` struct update syntax

## What This Unlocks

- **Custom derives for your own types** — understanding the pattern leads directly to writing proc macros with `syn` + `quote` that generate any code from a struct definition.
- **Knowing when NOT to derive** — hand-write `Debug` to redact secrets, hand-write `PartialEq` for semantic equality (two `HashMap`s with same entries but different capacities are equal).
- **Third-party derives** — `serde::Serialize`, `thiserror::Error`, `clap::Parser` all follow this same expansion model; you can reason about what they generate.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Code generation from type | `ppx_deriving` (show, eq, ord) — third-party, requires opam pkg | `#[derive(...)]` — built into compiler, no deps for std traits |
| Manual equivalent | Write `show_point`, `equal_point` functions | Implement `fmt::Debug`, `PartialEq` traits |
| Syntax | `[@@deriving show, eq]` attribute | `#[derive(Debug, PartialEq)]` attribute |
| Ordering | `compare` polymorphic function | `PartialOrd` + `Ord` traits; field order matters for derived `Ord` |
| Hash map key | Any type with structural equality | Requires `Hash + Eq` both derived or both implemented |

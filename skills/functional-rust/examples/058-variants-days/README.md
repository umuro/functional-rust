# 058: Variants — Days of the Week

**Difficulty:** 1  **Level:** Beginner

Model a finite set of named values as an enum with exhaustive pattern matching.

## The Problem This Solves

Days of the week are a fixed, finite set. Using `&str` or `i32` to represent them means you can pass `"Funday"` or `8` and the compiler won't stop you. A typo compiles. An invalid value compiles.

An enum makes invalid values unrepresentable. `Day::Funday` doesn't compile. `Day::from_index(8)` returns `None`. The compiler checks every `match` is exhaustive — if you add `Day::Holiday` later, every match arm that forgot to handle it becomes a compile error, not a runtime surprise.

This is the **algebraic data type** applied to the simplest case: a sum type with no data attached to each variant (called a *fieldless* or *C-like* enum).

## The Intuition

Variants are named constants that the type system tracks. Instead of `1 = Monday, 2 = Tuesday`, you have `Day::Mon, Day::Tue`. The compiler knows exactly which values exist. The `match` expression forces you to handle all of them.

In OCaml, you get equality, printing, and copying for free. In Rust, you opt into traits with `#[derive]`. Same concept, more explicit declaration.

## How It Works in Rust

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Day { Sun, Mon, Tue, Wed, Thu, Fri, Sat }
//              ^ derive gives: print, copy, compare — all free

impl Day {
    pub fn name(self) -> &'static str {
        match self {
            Day::Sun => "Sunday", Day::Mon => "Monday",
            // ... compiler errors if any variant is missing
            Day::Sat => "Saturday",
        }
    }

    pub fn is_weekend(self) -> bool {
        matches!(self, Day::Sun | Day::Sat)  // matches! macro for boolean patterns
    }

    pub fn next(self) -> Day {
        match self { Day::Sat => Day::Sun, Day::Sun => Day::Mon, /* ... */ }
    }

    // Arithmetic alternative (avoids exhaustive match):
    pub fn next_arithmetic(self) -> Day {
        Day::from_index((self as u8 + 1) % 7).unwrap()
    }
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())  // enables format!("{day}")
    }
}
```

`Copy` makes `Day` cheaply copyable (no heap allocation). The `as u8` cast gives access to the discriminant for arithmetic.

## What This Unlocks

- **Type-safe domain values** — directions, states, card suits, HTTP methods — any fixed set of named things.
- **Exhaustive handling** — the compiler guarantees you've handled every variant; add a new one and find every match that needs updating.
- **State machines** — enums with methods like `next()` are the foundation for finite state automata.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Declaration | `type day = Sun \| Mon \| ...` | `enum Day { Sun, Mon, ... }` |
| Debug printing | Automatic | `#[derive(Debug)]` required |
| Equality | Automatic | `#[derive(PartialEq, Eq)]` required |
| Copy semantics | Automatic | `#[derive(Clone, Copy)]` required |
| Methods | Free functions | `impl Day { fn name(self) ... }` |
| Display | Manual or ppx | `impl std::fmt::Display for Day` |

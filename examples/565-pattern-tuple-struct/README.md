📖 **[View on hightechmind.io →](https://hightechmind.io/rust/565-pattern-tuple-struct)**

---

# 565: Tuple Struct Patterns

**Difficulty:** 2  **Level:** Beginner

Destructure newtype and multi-field tuple structs in patterns — get type safety without accessor boilerplate.

## The Problem This Solves

Primitive obsession is the bug where your function takes three `f64` arguments — distance, time, and speed — and nothing stops you from passing them in the wrong order. At the call site they're all just `f64`. The compiler can't help you. Tests catch it only if you write them right.

Tuple structs are the lightweight fix. `Meters(f64)` and `Seconds(f64)` are distinct types. `speed(seconds, meters)` won't compile. You pay almost nothing: no field names, just a wrapper type. But to use the inner value, you have to destructure — and that's where patterns shine.

The same technique applies whenever you want branded primitives: `UserId(u64)`, `Rgb(u8, u8, u8)`, `NonEmpty(String)`. You define one type, one pattern, and you're done.

## The Intuition

A tuple struct is just a struct with positional fields — `struct Rgb(u8, u8, u8)` is like `struct Rgb { 0: u8, 1: u8, 2: u8 }` without the field names. The pattern mirrors the constructor: if you built it with `Rgb(255, 0, 0)`, you unpack it with `Rgb(r, g, b)`.

This is OCaml's single-constructor type — `type meters = Meters of float` — used as a newtype wrapper. The Rust version is slightly more ergonomic because it works directly in function parameter position.

The pattern `fn add(Meters(a): Meters, Meters(b): Meters)` reads as: "this function takes something shaped like `Meters(a)` and `Meters(b)`." The destructuring happens right at the boundary. No intermediate variables, no `.0` accessor noise.

## How It Works in Rust

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Meters(f64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Seconds(f64);

#[derive(Debug, Clone, Copy)]
struct Rgb(u8, u8, u8);

// Destructure in function parameter — can't mix up types
fn add(Meters(a): Meters, Meters(b): Meters) -> Meters {
    Meters(a + b)
}

fn speed(Meters(d): Meters, Seconds(t): Seconds) -> f64 {
    d / t  // d and t are plain f64 — safe to combine here
}

// Multi-field tuple struct
fn to_gray(Rgb(r, g, b): Rgb) -> Rgb {
    let avg = ((r as u16 + g as u16 + b as u16) / 3) as u8;
    Rgb(avg, avg, avg)
}

// Destructure in let binding
let Meters(total) = add(Meters(100.0), Meters(50.0));
println!("{:.1} m", total);  // 150.0 m

// Destructure in match
match color {
    Rgb(255, 0, 0) => "pure red",
    Rgb(r, g, b) if r == g && g == b => "gray",
    _ => "other",
}
```

## What This Unlocks

- **Type-safe units** — `Meters`, `Seconds`, `Celsius`, `UserId` catch transposition errors at compile time.
- **Zero-cost abstraction** — tuple structs compile to the same memory layout as the inner type; no overhead.
- **Pattern matching on shape** — combine with match to branch on specific values (`Rgb(255, 0, 0)`) or extract all fields (`Rgb(r, g, b)`).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Definition | `type meters = Meters of float` | `struct Meters(f64);` |
| Constructor | `Meters 3.0` | `Meters(3.0)` |
| Destructure in `let` | `let Meters n = x in ...` | `let Meters(n) = x;` |
| Destructure in param | `let f (Meters n) = ...` | `fn f(Meters(n): Meters)` |
| Multi-field | `type rgb = RGB of int * int * int` | `struct Rgb(u8, u8, u8)` |

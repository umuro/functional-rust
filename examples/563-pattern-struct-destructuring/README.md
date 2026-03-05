# 563: Struct Destructuring

**Difficulty:** 2  **Level:** Beginner

Extract struct fields directly into bindings — no `.field` access needed.

## The Problem This Solves

Without destructuring, pulling data out of structs is verbose noise. You write `person.name`, `person.age`, `person.age` again, over and over. When a function only cares about two of five fields, you still have to lug the whole struct around and access each field by name at every use site.

The real pain shows up in `match` guards: `if person.age < 18` is fine, but when you're pattern-matching on a `Person` wrapped inside an `Option` inside a `Vec`, you end up with deeply nested `.field` chains that bury the actual logic.

Struct destructuring lets you name what you want, right where you need it, and ignore the rest with `..`. The signal-to-noise ratio flips: the binding is front-and-center, the struct name is just context.

## The Intuition

Think of it as unpacking a labelled box. You write the label you want (`x`, `name`, `age`) and Rust pulls the matching field. You can skip fields with `..`, rename them with `field: local_name`, and combine destructuring with guards in a single `match` arm.

Python's dataclasses don't have this at all — you always write `p.x`. JavaScript has object destructuring (`const { x, y } = point`), which is the closest analogy. OCaml's record patterns work the same way. Rust is strict: the field names must match exactly, but you only pay for what you use.

The key insight: destructuring works *anywhere* a pattern is valid — `let`, `match`, function parameters, `if let`, `for` loops.

## How It Works in Rust

```rust
struct Point { x: f64, y: f64 }
struct Person { name: String, age: u32, email: String }

// Function parameter destructuring — no body boilerplate
fn distance(Point { x, y }: &Point) -> f64 {
    (x * x + y * y).sqrt()  // x and y are directly in scope
}

// Skip fields you don't need with ..
fn greet(Person { name, age, .. }: &Person) -> String {
    format!("Hello {}, age {}", name, age)  // email not mentioned
}

// Destructuring in match + guard
fn classify(person: &Person) -> &'static str {
    match person {
        Person { age, .. } if *age < 18 => "minor",
        Person { age, .. } if *age < 65 => "adult",
        _                               => "senior",
    }
}

// Rename a field: `x: local_name`
let Point { x: px, y: py } = some_point;

// Nested struct destructuring in one let
struct Rect { tl: Point, br: Point }
let Rect { tl: Point { x: x1, y: y1 }, br: Point { x: x2, y: y2 } } = r;
```

## What This Unlocks

- **Zero-cost clarity in function signatures** — destructure parameters directly, no `.field` inside the body.
- **Surgical `match` arms** — pull out only the fields you care about, add guards, ignore the rest with `..`.
- **Nested unpacking in one expression** — `Rect { tl: Point { x, y }, .. }` reaches arbitrarily deep without intermediate bindings.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Syntax | `{ x; y; _ }` | `{ x, y, .. }` |
| Field rename | `{ x = px; _ }` | `{ x: px, .. }` |
| Exhaustiveness | Warning if field omitted without `_` | Compile error; use `..` to skip |
| In function params | Yes — `let f { x; _ } = ...` | Yes — `fn f(Point { x, .. }: &Point)` |
| Works in `let` | Yes | Yes |

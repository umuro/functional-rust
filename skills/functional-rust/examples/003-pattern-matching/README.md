# 003: Pattern Matching

**Difficulty:** ⭐⭐  **Level:** Intermediate

Replace if/else chains with exhaustive matching — and let the compiler tell you when you missed a case.

## The Problem This Solves

Imagine you're handling different types of shapes, network packets, or UI events. The traditional approach: a chain of `if type == "circle" ... else if type == "rectangle" ...`. This is fragile: add a new type, forget to update the handler, and nothing tells you until a bug surfaces in production.

Rust's `match` expression changes the contract: **the compiler checks that you've handled every case**. Add a new variant to your `enum`, and every `match` on that type becomes a compile error until you handle it. Miss a case — the program doesn't build.

This makes refactoring safe. It makes adding features explicit. It's one of the most practically useful things in Rust.

## The Intuition

```python
# Python (3.10+) — match/case exists, but no exhaustiveness check
def area(shape):
    match shape['type']:
        case 'circle':   return math.pi * shape['r'] ** 2
        case 'rectangle': return shape['w'] * shape['h']
        # Forgot triangles? Python doesn't notice. Returns None silently.
```

```java
// Java — instanceof checks, easy to miss a case
if (shape instanceof Circle c) { ... }
else if (shape instanceof Rectangle r) { ... }
// Add Triangle later? Nothing reminds you to handle it here.
```

```rust
// Rust — exhaustive match, compile error if you miss a variant
match shape {
    Shape::Circle(r)         => std::f64::consts::PI * r * r,
    Shape::Rectangle(w, h)   => w * h,
    Shape::Triangle(a, b, c) => { /* Heron's formula */ }
    // Miss any variant → compiler error: "non-exhaustive patterns"
}
```

## How It Works in Rust

Define your type as an `enum`:

```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),             // holds: radius
    Rectangle(f64, f64),     // holds: width, height
    Triangle(f64, f64, f64), // holds: three sides
}
```

Match on it — binding the inner values directly in the pattern:

```rust
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}
```

**Guard clauses** — add extra conditions inside patterns:

```rust
fn describe(shape: &Shape) -> String {
    match shape {
        Shape::Rectangle(w, h) if (w - h).abs() < f64::EPSILON => {
            format!("Square with side {w}")  // special case: equal sides
        }
        Shape::Rectangle(w, h) => format!("Rectangle {w}×{h}"),
        Shape::Circle(r) => format!("Circle with radius {r}"),
        Shape::Triangle(a, b, c) => format!("Triangle with sides {a}, {b}, {c}"),
    }
}
```

**Match anywhere** — not just in standalone statements, but inside `.map()`, closures, `if let`:

```rust
// Scale all shapes using match inside a closure
let scaled: Vec<Shape> = shapes.iter().map(|s| match s {
    Shape::Circle(r)         => Shape::Circle(r * 2.0),
    Shape::Rectangle(w, h)   => Shape::Rectangle(w * 2.0, h * 2.0),
    Shape::Triangle(a, b, c) => Shape::Triangle(a * 2.0, b * 2.0, c * 2.0),
}).collect();
```

## What This Unlocks

- **Safe enum dispatch** — handle every variant of a configuration, error type, or message
- **Exhaustive refactoring** — add a new enum variant, the compiler lists every match that needs updating
- **Replacing `instanceof` chains** — cleaner, safer, zero runtime cost

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Custom type | `type shape = Circle of float \| ...` | `enum Shape { Circle(f64), ... }` |
| Match expression | `match shape with \| Circle r -> ...` | `match shape { Shape::Circle(r) => ... }` |
| Exhaustiveness | Enforced by compiler | Enforced by compiler |
| Guard clauses | `when condition` | `if condition` (after pattern) |
| Binding names | `\| Circle r ->` (direct) | `Shape::Circle(r) =>` (qualified) |
| Nested match | Natural | Natural — `match` works in any expression |

📖 **[View on hightechmind.io →](https://hightechmind.io/rust/561-pattern-or)**

---

# 561: Or-Patterns — pat1 | pat2

**Difficulty:** 2  **Level:** Beginner

Match multiple patterns in a single arm using `|`. Reduces repetition when different patterns share the same action.

## The Problem This Solves

Without or-patterns, matching several variants with the same handler requires either duplicating the arm body or indirection through a helper function:

```rust
match color {
    Color::Red   => "warm",   // duplicate arms
    Color::Yellow=> "warm",   // same body — tedious and easy to miss one
    Color::Green => "cool",
    Color::Blue  => "cool",
    Color::Purple=> "mixed",
}
```

Or-patterns let you express "any of these → same action" directly in the pattern language, with no duplication.

## The Intuition

`pat1 | pat2` reads as "matches if pat1 OR pat2 matches." It's a logical OR on patterns — both branches share the same binding names and must bind the same types. The arm body executes once for whichever pattern matched.

Or-patterns work at any nesting level — inside tuples, enums, or other compound patterns. They also work in `if let`, `while let`, and `matches!`.

## How It Works in Rust

**Enum variant grouping:**

```rust
#[derive(Debug)]
enum Color { Red, Green, Blue, Yellow, Purple }

fn classify(c: &Color) -> &'static str {
    match c {
        Color::Red | Color::Yellow  => "warm",   // two variants, one arm
        Color::Green | Color::Blue  => "cool",
        Color::Purple               => "mixed",
    }
}
```

**`matches!` macro — boolean test with or-patterns:**

```rust
fn is_primary(c: &Color) -> bool {
    matches!(c, Color::Red | Color::Green | Color::Blue)
}
```

**Integer or-patterns:**

```rust
fn describe_number(n: i32) -> &'static str {
    match n {
        0 | 1         => "tiny",
        2 | 3 | 4     => "small",
        5..=9         => "medium",
        _             => "large",
    }
}
```

**Nested or-patterns (inside compound patterns):**

```rust
enum Shape { Circle(f64), Square(f64), Triangle(f64, f64) }

fn has_single_param(s: &Shape) -> bool {
    matches!(s, Shape::Circle(_) | Shape::Square(_))
}

// Or in a full match with data binding:
fn area_approx(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) | Shape::Square(r) => r * r, // both bind `r`
        Shape::Triangle(base, height)       => base * height / 2.0,
    }
}
```

**`if let` with or-patterns:**

```rust
let c = Color::Red;
if let Color::Red | Color::Yellow = c {
    println!("warm color");
}
```

## What This Unlocks

- **DRY match arms** — when five variants map to one of two outcomes, write two arms instead of five. The intent is immediately clear.
- **Type-safe grouping** — or-patterns are checked by the exhaustiveness checker. Add a new variant to the enum and the compiler points you to every match that needs updating.
- **Clean `matches!` predicates** — express "is this one of these values?" as a single boolean expression usable in `filter`, `assert`, and `if` conditions.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Multiple patterns, one arm | `| pat1 | pat2 ->` (leading `|` optional) | `pat1 \| pat2 =>` — same syntax, middle `\|` |
| Binding in or-patterns | Each alternative must bind same names/types | Same rule — all alternatives must produce same bindings |
| `matches!` equivalent | N/A (inline match or `function`) | `matches!(expr, pat1 \| pat2)` → `bool` |
| Exhaustiveness | Compiler checks all constructors covered | Same — exhaustiveness checked across all or-pattern arms |
| Range patterns | `0..9 ->` (exclusive) | `0..=9 =>` (inclusive) — combinable with `\|` |

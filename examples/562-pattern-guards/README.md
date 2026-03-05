# 562: Match Guards with if Conditions

**Difficulty:** 2  **Level:** Beginner

A match guard adds an `if condition` after a pattern. The arm only fires if both the pattern matches *and* the condition is true. Guards handle cases that pattern syntax alone can't express.

## The Problem This Solves

Patterns match on structure and values, but not on arbitrary predicates. A pattern can match `Shape::Circle(r)` — it can't match "a Circle whose radius is between 0.1 and 1.0." That requires a guard.

Without guards, you'd need nested `if` statements inside each match arm, or a helper function, losing the clean exhaustiveness checking of `match`:

```rust
match shape {
    Shape::Circle(r) => {
        if r <= 0.0 { "invalid" }
        else if r < 1.0 { "tiny" }
        else { "circle" }  // nested ifs — readable but breaks the match structure
    }
    ...
}
```

Guards keep the branching in the `match` expression itself, where the exhaustiveness checker can see it.

## The Intuition

`pattern if condition => arm` reads as "if the value looks like `pattern` AND satisfies `condition`, execute this arm." If the pattern matches but the guard is false, the match continues to the next arm — like the pattern didn't match at all.

Guards are evaluated *after* the pattern binds its variables, so you can use bound names inside the condition: `Shape::Circle(r) if *r < 1.0`.

**Important:** A guard that doesn't cover all cases for a pattern makes that pattern non-exhaustive for the match checker's purposes. You often need a final fallback arm.

## How It Works in Rust

**Shape classification with guards:**

```rust
#[derive(Debug)]
enum Shape { Circle(f64), Rect(f64, f64) }

fn describe(s: &Shape) -> &'static str {
    match s {
        Shape::Circle(r) if *r <= 0.0  => "invalid",   // guard: radius non-positive
        Shape::Circle(r) if *r < 1.0   => "tiny circle",
        Shape::Circle(_)                => "circle",     // catch-all for Circle
        Shape::Rect(w, h) if w == h     => "square",    // equal sides
        Shape::Rect(w, h) if w > h      => "wide",
        Shape::Rect(_, _)               => "tall",       // catch-all for Rect
    }
}
```

**Grade calculation — guard on a binding:**

```rust
fn grade(score: u32) -> char {
    match score {
        n if n >= 90 => 'A',
        n if n >= 80 => 'B',
        n if n >= 70 => 'C',
        n if n >= 60 => 'D',
        _            => 'F',
    }
}
// Note: guards are checked top-to-bottom — order matters
// n >= 90 is checked first, so 95 correctly gets 'A', not 'B'
```

**Guards with or-patterns:**

```rust
// Guard applies to BOTH alternatives in an or-pattern
fn classify_temp(celsius: f64) -> &'static str {
    match celsius as i32 {
        n if n < 0         => "freezing",
        0 | 1 | 2 | 3 if true => "near zero",  // or: use range
        n if n < 20        => "cold",
        n if n < 30        => "warm",
        _                  => "hot",
    }
}
```

**Guards with struct patterns:**

```rust
struct Point { x: i32, y: i32 }

fn quadrant(p: &Point) -> &'static str {
    match p {
        Point { x, y } if *x > 0 && *y > 0 => "Q1",
        Point { x, y } if *x < 0 && *y > 0 => "Q2",
        Point { x, y } if *x < 0 && *y < 0 => "Q3",
        Point { x, y } if *x > 0 && *y < 0 => "Q4",
        _                                   => "on axis",
    }
}
```

## What This Unlocks

- **Precise case analysis** — classify continuous values (scores, temperatures, prices) without losing the exhaustiveness checking that raw `if-else` chains lack.
- **State-dependent dispatch** — match on both structure and runtime state in one expression. A message handler can dispatch on enum variant AND session state with a single `match`.
- **Clean validation in parsers** — `Token::Number(n) if n > 0 => valid_id(n)` handles structural and value constraints in one step.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Match guard syntax | `pattern when condition ->` | `pattern if condition =>` |
| Guard variable access | Bound names available | Same — bound names usable in the guard condition |
| Guard + or-pattern | `(p1 \| p2) when cond ->` | `p1 \| p2 if cond =>` — guard applies to both |
| Exhaustiveness | Guards make arms non-exhaustive; compiler warns | Same — a guarded arm doesn't cover its pattern fully |
| Guard evaluation order | Top-to-bottom; first match wins | Same — ordered evaluation, first matching arm + guard wins |

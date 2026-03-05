📖 **[View on hightechmind.io →](https://hightechmind.io/rust/566-pattern-nested)**

---

# 566: Nested Pattern Matching

**Difficulty:** 2  **Level:** Beginner

Match deeply nested structures in one expression — no intermediate bindings, no nested ifs.

## The Problem This Solves

Imagine a card game. You want to check: is this `Hand::One`, and does it contain an `Ace`, and is the suit specifically `Spades`? In a language without nested patterns, you write three levels of if/instanceof checks. You pull `.r` then check its type, then pull `.s`, and the logic dissolves into scaffolding.

With nested patterns, you write the shape of what you're looking for and Rust either matches it or doesn't. The logic reads like a description: "a hand of one card, where the rank is Ace and the suit is Spades." No intermediaries.

The second problem is combinatorial matching. You want to handle `(Some(x), Some(y))` differently from `(Some(x), None)`, `(None, Some(y))`, and `(None, None)`. Nested if-let chains get unwieldy fast. A tuple pattern in a `match` handles all four cases in four arms, each self-describing.

## The Intuition

Nested patterns work because patterns can be composed: any place you'd write a value, you can write a pattern instead. `Hand::One(Card { r: Rank::A, s: Suit::S })` is just a pattern nested inside a pattern nested inside a pattern. Rust evaluates them inside-out and either all match or none do.

Python's `match` (3.10+) has this too, though it uses class patterns. OCaml has it natively and it's one of the most productive features of the language. The concept is the same: instead of querying an object layer by layer, you describe the shape you want and let the runtime/compiler check it in one shot.

The key insight: Rust can match on tuples too, so `match (opt1, opt2) { (Some(x), Some(y)) => ... }` gives you a 2D dispatch table in a few clean arms.

## How It Works in Rust

```rust
enum Rank { N(u8), J, Q, K, A }
enum Suit { C, D, H, S }
struct Card { r: Rank, s: Suit }
enum Hand { Empty, One(Card), Two(Card, Card) }

fn describe(h: &Hand) -> &'static str {
    match h {
        // Nested three levels deep in one arm
        Hand::One(Card { r: Rank::A, s: Suit::S }) => "ace of spades!",

        // Match the outer shape, ignore inner details
        Hand::One(Card { r: Rank::A, .. })          => "an ace",

        // Pattern guard on destructured inner fields
        Hand::Two(Card { r: r1, .. }, Card { r: r2, .. }) if r1 == r2 => "a pair",

        Hand::Two(_, _) => "two cards",
        Hand::Empty     => "nothing",
    }
}

// Tuple patterns: dispatch on two values simultaneously
match (left, right) {
    (Some(x), Some(y)) => format!("both: {} {}", x, y),
    (Some(x), None)    => format!("only left: {}", x),
    (None, Some(y))    => format!("only right: {}", y),
    (None, None)       => "neither".into(),
}

// Nested array/slice patterns
if let [[a, _], [_, d]] = [[1, 2], [3, 4]] {
    println!("Diagonal: {} {}", a, d);  // a=1, d=4
}
```

## What This Unlocks

- **Readable multi-condition branches** — describe the exact shape you need rather than a chain of guards.
- **Tuple dispatch** — match on multiple independent values in a single `match`, exhaustively.
- **Deeply nested extraction** — pull values out of `Option<Result<Vec<T>>>` style structures in one arm.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Nested record patterns | `{ r = A; s = S }` | `Card { r: Rank::A, s: Suit::S }` |
| Tuple match | `match (a, b) with (Some x, Some y) ->` | `match (a, b) { (Some(x), Some(y)) => }` |
| Pattern guard | `when r1 = r2` | `if r1 == r2` |
| Skip fields | `{ r = A; _ }` | `Card { r: Rank::A, .. }` |
| Array pattern | `[a; _; b]` (list) | `[a, _, b]` (array/slice) |

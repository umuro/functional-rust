# 564: Enum Variant Matching

**Difficulty:** 2  **Level:** Beginner

Match on every variant of an enum — the compiler ensures no case is missed.

## The Problem This Solves

Without enum matching, branching on "what kind of thing is this?" means runtime type tags, instanceof chains, or string-keyed dispatch maps. You might forget a case. You might add a new case and not notice the six places in your code that need updating.

In most languages, adding a new message type to a message-passing system means grep-and-pray. You search for every switch statement, hope you don't miss one, and find out at runtime when something falls into the default branch silently.

Rust's `match` on enums is exhaustive by design. The compiler refuses to compile a match that doesn't cover every variant. Add `Message::Ping` to your enum and every existing match stops compiling, guiding you straight to each callsite that needs updating.

## The Intuition

Think of this as a switch statement that the compiler actually enforces. Java's `switch` on enums is close, but it lets you omit cases and silently fall through to a default. Python's `match` (3.10+) has structural matching but no exhaustiveness guarantee.

The killer insight is that enum variants can *carry data*. `Message::Move { x, y }` isn't just a tag — it bundles the coordinates with the variant. In one match arm you get both the dispatch and the destructuring. That's something C's enum + union can't give you safely, and OOP inheritance makes you work much harder for.

Recursive enums (like the expression tree in this example) are the natural representation for trees, ASTs, JSON, and anything that's "one of several shapes, possibly nested."

## How It Works in Rust

```rust
#[derive(Debug, Clone)]
enum Expr {
    Num(f64),                    // leaf: a number
    Add(Box<Expr>, Box<Expr>),   // node: left + right
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Var(String),                 // variable lookup
}

// Every arm handles one variant; compiler rejects if any are missing
fn eval(env: &[(&str, f64)], e: &Expr) -> f64 {
    match e {
        Expr::Num(n)    => *n,
        Expr::Add(l, r) => eval(env, l) + eval(env, r),
        Expr::Mul(l, r) => eval(env, l) * eval(env, r),
        Expr::Neg(e)    => -eval(env, e),
        Expr::Var(s)    => env.iter()
                              .find(|&&(k, _)| k == s)
                              .map(|&(_, v)| v)
                              .unwrap_or(0.0),
    }
}
// Try removing any arm — the compiler will tell you exactly which variant
// is unhandled. No runtime surprises.
```

## What This Unlocks

- **Safe recursive data structures** — `Box<Expr>` breaks the size cycle; you get full ASTs, JSON trees, linked lists.
- **Compiler-guided refactoring** — add a variant, find every match that needs updating immediately at compile time.
- **Combined dispatch + destructuring** — `Mul(l, r)` gives you the variant tag *and* the inner values in one arm.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Syntax | `type expr = Num of float \| Add of expr * expr` | `enum Expr { Num(f64), Add(Box<Expr>, Box<Expr>) }` |
| Exhaustiveness | Warning (configurable to error) | Hard compile error |
| Recursive type | Native — no Box needed | Requires `Box<T>` for heap indirection |
| Namespace | Variants unqualified by default | `Expr::Num` (use `use Expr::*` to shorten) |
| Pattern syntax | `Num n` | `Num(n)` or `Add(l, r)` |

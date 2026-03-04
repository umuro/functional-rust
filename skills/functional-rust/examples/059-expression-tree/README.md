# 059: Expression Tree

**Difficulty:** 2  **Level:** Beginner

Represent arithmetic expressions as a recursive enum and evaluate them with structural recursion.

## The Problem This Solves

Calculators, compilers, query engines, and rule systems all need to represent expressions: things that can be composed from smaller expressions. `(1 + 2) * (10 - 4)` is an expression. Its sub-expressions (`1 + 2`) and (`10 - 4`) are expressions too, composed of leaf values.

A flat `String` or `Vec` can't capture this structure. You need a recursive data type: an expression is either a leaf (a number) or a node (an operator applied to two sub-expressions). This is an **Abstract Syntax Tree** — the core data structure of every compiler.

Without a proper recursive type, you end up with string parsing, switch statements on magic strings, or parallel arrays — all fragile. The enum makes the structure explicit and the compiler enforces it.

## The Intuition

An expression tree is a tree where:
- Leaves hold values (`Num(3.0)`)
- Branches hold operators and point to their operands (`Add(left, right)`)

Evaluating the tree is structural recursion: to evaluate `Add(l, r)`, evaluate `l`, evaluate `r`, add the results. The structure of the code mirrors the structure of the data.

## How It Works in Rust

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),  // Box<Expr> required: Expr cannot have infinite size
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    // Convenience constructors — hide Box::new boilerplate
    pub fn num(n: f64) -> Self { Expr::Num(n) }
    pub fn new_add(l: Expr, r: Expr) -> Self { Expr::Add(Box::new(l), Box::new(r)) }

    // Structural recursion: eval mirrors the shape of the type
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Num(n)      => *n,
            Expr::Add(l, r)   => l.eval() + r.eval(),
            Expr::Sub(l, r)   => l.eval() - r.eval(),
            Expr::Mul(l, r)   => l.eval() * r.eval(),
            Expr::Div(l, r)   => l.eval() / r.eval(),
        }
    }
}

// Build: (1 + 2) * (10 - 4) = 18
let e = Expr::new_mul(
    Expr::new_add(Expr::num(1.0), Expr::num(2.0)),
    Expr::new_sub(Expr::num(10.0), Expr::num(4.0)),
);
assert!((e.eval() - 18.0).abs() < f64::EPSILON);
```

`Box<Expr>` provides one level of heap indirection, giving `Expr` a known size at compile time. Without it, `Expr` would contain `Expr` which would contain `Expr` — infinite size.

## What This Unlocks

- **Compiler front-ends** — this is the AST pattern every language implementation uses.
- **Rule engines and query builders** — represent composable conditions as expression trees.
- **Safe error handling improvement** — `eval_safe()` returning `Result<f64, String>` for divide-by-zero shows how Rust improves on OCaml's silent `infinity`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive type | Implicit heap allocation | `Box<Expr>` required for known size |
| Constructors | `Add (Num 1.0, Num 2.0)` | `Expr::new_add(Expr::num(1.0), Expr::num(2.0))` |
| Eval function | `let rec eval = function ...` | `impl Expr { fn eval(&self) -> f64 }` |
| Display | Manual `to_string` function | `impl Display for Expr` — enables `format!("{e}")` |
| Division safety | Returns `infinity` silently | Can return `Result` for explicit error |

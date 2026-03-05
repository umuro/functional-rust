📖 **[View on hightechmind.io →](https://hightechmind.io/rust/593-interpreter-pattern)**

---

# 593: Interpreter Pattern

**Difficulty:** 3  **Level:** Intermediate

Define a grammar as an enum (AST) and write multiple interpreters over it — evaluator, pretty-printer, and optimizer are all separate functions.

## The Problem This Solves

You need to process structured data in multiple ways: evaluate a math expression, pretty-print it, optimize it, compile it to bytecode. In OOP, the Visitor pattern solves this, but it requires a double-dispatch class hierarchy — boilerplate-heavy and fragile when you add new variants.

The functional approach is simpler: define the grammar as a recursive enum (the AST), then write one function per interpretation. Each interpreter is a recursive `match` over the enum. Adding a new interpreter (e.g., a type-checker) means adding one function — no existing code changes.

This is how real compilers are built. Rust's own compiler represents the language as a tree of enum variants and passes it through dozens of transformations. The power is in the clean separation: the grammar (enum) is defined once; behaviors (interpreters) are defined separately and can be added independently.

## The Intuition

A Rust `enum` + recursive `match` *is* the interpreter pattern — no abstract classes, no visitor infrastructure, no interface hierarchy: just data and functions that transform it. The trade-off: adding a new AST variant requires updating all interpreters; adding a new interpreter requires no changes to existing code (the opposite trade-off from OOP's open/closed principle per axis).

## How It Works in Rust

```rust
// The grammar — defined once
#[derive(Debug, Clone)]
enum Expr {
    Num(f64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

use std::collections::HashMap;

// Interpreter 1: evaluate to a number
fn eval(expr: &Expr, env: &HashMap<String, f64>) -> f64 {
    match expr {
        Expr::Num(n)    => *n,
        Expr::Var(name) => *env.get(name).unwrap_or(&0.0),
        Expr::Add(l, r) => eval(l, env) + eval(r, env),
        Expr::Mul(l, r) => eval(l, env) * eval(r, env),
    }
}

// Interpreter 2: pretty-print (no env needed)
fn pretty(expr: &Expr) -> String {
    match expr {
        Expr::Num(n)    => n.to_string(),
        Expr::Var(name) => name.clone(),
        Expr::Add(l, r) => format!("({} + {})", pretty(l), pretty(r)),
        Expr::Mul(l, r) => format!("({} * {})", pretty(l), pretty(r)),
    }
}

// Interpreter 3: constant folding optimizer
fn optimize(expr: Expr) -> Expr {
    match expr {
        Expr::Add(l, r) => match (optimize(*l), optimize(*r)) {
            (Expr::Num(a), Expr::Num(b)) => Expr::Num(a + b),  // fold constants
            (l, r) => Expr::Add(Box::new(l), Box::new(r)),
        },
        other => other,
    }
}

// All three work on the same AST — no changes to Expr needed
let ast = Expr::Add(Box::new(Expr::Num(1.0)), Box::new(Expr::Num(2.0)));
assert_eq!(eval(&ast, &HashMap::new()), 3.0);
assert_eq!(pretty(&ast), "(1 + 2)");
```

## What This Unlocks

- **DSL engines**: define a query language as an enum, write SQL and NoSQL backends as separate interpreters.
- **Multi-target compilers**: one AST, multiple backends (WASM, native, bytecode) — each is an interpreter.
- **Testing**: test the optimizer and evaluator independently; the AST is pure data, easy to construct in tests.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| AST definition | Recursive variant type | Recursive `enum` with `Box` for indirection |
| Interpreter | Recursive function + pattern match | Recursive `fn` + `match` |
| Environment | Association list / `Map` | `HashMap<String, f64>` |
| Adding interpreter | New function | New function — no existing code changes |
| Adding variant | Update all interpreters | Update all interpreters (same trade-off) |
| OOP equivalent | Visitor pattern | Not needed — `match` handles dispatch |

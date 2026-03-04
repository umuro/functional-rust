# 587: Visitor Pattern via Match

**Difficulty:** 3  **Level:** Intermediate

Implement multiple independent traversals over a recursive data structure — each traversal is a function, not a class.

## The Problem This Solves

The classic object-oriented Visitor pattern requires a `Visitor` interface, an `accept(visitor)` method on every node type, and a concrete class for each operation. To add evaluation, pretty-printing, and literal collection to an expression tree, you write three visitor classes, each with five `visit_*` methods. That's fifteen methods, two interfaces, and a pile of boilerplate before you've written any real logic.

The alternative in OOP — just adding methods to each node type — breaks open/closed: every new operation means touching every class. Neither solution is clean.

Rust's enum + match inverts this. The data structure is one enum. Each operation is one function. Adding a new operation means adding one function. The compiler guarantees every variant is handled in every function. It's the ideal tradeoff for "stable structure, many operations."

## The Intuition

An enum represents a closed set of variants — you control the full list. A recursive function over that enum is a "visitor" in the traditional sense, but without any of the ceremony. The `match` dispatches to the right case, the recursion handles nesting, and the compiler verifies completeness.

OCaml built its entire standard library around this pattern — `type expr = Lit of float | Add of expr * expr | ...` with `let rec eval = function ...` for each operation. It's idiomatic ML. Rust inherited this style directly.

The key insight: each "visitor" function has the same structure — match on the variant, handle the leaf, recurse on the children. The structure is identical; only the operation differs. Once you see it, you can write a new traversal in minutes.

## How It Works in Rust

```rust
#[derive(Debug, Clone)]
enum Expr {
    Lit(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

// Visitor 1: evaluate the tree
fn eval(e: &Expr) -> f64 {
    match e {
        Expr::Lit(n)   => *n,
        Expr::Add(l,r) => eval(l) + eval(r),
        Expr::Sub(l,r) => eval(l) - eval(r),
        Expr::Mul(l,r) => eval(l) * eval(r),
        Expr::Div(l,r) => eval(l) / eval(r),
    }
}

// Visitor 2: count operations (non-leaf nodes)
fn count_ops(e: &Expr) -> usize {
    match e {
        Expr::Lit(_) => 0,
        // Or-pattern for all binary ops — shared recursive structure
        Expr::Add(l,r) | Expr::Sub(l,r) | Expr::Mul(l,r) | Expr::Div(l,r) =>
            1 + count_ops(l) + count_ops(r),
    }
}

// Visitor 3: pretty-print to string
fn pretty(e: &Expr) -> String {
    match e {
        Expr::Lit(n)   => format!("{}", n),
        Expr::Add(l,r) => format!("({}+{})", pretty(l), pretty(r)),
        // ... each variant gets its operator
    }
}

// Visitor 4: collect all leaf values
fn collect_lits(e: &Expr) -> Vec<f64> {
    match e {
        Expr::Lit(n)   => vec![*n],
        Expr::Add(l,r) | Expr::Sub(l,r) | Expr::Mul(l,r) | Expr::Div(l,r) => {
            let mut v = collect_lits(l);
            v.extend(collect_lits(r));
            v
        }
    }
}
```

## What This Unlocks

- **Zero-boilerplate traversals** — each new operation is one function; no interfaces, no accept/visit plumbing.
- **Compiler-enforced completeness** — add `Expr::Pow` and every visitor function stops compiling until you handle it.
- **Or-patterns for shared structure** — `Add(l,r) | Sub(l,r) | Mul(l,r)` shares identical recursive logic across variants.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Data structure | `type expr = ...` | `enum Expr { ... }` |
| Each traversal | `let rec eval = function ...` | `fn eval(e: &Expr) -> T { match e { ... } }` |
| Or-patterns | `\| Add(l,r) \| Sub(l,r) -> same_logic` | `Expr::Add(l,r) \| Expr::Sub(l,r) =>` |
| OOP alternative | Visitor pattern with interfaces | Not needed — functions are first-class |
| Adding a variant | Compile error in all visitors | Same — compiler finds every match |

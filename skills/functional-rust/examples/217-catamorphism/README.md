# 217: Catamorphism — The Universal Fold

**Difficulty:** ⭐⭐⭐  **Category:** Recursion Schemes

You've been writing catamorphisms your whole career — they just had a boring name before: "fold".

## The Problem This Solves

Every function that consumes a recursive structure follows the same shape. You visit the leaves, do something, then combine results on the way up. You've written this a dozen times:

```rust
fn eval(e: &Expr) -> i64 { ... visits children, combines ... }
fn show(e: &Expr) -> String { ... visits children, combines ... }
fn optimize(e: &Expr) -> Expr { ... visits children, combines ... }
fn free_vars(e: &Expr) -> Vec<String> { ... visits children, combines ... }
```

The traversal logic (recurse into children, collect results) is identical across all of them. Only the "what to do with one node" differs.

There's also a subtler problem: when you add a new variant to your AST — say `NegF` or `IfZeroF` — you have to update every function. Miss one and it's a compile error. Update them all and it's tedious boilerplate. Neither is fun.

Catamorphisms solve this by making "traverse recursively" a single reusable function called `cata`, and making "what to do at one node" a small data-free function called an **algebra**. You write one algebra per operation — no recursion, no traversal boilerplate. `cata` handles everything else.

The name "catamorphism" comes from Greek "kata" (downward) — it collapses a structure from the bottom up. But you don't need to know that to use it. You just need to know: **catamorphism = fold**. The same fold you use on lists, generalized to any recursive type.

## The Intuition

`Vec::fold` already works this way. You give it:
1. A starting value (the "base case")
2. A function to combine one element with the accumulated result

It handles the iteration. You just provide the logic.

`cata` is the same idea, but for tree-shaped structures instead of flat lists. You give it:
1. An **algebra** — a function that handles *one node*, with children already processed
2. A structure to fold over

It handles the traversal. You provide the logic.

Here's the key mental model: when your algebra sees `AddF(a, b)`, both `a` and `b` are already the *result* of folding the subtrees — not the original child nodes. The traversal happened before your function was called. You just combine two already-computed results.

```
Tree:           add(lit(1), mul(lit(2), lit(3)))

Cata visits:    LitF(1) → eval_alg → 1
                LitF(2) → eval_alg → 2
                LitF(3) → eval_alg → 3
                MulF(2, 3) → eval_alg → 6   ← children are already i64s!
                AddF(1, 6) → eval_alg → 7   ← children are already i64s!
```

Your algebra never sees the original tree — it only sees values that have already been processed. It's purely bottom-up.

## How It Works in Rust

**The functor and fix point** (same as example 216, extended with more node types):

```rust
enum ExprF<A> {
    LitF(i64),
    AddF(A, A),
    MulF(A, A),
    NegF(A),
    IfZeroF(A, A, A),  // condition, then-branch, else-branch
}

struct Fix(Box<ExprF<Fix>>);

fn cata<A>(alg: &dyn Fn(ExprF<A>) -> A, Fix(f): &Fix) -> A {
    alg(f.map_ref(|child| cata(alg, child)))
}
```

**Writing algebras — no recursion, just one layer:**

```rust
// Algebra 1: evaluate to a number
fn eval_alg(e: ExprF<i64>) -> i64 {
    match e {
        ExprF::LitF(n)          => n,
        ExprF::AddF(a, b)       => a + b,         // a, b already evaluated
        ExprF::MulF(a, b)       => a * b,
        ExprF::NegF(a)          => -a,
        ExprF::IfZeroF(c, t, e) => if c == 0 { t } else { e },
    }
}

// Algebra 2: pretty-print to a string
fn show_alg(e: ExprF<String>) -> String {
    match e {
        ExprF::LitF(n)          => n.to_string(),
        ExprF::AddF(a, b)       => format!("({a} + {b})"),   // a, b already strings
        ExprF::MulF(a, b)       => format!("({a} * {b})"),
        ExprF::NegF(a)          => format!("(-{a})"),
        ExprF::IfZeroF(c, t, e) => format!("(if0 {c} then {t} else {e})"),
    }
}
```

**Algebras can return any type — including the structure itself!**

An algebra returning `Fix` is a *tree transformation* — it rewrites the tree instead of collapsing it. This is how you write optimization passes:

```rust
// Constant folding: simplify obvious patterns
// Note: uses box patterns (nightly feature); stable Rust needs manual matching
fn opt_alg(e: ExprF<Fix>) -> Fix {
    match &e {
        // 0 + x = x
        ExprF::AddF(Fix(box ExprF::LitF(0)), b) => b.clone(),
        // x * 0 = 0
        ExprF::MulF(Fix(box ExprF::LitF(0)), _) => lit(0),
        // -(-x) = x
        ExprF::NegF(Fix(box ExprF::NegF(inner))) => inner.clone(),
        // everything else stays as-is
        _ => Fix(Box::new(e)),
    }
}
```

**Algebras over expressions with variables:**

```rust
// A different functor — expressions with variables
enum VarExprF<A> {
    VLit(i64),
    VVar(String),  // a named variable
    VAdd(A, A),
}

// Collect all free variable names
fn free_vars_alg(e: VarExprF<Vec<String>>) -> Vec<String> {
    match e {
        VarExprF::VLit(_)    => vec![],           // no variables here
        VarExprF::VVar(s)    => vec![s],           // found one!
        VarExprF::VAdd(a, b) => { let mut a = a; a.extend(b); a }, // merge both sides
    }
}
```

**Running it all:**

```rust
let e = add(lit(1), mul(lit(2), neg(lit(3))));

assert_eq!(cata(&eval_alg, &e), -5);
assert_eq!(cata(&show_alg, &e), "(1 + (2 * (-3)))");

let ve = vadd(vvar("x"), vadd(vlit(1), vvar("y")));
assert_eq!(cata_v(&free_vars_alg, &ve), vec!["x", "y"]);
```

## What This Unlocks

- **Add new analyses without touching existing code.** Want to count literals? Count operators? Compute depth? Each is one small algebra, no recursion. Existing algebras are untouched.
- **Tree transformations for free.** An algebra that produces `Fix` is a rewriting pass — constant folding, dead code elimination, normalization. Compose them with `cata` and you have a compiler pipeline.
- **Correctness by construction.** Algebras are pure functions on flat `ExprF<A>` values. They're easy to test without building a tree at all.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Algebra type | `'a expr_f -> 'a` | `ExprF<A> -> A` (identical idea) |
| Calling cata | `cata eval_alg expr` | `cata(&eval_alg, &expr)` |
| Box patterns (for opt) | Pattern matching on constructors directly | Requires `#![feature(box_patterns)]` or manual nested `match` |
| String building | `^` string concat operator | `format!` macro |
| Closure as algebra | First-class, clean syntax | `&dyn Fn(ExprF<A>) -> A` — trait object |

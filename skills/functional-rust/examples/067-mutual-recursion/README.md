# 067: Mutual Recursion

**Difficulty:** 1  **Level:** Beginner

Two functions that call each other — Rust handles it naturally, OCaml needs the `and` keyword.

## The Problem This Solves

Some algorithms are most naturally expressed as two functions that call each other. The classic example: `is_even(n)` returns true if `n == 0`, otherwise delegates to `is_odd(n - 1)`. And `is_odd(n)` returns false if `n == 0`, otherwise delegates to `is_even(n - 1)`. Neither function can be defined without the other.

More practically: an expression evaluator might have `eval_expr` that handles addition and multiplication, and `eval_mul` that handles a special case of multiplication with additional logic. They call each other. Or a parser might have `parse_expr` calling `parse_term` and `parse_term` calling `parse_factor` calling back into `parse_expr` for grouped expressions.

OCaml requires explicit syntax (`let rec ... and ...`) to co-define mutually recursive functions because definitions are processed sequentially. Understanding this difference helps you reason about *when* forward declarations matter and why Rust doesn't need them.

## The Intuition

**Mutual recursion** means two (or more) functions form a cycle in their call graph: A calls B, and B calls A.

In OCaml, the compiler processes definitions top-to-bottom. If you write `let is_even` first, then `let is_odd`, the body of `is_even` can't reference `is_odd` because it hasn't been defined yet. The `and` keyword co-defines them simultaneously: `let rec is_even = ... and is_odd = ...`.

In Rust, all items (functions, structs, impl blocks) in a module are visible to each other regardless of order. The compiler does a full pass over all items before resolving names. So `is_even` can call `is_odd` even if `is_odd` is defined further down the file. No special syntax needed.

**Stack depth warning**: the naive mutual recursion for `is_even(1_000_000)` would make 1,000,000 recursive calls and overflow the stack. For deep recursion, use the iterative version: `n % 2 == 0`. Or use a **trampoline** (see the tail-recursive accumulator example 068 for the pattern).

## How It Works in Rust

```rust
// No special syntax needed — these two functions can call each other freely
pub fn is_even(n: u32) -> bool {
    match n {
        0 => true,
        n => is_odd(n - 1),   // calls is_odd — defined below, that's fine
    }
}

pub fn is_odd(n: u32) -> bool {
    match n {
        0 => false,
        n => is_even(n - 1),  // calls is_even — defined above, also fine
    }
}
```

For production use (avoids stack overflow):
```rust
pub fn is_even_iter(n: u32) -> bool { n % 2 == 0 }
```

Mutual recursion over algebraic data types — an expression evaluator where `eval_expr` and `eval_mul` call each other:
```rust
pub fn eval_expr(e: &Expr) -> i32 {
    match e {
        Expr::Lit(n) => *n,
        Expr::Add(l, r) => eval_expr(l) + eval_expr(r),
        Expr::Mul(l, r) => eval_mul(l, r),   // delegate to specialized handler
    }
}

fn eval_mul(l: &Expr, r: &Expr) -> i32 {
    eval_expr(l) * eval_expr(r)   // calls back into eval_expr
}
```

This pattern is common when you want to separate concerns within a recursive evaluator — different arms of the match delegate to different helpers, and those helpers may recurse back to the main function.

## What This Unlocks

- **Recursive descent parsers**: `parse_expr` → `parse_term` → `parse_factor` → `parse_expr` forms a natural mutual recursion that mirrors the grammar.
- **State machines**: `state_a` transitions to `state_b`, `state_b` transitions to `state_a` — mutual recursion models the cycle directly.
- **Separation of concerns in evaluators**: split a large `eval` match into specialized helpers that can call back into `eval` for recursive cases.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutual recursion syntax | `let rec f = ... and g = ...` — required | No special syntax — functions see each other automatically |
| Why the difference | Definitions processed top-to-bottom | All module items resolved in one pass |
| Stack safety | Same risk of overflow | Same; neither guarantees TCO for mutual recursion |
| Trampoline | Manual or via continuation | Manual; same pattern (see example 068) |

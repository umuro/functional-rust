[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 079 — Lambda Calculus Interpreter
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement a small-step interpreter for a minimal lambda calculus with integers, variables, lambda abstraction, function application, and addition. The interpreter evaluates `Expr` trees against an environment, returning `Value` (integer or closure) — and compare the implementation with OCaml's natural match-based interpreter.

## Learning Outcomes

- Represent recursive expression trees using `Box<Expr>` for heap allocation
- Use `Result<Value, String>` for interpreter errors (unbound variable, type error)
- Understand why closures capture environments by clone in this design
- Trace evaluation of beta reduction through pattern matching on `VClosure`
- Map Rust's `enum` + `Box` ADT to OCaml's native recursive algebraic types
- Appreciate when `Rc<Expr>` would avoid deep clones in shared subtrees

## Rust Application

`Expr` and `Value` are recursive enums; `Box` breaks the infinite-size cycle. The environment `Env = Vec<(String, Value)>` is a simple association list searched in reverse (most-recent binding wins). Creating a closure captures the current `env` by clone — value semantics mean each closure owns a snapshot of its environment. Beta reduction pushes `(x, av)` onto the closure's saved environment and recurses. The `?` operator propagates `Err` cleanly through the recursive `eval` calls. The test helpers (`int`, `var`, `lam`, `app`, `add`) construct trees concisely.

## OCaml Approach

OCaml's native recursive types require no `Box`: `type expr = Int of int | Lam of string * expr | …`. The interpreter is a single `let rec eval env = function` with four cases. Closure creation captures the current `env` as an association list (a list value, so structural sharing is free). Variable lookup uses `List.assoc`. The OCaml version is terser because recursive types need no heap annotation, and closures capture by value naturally. Both implementations share the same logical structure.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Recursive types | `Box<Expr>` required | Native recursive ADT |
| Environment | `Vec<(String, Value)>` cloned | `(string * value) list` shared |
| Closure capture | Explicit `.clone()` of env | Implicit value copy |
| Error handling | `Result<Value, String>` | `failwith` (exception) |
| Lookup | `.iter().rev().find()` | `List.assoc` |
| Code length | ~60 lines | ~20 lines |

The fundamental logic is identical: match on the expression, recurse on sub-expressions, extend the environment for beta reduction. Rust's verbosity comes from explicit heap management and typed error propagation rather than any semantic difference.

## Exercises

1. Add a `Let(String, Box<Expr>, Box<Expr>)` variant representing `let x = e1 in e2`, and implement its evaluation as syntactic sugar for `App(Lam(x, e2), e1)`.
2. Add a `Bool(bool)` value type and an `If(Box<Expr>, Box<Expr>, Box<Expr>)` expression with evaluation.
3. Replace the `Vec<(String, Value)>` environment with `std::collections::HashMap<String, Value>` and benchmark the difference on deeply nested environments.
4. Implement call-by-name semantics: instead of evaluating the argument before passing it, pass the unevaluated `Expr` and evaluate only when a `Var` is looked up.
5. In OCaml, add a `Fix` (fixed-point combinator) expression to support recursion without a separate `let rec` and verify it can compute factorial.

📖 **[View on hightechmind.io →](https://hightechmind.io/rust/224-mutumorphism)**

---

# Mutumorphism — Genuinely Mutual Recursion
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Mutually recursive functions — `is_even` and `is_odd` depending on each other — are natural in mathematics but require mutual `let rec` in functional languages. A mutumorphism generalizes this: two folds that are mutually dependent, each using the other's result at each recursive step. Unlike zygomorphism (one feeds the other), mutumorphism has symmetric dependency. The classic example: `is_even(n)` uses `is_odd(n-1)` and vice versa.

## Learning Outcomes

- Understand mutumorphisms as mutually recursive catamorphisms
- Learn how `mutu` computes two results simultaneously in one traversal
- See `is_even`/`is_odd` as the canonical mutumorphism example
- Understand the difference between `zygo` (one-way dependency) and `mutu` (two-way dependency)

## Rust Application

`mutu<A, B>(alg_a: impl Fn(NatF<(A, B)>) -> A, alg_b: impl Fn(NatF<(A, B)>) -> B) -> impl Fn(FixNat) -> (A, B)`. Both algebras receive `NatF<(A, B)>` — the pair of both results for each child. `alg_a` for `is_even`: `NatF::ZeroF => true`, `NatF::SuccF((_, b_pred)) => b_pred` (is_even(n) = is_odd(n-1) = the b result of the predecessor). Symmetric for `alg_b`. The result is `(is_even(n), is_odd(n))` in one pass.

## OCaml Approach

OCaml's `let rec ... and` provides native mutual recursion:
```ocaml
let rec is_even n = if n = 0 then true else is_odd (n - 1)
and is_odd n = if n = 0 then false else is_even (n - 1)
```
OCaml's native mutual recursion is more readable than the `mutu` scheme for simple cases. The `mutu` formulation is useful when the mutual structure must be captured as a first-class value (e.g., for testing or transformation).

## Key Differences

1. **Native mutual recursion**: OCaml's `let rec ... and` expresses mutual recursion directly; Rust's `fn` definitions can call each other directly too — `mutu` is the structured/compositional version.
2. **First-class capture**: `mutu` captures the mutual recursion as a composable value; native `let rec ... and` is not first-class.
3. **Zygomorphism vs. mutumorphism**: `zygo` has one-way dependency (A uses B's results); `mutu` has two-way dependency (A uses B's and B uses A's).
4. **Practical use**: Mutual recursion arises in grammar parsers (expression/statement), type checkers (term/type), and game AI (agent/environment).

## Exercises

1. Implement `count_even_odd(n)` using `mutu`: count how many even and odd numbers exist in `[0..n]` in one traversal.
2. Write a `mutu`-based tree algorithm: `min_leaf` and `max_leaf` computed simultaneously.
3. Express the Fibonacci sequence using `mutu`: `fib(n)` and `fib(n+1)` computed together.

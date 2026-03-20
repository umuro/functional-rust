📖 **[View on hightechmind.io →](https://hightechmind.io/rust/273-fibonacci-variants)**

---

# Example 273: Fibonacci Variants

**Difficulty:** ⭐⭐
**Category:** Recursion & Iteration | Accumulators | Fold
**OCaml Source:** Classic functional programming exercise

## Problem Statement

Compute the nth Fibonacci number using three different strategies: direct recursion, tail-recursive accumulator, and a fold over a range — then add an idiomatic iterative Rust version for comparison.

## Learning Outcomes

- How OCaml tail-call optimisation maps to a simple loop or inner-function recursion in Rust
- How `List.fold_left` over a dummy list translates to `(0..n).fold` over a range
- Why pattern matching on `u64` cleanly handles the base cases without `if`/`else`
- The ergonomic difference between Rust's mutable-local-variable loop and OCaml's accumulator style

## OCaml Approach

OCaml uses three styles: naked double recursion (`fib_naive`), an inner `go` function that threads accumulator values and is tail-call optimised by the compiler (`fib_tail`), and a fold over a dummy integer list created with `List.init` (`fib_fold`). The tail-recursive version is idiomatic OCaml for linear recursion.

## Rust Approach

Rust mirrors each OCaml variant directly. `fib_naive` is a straightforward `match`; `fib_tail` uses a nested `fn go` with the same accumulator pattern; `fib_fold` uses `(0..n).fold`. A fourth `fib_iter` variant uses an explicit `for` loop, which is the most idiomatic Rust style and avoids any stack concerns.

## Key Differences

1. **Tail-call optimisation:** OCaml guarantees TCO for tail calls; Rust does not, so `fib_tail` could theoretically overflow for very large `n`. The iterative `fib_iter` is the safe Rust default.
2. **Fold target:** OCaml folds over a `List.init n Fun.id` dummy list; Rust folds over a `0..n` range — no allocation needed.
3. **Pattern matching on integers:** Both languages support it, but Rust's `match` arm `n => …` binds the same variable name, which is clean and identical in structure to OCaml's function clauses.
4. **Mutability:** `fib_iter` uses two `mut` locals, which would be unidiomatic in OCaml; in Rust it is perfectly natural and zero-cost.

## Exercises

1. Implement the Tribonacci sequence (each term is the sum of the three preceding terms) using the same structural techniques as the Fibonacci variants.
2. Implement a `generalized_fibonacci` that takes an initial pair `(a, b)` and computes the `n`-th term, then verify that `golden_ratio` emerges from `fib(n+1) / fib(n)` as `n` grows.
3. Compare the performance of the recursive, memoized, iterative, and matrix-exponentiation Fibonacci variants for computing `fib(50)` and explain the asymptotic complexity of each.

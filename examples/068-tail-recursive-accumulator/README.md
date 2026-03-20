📖 **[View on hightechmind.io →](https://hightechmind.io/rust/068-tail-recursive-accumulator)**

---

# 068 — Tail-Recursive Accumulator

## Problem Statement

Tail recursion optimization (TCO) transforms tail-recursive functions into loops, enabling recursion on large inputs without stack overflow. OCaml guarantees TCO for tail-recursive functions. Rust does not — instead, it encourages using iterators and explicit loops which compile to the same efficient code without TCO guarantees.

Understanding the accumulator pattern — rewriting `f(x) = x + f(x-1)` into `f_acc(x, acc) = f_acc(x-1, acc+x)` — is essential for writing stack-safe recursive functions in any language. It is the bridge between mathematical induction and efficient iteration.

## Learning Outcomes

- Understand the difference between naive recursion (not tail-recursive) and accumulator-based recursion (tail-recursive)
- Convert `sum([1,2,3,4,5])` from `1 + sum([2,3,4,5])` to `sum_acc([2,3,4,5], 1)`
- Recognize that Rust's `fold` is the idiomatic equivalent of a tail-recursive accumulator
- Write both recursive and loop-based versions and verify they produce identical results
- Understand why deep naive recursion causes stack overflow in Rust but not OCaml

## Rust Application

`sum_recursive` is naive — not tail-recursive (the addition `v[0] +` happens after the recursive call returns). `sum_loop` is the equivalent loop — Rust prefers this. `sum_fold` uses `fold(0, |acc, &x| acc + x)` — the idiomatic functional form, compiled to a loop. `fact_recursive` vs `fact_loop` shows the same pattern for factorial. The key lesson: in Rust, use `fold` or explicit loops instead of accumulator recursion.

## OCaml Approach

OCaml's tail-recursive sum: `let rec sum_acc acc = function [] -> acc | x :: t -> sum_acc (acc + x) t`. This is guaranteed to be compiled to a loop by OCaml's TCO. The non-tail-recursive `let rec sum = function [] -> 0 | x :: t -> x + sum t` risks stack overflow for large lists. Idiomatic OCaml always uses the accumulator form for list traversals.

## Key Differences

1. **TCO guarantee**: OCaml guarantees TCO for tail calls. Rust does not — tail-recursive functions in Rust still allocate stack frames. Use `fold` or explicit loops instead.
2. **`fold` = tail recursion**: Rust's `iter().fold(init, |acc, x| ...)` is exactly the accumulator pattern, compiled to an efficient loop. It is the idiomatic replacement.
3. **Stack overflow**: `sum_recursive` on a 100,000-element slice will likely stack overflow in Rust. OCaml's `sum_acc` on a 100,000-element list is safe due to TCO.
4. **Mutual recursion**: Even in OCaml, mutually recursive tail calls (A calls B, B calls A) are not always optimized. Both languages should use loops for mutual recursion at scale.

## Exercises

1. **Fibonacci with accumulator**: Write `fib_acc(n: u64, a: u64, b: u64) -> u64` where `a` and `b` carry the last two Fibonacci numbers. Verify it does not overflow for n=100 (use `u128`).
2. **Flatten accumulator**: Write `flatten_acc<T: Clone>(lists: &[Vec<T>], acc: Vec<T>) -> Vec<T>` that flattens nested lists using an accumulator. Compare with `iter().flatten().collect()`.
3. **CPS transformation**: Transform `sum_recursive` into continuation-passing style (CPS): `sum_cps(v: &[i32], k: impl Fn(i32) -> i32) -> i32`. This makes any recursion tail-recursive.

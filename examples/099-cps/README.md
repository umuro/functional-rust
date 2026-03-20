[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 099 — Continuation-Passing Style (CPS)
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Transform recursive functions to CPS by passing a continuation — "what to do next" — as an explicit function argument. Implement `factorial` in direct, CPS, and iterative styles. Show CPS on a binary tree sum. Compare with OCaml where CPS provides genuine tail recursion versus Rust where CPS uses boxed closures but does not eliminate stack use.

## Learning Outcomes

- Understand CPS: every call passes a function that receives the result
- Implement `factorial_cps(n, k)` where `k` accumulates pending multiplications
- Recognise that `go(n-1, move |result| k(n * result))` builds a closure chain on the heap
- Understand that Rust CPS with `Box<dyn FnOnce>` does not achieve true tail recursion
- Map Rust's boxed CPS to OCaml's natively tail-recursive CPS
- Prefer `(1..=n).product()` — the idiomatic iterative alternative

## Rust Application

`factorial_cps` defines an inner `go(n, k: Box<dyn FnOnce(u64) -> u64>)`. At `n=0` it calls `k(1)`. Otherwise it recurses with `go(n-1, Box::new(move |result| k(n * result)))` — building a chain of closures on the heap. Each continuation captures `k` and `n`. Unlike OCaml, Rust's CPS still uses the call stack for the `go` call itself. The practical solution is `(1..=n).product()`. The `tree_sum_cps` version shows CPS on a tree structure.

## OCaml Approach

OCaml's `go n k` is `let rec go n k = if n = 0 then k 1 else go (n-1) (fun result -> k (n * result))`. Because `go` is tail-recursive (the last operation in each branch is the call to `go` or `k`), OCaml optimises this to a loop with O(1) stack. The `sum_cps` tree version shows nested CPS: `go l (fun sl -> go r (fun sr -> k (sl + sr)))`.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Continuation type | `Box<dyn FnOnce(u64) -> u64>` | `int -> int` (polymorphic) |
| Tail recursion | No (stack still grows) | Yes (TCO to loop) |
| Heap allocation | One `Box` per step | Closure captured on OCaml heap |
| Practical version | `(1..=n).product()` | `let rec fold` or `Stdlib.fold_left` |
| Verbosity | High | Low |
| Educational value | Shows closure mechanics | Shows genuine TCO |

CPS is primarily a transformation technique for compilers and theoretical study. In practice, Rust solves the stack-overflow problem with iterative solutions and the `Iterator` protocol; OCaml uses tail-call optimisation. Understanding CPS helps with async/await, callback-based APIs, and compiler intermediate representations.

## Exercises

1. Implement `fib_cps(n: u64) -> u64` in CPS style and verify it matches the direct recursive version.
2. Convert `tree_sum` to use an explicit `Vec`-based stack instead of CPS — the standard iterative tree traversal technique in Rust.
3. Implement a `trampolining` version: instead of `Box<dyn FnOnce>`, return an enum `Step::Done(u64) | Step::Bounce(Box<dyn FnOnce() -> Step>)` and loop in a driver.
4. In OCaml, implement `map_cps : ('a -> 'b) -> 'a list -> 'b list` in CPS and verify tail recursion.
5. Explore Rust's `async`/`await` as a form of CPS desugaring: sketch how `async fn f()` resembles a continuation-passing transformation.

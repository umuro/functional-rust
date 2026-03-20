📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1052-fibonacci-dp)**

---

# 1052-fibonacci-dp — Fibonacci Bottom-Up DP
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Bottom-up dynamic programming fills a table iteratively from the smallest sub-problems to the largest, avoiding recursion stack overhead and the overhead of hash map lookups in top-down memoization. For Fibonacci, this means filling an array from index 0 upward.

The classic space optimization reduces the O(n) array to O(1) space by observing that `fib(n)` depends only on the two previous values — keeping just two variables suffices.

## Learning Outcomes

- Implement Fibonacci with a `Vec`-based DP table (O(n) space)
- Optimize to O(1) space using two rolling variables
- Express the same computation as an iterator/fold
- Understand when O(n) table is needed (path reconstruction) vs O(1) is sufficient
- Connect to the general DP pattern: define sub-problems, fill bottom-up

## Rust Application

`src/lib.rs` provides three implementations. `fib_vec` fills a `Vec<u64>` of size `n+1` from index 2 upward. `fib_const` uses two variables `(a, b)` with a rolling update. `fib_fold` expresses the same as `(0..n-1).fold((0u64, 1u64), |(a, b), _| (b, a + b)).1` — a purely functional expression of the O(1) algorithm.

The `fib_fold` implementation shows that the O(1) space DP is isomorphic to a fold over a range — the accumulator IS the DP state.

## OCaml Approach

OCaml's bottom-up Fibonacci:

```ocaml
let fib_dp n =
  if n <= 1 then n
  else
    let arr = Array.make (n + 1) 0 in
    arr.(1) <- 1;
    for i = 2 to n do
      arr.(i) <- arr.(i-1) + arr.(i-2)
    done;
    arr.(n)

(* O(1) space version *)
let fib n =
  let rec go a b = function
    | 0 -> a
    | n -> go b (a + b) (n - 1)
  in
  go 0 1 n
```

OCaml's tail-recursive version is naturally O(1) space and stack-safe. Rust's iterative version avoids stack overflow for the same reason.

## Key Differences

1. **Tail recursion**: OCaml's tail-recursive `fib` is idiomatic and stack-safe; Rust's iterative version achieves the same without tail-call optimization (which Rust does not guarantee).
2. **Fold idiom**: Rust's `fold((0, 1), |(a, b), _| (b, a+b))` is a compact functional expression; OCaml's equivalent is a tail-recursive fold.
3. **Overflow behavior**: Rust's `u64` panics on overflow in debug mode; OCaml's `int` wraps silently. Use `u64::checked_add` for safe overflow detection in Rust.
4. **Table vs rolling**: Both languages can use either the table or the rolling variable approach — the choice depends on whether intermediate values are needed.

## Exercises

1. Implement `fib_matrix(n: u64) -> u64` using matrix exponentiation for O(log n) time complexity.
2. Generalize `fib_fold` into a `linear_recurrence(coeffs: &[i64], init: &[i64], n: usize) -> i64` for any linear recurrence.
3. Write `fib_pairs(n: usize) -> Vec<(u64, u64)>` that returns all (fib(i), fib(i+1)) pairs up to n using the rolling-variable approach.

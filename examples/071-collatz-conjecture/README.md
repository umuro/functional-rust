📖 **[View on hightechmind.io →](https://hightechmind.io/rust/071-collatz-conjecture)**

---

# 071 — Collatz Conjecture
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

The Collatz conjecture (1937) states that iterating `f(n) = n/2` (if n is even) or `3n+1` (if n is odd) always eventually reaches 1. Despite being trivially stateable, it has resisted proof for nearly 90 years. The sequence for n=27 takes 111 steps before reaching 1. Verified for all n up to 2^68.

The Collatz sequence is a classic recursion exercise: the number of steps to reach 1 from n. It demonstrates safe integer arithmetic with `Result`, recursive vs iterative implementation, and the structural difference between "count steps" (returns `u64`) and "list steps" (returns `Vec<u64>`).

## Learning Outcomes

- Implement Collatz step count using both recursion and iteration
- Use `Result<u64, String>` to handle non-positive inputs safely
- Use `is_multiple_of` for readable parity checks
- Understand that iterative implementation is preferred in Rust (no TCO)
- Connect to `successors` for generating the full Collatz sequence

## Rust Application

`collatz_steps(n: u64) -> u64` uses pattern matching: `1 => 0`, even `n => 1 + collatz_steps(n/2)`, odd `n => 1 + collatz_steps(3*n+1)`. The public `collatz(n: i64) -> Result<u64, String>` validates input. `collatz_iter` uses a while loop — safer in Rust since the recursion can be deep (n=27 is 111 steps; n=871 is 178 steps). `std::iter::successors` generates the full sequence lazily.

## OCaml Approach

OCaml's version: `let rec collatz n = if n = 1 then 0 else if n mod 2 = 0 then 1 + collatz (n / 2) else 1 + collatz (3 * n + 1)`. OCaml's TCO does not help here — the recursion is not tail-recursive (the `1 +` is computed after the recursive call). Both languages should use iteration for safety. The iterative version: `let collatz_iter n = let rec aux n acc = if n = 1 then acc else let next = if n mod 2 = 0 then n / 2 else 3 * n + 1 in aux next (acc + 1) in aux n 0` — this IS tail-recursive.

## Key Differences

1. **Tail recursion**: The naive `1 + collatz(next)` is not tail-recursive. OCaml's accumulator version `aux next (acc + 1)` is tail-recursive and safe. Rust's iterative version is equivalent.
2. **`is_multiple_of`**: Rust uses `n.is_multiple_of(2)` (stable since 1.72). OCaml uses `n mod 2 = 0`. Both express even-number check.
3. **Integer overflow**: `3n+1` can overflow `u64` for large n. Rust's `u64::checked_mul(3)?.checked_add(1)?` detects overflow. OCaml's arbitrary-precision integers (with Zarith) avoid this.
4. **`successors` for sequence**: Rust's `successors(Some(n), |&x| if x <= 1 { None } else { Some(step(x)) })` generates the sequence lazily — stopping when 1 is reached.

## Exercises

1. **Longest sequence**: Find the starting number below 1,000,000 with the longest Collatz sequence. Memoize intermediate results to avoid recomputation.
2. **Stopping time distribution**: Generate a histogram of Collatz stopping times for all n from 1 to 10,000. What is the mean, median, and maximum stopping time?
3. **Syracuse problem variant**: Instead of `3n+1`, try `5n+1`. Does it always reach 1? What about `3n+3`? Experiment and observe.

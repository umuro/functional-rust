📖 **[View on hightechmind.io →](https://hightechmind.io/rust/071-collatz-conjecture)**

---

# 071 — Collatz Conjecture

## Problem Statement

The Collatz conjecture (proposed by Lothar Collatz in 1937) states that iterating `f(n) = n/2` when n is even, or `f(n) = 3n+1` when n is odd, always eventually reaches 1, regardless of the starting value. Despite being trivially stateable to a schoolchild, it has resisted proof by the best mathematicians for nearly 90 years. The sequence for n=27 takes 111 steps and reaches a peak of 9232 before eventually descending to 1. As of 2023, the conjecture has been verified computationally for all n up to approximately 2^68.

Paul Erdős famously said of the Collatz conjecture: "Mathematics is not yet ready for such problems." Its combination of simplicity and intractability makes it a favorite in recreational mathematics and computer science education.

As an engineering exercise, the Collatz sequence demonstrates: safe integer arithmetic with checked operations, recursive vs iterative implementation choices, the difference between computing a count (fold) and generating a sequence (unfold), and why Rust lacks tail-call optimization — making the iterative form the practical choice.

## Learning Outcomes

- Implement Collatz step count using both direct recursion and a while loop
- Use `Result<u64, String>` to handle non-positive inputs safely at the API boundary
- Understand why iterative implementation is preferred in Rust — the language does not guarantee tail-call optimization
- Use `std::iter::successors` to generate the full Collatz sequence lazily as an iterator
- Distinguish "count steps" (returns a number) from "list steps" (returns a sequence) as different problem shapes
- Recognize that naive recursion is fragile for sequences of unknown bounded depth

## Rust Application

`collatz_steps(n: u64) -> u64` uses pattern matching:
- `1 => 0` — the base case, 1 step away from terminating
- `n if n % 2 == 0 => 1 + collatz_steps(n/2)` — halve even numbers
- `n => 1 + collatz_steps(3*n+1)` — apply 3n+1 for odd numbers

The public `collatz(n: i64) -> Result<u64, String>` validates that n > 0. `collatz_iter` uses a while loop — safer in Rust since the recursion can be deep (n=27 is 111 steps; n=871 is 178 steps) and Rust has no TCO guarantee. `std::iter::successors(Some(n), ...)` generates the full sequence lazily, stopping at 1.

## OCaml Approach

OCaml's naive version is not tail-recursive:

```ocaml
let rec collatz n =
  if n = 1 then 0
  else if n mod 2 = 0 then 1 + collatz (n / 2)
  else 1 + collatz (3 * n + 1)
```

The tail-recursive version uses an accumulator:

```ocaml
let collatz_iter n =
  let rec aux n acc =
    if n = 1 then acc
    else let next = if n mod 2 = 0 then n / 2 else 3 * n + 1
    in aux next (acc + 1)
  in aux n 0
```

OCaml guarantees TCO for tail-recursive functions, making the accumulator version stack-safe. Both forms exist in practice.

## Key Differences

1. **Tail recursion**: The naive `1 + collatz(next)` is not tail-recursive. OCaml's accumulator version `aux next (acc + 1)` is tail-recursive and safe. Rust's iterative version is equivalent.
2. **`is_multiple_of`**: Rust uses `n.is_multiple_of(2)` (stable since 1.72). OCaml uses `n mod 2 = 0`. Both express even-number check.
3. **Integer overflow**: `3n+1` can overflow `u64` for large n. Rust's `u64::checked_mul(3)?.checked_add(1)?` detects overflow. OCaml's arbitrary-precision integers (with Zarith) avoid this.
4. **`successors` for sequence**: Rust's `successors(Some(n), |&x| if x <= 1 { None } else { Some(step(x)) })` generates the sequence lazily — stopping when 1 is reached.

## Exercises

1. **Longest sequence**: Find the starting number below 1,000,000 with the longest Collatz sequence. Memoize intermediate results to avoid recomputation.
2. **Stopping time distribution**: Generate a histogram of Collatz stopping times for all n from 1 to 10,000. What is the mean, median, and maximum stopping time?
3. **Syracuse problem variant**: Instead of `3n+1`, try `5n+1`. Does it always reach 1? What about `3n+3`? Experiment and observe.

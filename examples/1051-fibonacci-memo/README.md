📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1051-fibonacci-memo)**

---

# 1051-fibonacci-memo — Fibonacci with Memoization
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

The naive recursive Fibonacci is O(2^n) due to exponential redundant recomputation — `fib(n)` calls `fib(n-1)` and `fib(n-2)`, each of which calls the same sub-problems repeatedly. Memoization caches each result after its first computation, reducing the complexity to O(n) time and O(n) space.

Memoization is the top-down variant of dynamic programming. It is the mechanical transformation of any recursive function with overlapping sub-problems into an efficient one.

## Learning Outcomes

- Transform naive recursive Fibonacci into a memoized version using `HashMap`
- Implement memoization with an explicit cache parameter
- Build a memoizing closure using `move` semantics
- Compare top-down memoization to bottom-up DP (example 1052)
- Understand the trade-offs between the two approaches

## Rust Application

`src/lib.rs` provides `fib_memo(n: u64, cache: &mut HashMap<u64, u64>)` — a recursive function that checks the cache before computing. `make_fib_memo()` returns a closure that captures its own cache, providing a clean API. A third approach using `BTreeMap` demonstrates that the data structure choice does not affect correctness, only performance characteristics.

Top-down memoization is natural for problems where not all sub-problems are needed. Bottom-up DP (example 1052) is better when all sub-problems must be computed and the recurrence is simple.

## OCaml Approach

OCaml's memoization uses a `Hashtbl`:

```ocaml
let make_fib () =
  let cache = Hashtbl.create 64 in
  let rec fib n =
    match Hashtbl.find_opt cache n with
    | Some v -> v
    | None ->
      let v = if n <= 1 then n else fib (n-1) + fib (n-2) in
      Hashtbl.add cache n v;
      v
  in
  fib
```

OCaml's recursive `let rec` inside a closure naturally captures the cache. The `Base.Memo.recursive` function automates memoization for any recursive function.

## Key Differences

1. **`Base.Memo.recursive`**: OCaml's `Base` library provides a generic memoizer for recursive functions; Rust has no equivalent in std, requiring manual implementation.
2. **Cache parameter vs captured**: Rust's explicit `&mut HashMap` parameter is testable and clear; OCaml's captured `Hashtbl` is more concise.
3. **Overflow**: Rust uses `u64` which overflows for Fibonacci numbers > 93; OCaml's `int` is the machine word size (64-bit on modern hardware).
4. **Stack depth**: Both languages risk stack overflow for large n with recursive memoization; the iterative bottom-up approach (1052) eliminates this.

## Exercises

1. Generalize `fib_memo` into `memoize<A: Hash + Eq + Clone, R: Clone>(f: impl Fn(A, &mut HashMap<A, R>) -> R)` for any memoizable function.
2. Implement memoized tribonacci (each term is the sum of the previous three) using the same cache pattern.
3. Write a memoized solution for the number of ways to tile a 1×n board with 1×1 and 1×2 tiles.

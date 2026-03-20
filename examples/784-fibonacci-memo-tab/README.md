📖 **[View on hightechmind.io →](https://hightechmind.io/rust/784-fibonacci-memo-tab)**

---

# 784-fibonacci-memo-tab — Fibonacci: Memoization vs Tabulation

## Problem Statement

Dynamic programming (DP) is an algorithm design technique from Richard Bellman (1950s) that solves problems by breaking them into overlapping subproblems and storing intermediate results. Fibonacci is the canonical teaching example: naive recursion recomputes `fib(n-2)` exponentially many times. Two DP strategies fix this: memoization (top-down, cache on demand) and tabulation (bottom-up, fill a table iteratively). Understanding both is essential for tackling more complex DP problems.

## Learning Outcomes

- Recognize why naive recursive Fibonacci is O(2^n) and understand the overlapping subproblems insight
- Implement top-down memoization using `HashMap` as a cache
- Implement bottom-up tabulation using a `Vec<u64>` table filled iteratively
- Implement space-optimized tabulation using only two variables: O(1) space
- Know when to prefer memoization (sparse subproblems) vs tabulation (dense subproblems)

## Rust Application

`fib_naive` shows the exponential baseline. `fib_memo` wraps a `HashMap<u64, u64>` cache in a local `helper` closure. `fib_tab` fills a `Vec<u64>` from index 0 to n, reading `dp[i-1] + dp[i-2]`. `fib_space_opt` uses two variables `a` and `b`, updating in place. All four produce identical results but with dramatically different performance characteristics tested by the benchmark.

## OCaml Approach

OCaml's functional style naturally encourages memoization via `Hashtbl` or `lazy` values. A `memoize` higher-order function can wrap any recursive function: `let memo_fib = memoize (fun f n -> if n <= 1 then n else f (n-1) + f (n-2))`. The `lazy` keyword creates deferred computations that are evaluated once and cached. Tabulation uses `Array.init n (fun i -> dp.(i-1) + dp.(i-2))`.

## Key Differences

1. **Memoization ergonomics**: OCaml's `Hashtbl.memo` pattern is more concise than Rust's explicit `HashMap`; Rust requires a closure or struct to carry the cache.
2. **Lazy evaluation**: OCaml's `lazy` keyword provides built-in memoization for thunks; Rust uses `std::sync::OnceLock` or `once_cell` for equivalent functionality.
3. **Tail calls**: OCaml's compiler optimizes tail-recursive accumulator patterns; Rust has no guaranteed TCO (though LLVM often optimizes it).
4. **Space optimization**: Both languages trivially implement the two-variable space-optimized form; neither requires special syntax.

## Exercises

1. Implement `fib_matrix(n: u64) -> u64` using matrix exponentiation in O(log n) time and O(1) space, and verify it matches the other implementations.
2. Generalize the memoization pattern: write `fn memoize<A: Eq + Hash, B: Clone>(f: impl Fn(A) -> B) -> impl FnMut(A) -> B` using a `HashMap`.
3. Use the space-optimized form to compute `fib(n) mod p` for large n and prime p (relevant to competitive programming problems involving Fibonacci modular arithmetic).

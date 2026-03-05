📖 **[View on hightechmind.io →](https://hightechmind.io/rust/784-fibonacci-memo-tab)**

---

# 784. Fibonacci: Memoisation vs Tabulation DP

**Difficulty:** 3  **Level:** Intermediate

Four implementations of Fibonacci: naive, top-down memoisation, bottom-up tabulation, space-optimised, and lazy iterator — showing the full DP spectrum in Rust.

## The Problem This Solves

Fibonacci is the canonical DP teaching example, but its real lesson isn't the numbers — it's the two fundamental approaches to dynamic programming: *top-down* (memoisation) and *bottom-up* (tabulation). Every DP problem you'll ever face can be attacked with either strategy, and Fibonacci lets you understand both without the complexity of a harder problem.

In production code you'd use the O(1)-space optimised version for simple sequences, but memoisation and tabulation patterns appear everywhere: caching expensive recursive computations in parsers, compilers, and planners; building DP tables for sequence alignment and shortest-path algorithms; and structuring lazy evaluation pipelines.

The iterator variant shows a distinctly Rust idiom: infinite lazy sequences via `impl Iterator`, letting consumers pull only what they need.

## The Intuition

The naive recursion recomputes `fib(n-2)` exponentially many times. Both DP approaches avoid this by storing previously computed results: memoisation caches on the way *down* the recursion tree; tabulation fills a table on the way *up* from the base cases. In OCaml you'd reach for a recursive solution with a `Hashtbl` for memoisation; in Rust, the tabulation approach using a `Vec` is usually preferable because it's cache-friendly and avoids recursive call overhead.

## How It Works in Rust

```rust
// 1. Naive — O(2^n): exponential recomputation, only useful as a baseline
pub fn fib_naive(n: u32) -> u64 {
    match n { 0 => 0, 1 => 1, n => fib_naive(n-1) + fib_naive(n-2) }
}

// 2. Top-down memoisation — O(n) time, O(n) space
//    HashMap caches results; each subproblem solved once
pub fn fib_memo(n: u64) -> u64 {
    fib_memo_inner(n, &mut HashMap::new())
}

// 3. Bottom-up tabulation — O(n) time, O(n) space
//    Fill Vec left-to-right; no recursion, better cache locality
pub fn fib_tab(n: usize) -> u64 {
    let mut t = vec![0u64; n + 1];
    t[1] = 1;
    for i in 2..=n { t[i] = t[i-1] + t[i-2]; }
    t[n]
}

// 4. Space-optimised — O(1) space: only keep last two values
pub fn fib_opt(n: usize) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n { let c = a + b; a = b; b = c; }
    b
}

// 5. Iterator (infinite lazy stream) — idiomatic Rust
pub struct FibIter { a: u64, b: u64 }
impl Iterator for FibIter {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let val = self.a;
        let next = self.a + self.b;
        self.a = self.b; self.b = next;
        Some(val)
    }
}
// Usage: FibIter::new().find(|&n| n > 1_000_000)
```

The benchmark shows naive is ~100× slower than tabulation at `fib(35)`, and memoisation is comparable to tabulation but with HashMap overhead.

## What This Unlocks

- **All DP algorithms**: every 2D DP table is tabulation; every recursive solution with caching is memoisation. Understand these two patterns and you understand DP.
- **Lazy infinite sequences**: the `FibIter` pattern applies to any recurrence — Pascal's triangle rows, Conway sequences, number-theoretic series.
- **Performance analysis**: the three variants demonstrate O(2^n) vs O(n) vs O(1) space, a common tradeoff in real DP optimisations.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Memoisation | `Hashtbl.find_opt` + `Hashtbl.add` | `HashMap::get` + `HashMap::insert` |
| Recursion depth | Fine (tail-rec optimised, or just works) | Stack overflow risk at large n; prefer iterative |
| Lazy stream | `Stream.from` / `Seq.unfold` | `impl Iterator` with mutable state |
| Space-opt loop | Pattern match on tuple | Destructuring `let (mut a, mut b)` |
| Tabulation | `Array.init` | `vec![0u64; n+1]` then indexed mutation |

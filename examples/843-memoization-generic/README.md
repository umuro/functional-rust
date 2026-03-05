📖 **[View on hightechmind.io →](https://hightechmind.io/rust/843-memoization-generic)**

---

# 843: Generic Memoization with HashMap Cache

**Difficulty:** 3  **Level:** Intermediate

Cache pure function results in a `HashMap` to turn exponential recursion into polynomial time — and learn the `RefCell` workaround for recursive self-memoization.

## The Problem This Solves

Recursive algorithms with overlapping subproblems recompute the same values exponentially. `fib(50)` with naive recursion makes ~10^10 calls; with memoization, exactly 50 unique calls. This is the same transformation as dynamic programming, but expressed top-down as memoized recursion rather than bottom-up table-filling. Both yield the same asymptotic complexity; the choice between them is engineering preference.

Memoization appears beyond just Fibonacci: parsing with Earley or CYK algorithms memoizes subparse results, making context-free parsing O(n³) instead of exponential. Edit distance, optimal matrix chain multiplication, longest common subsequence — all are exponential without memo, polynomial with it. Understanding memoization lets you convert any recursive solution into an efficient one.

The harder challenge in Rust: how do you memoize a *recursive* function where the memo table must be accessible during the recursive call? The `HashMap` can't be borrowed mutably while a recursive call is happening. The solutions: pass `&mut HashMap` as a parameter (cleanest for Rust), or use `RefCell<HashMap>` for interior mutability when you need the memo table in a closure or struct method.

## The Intuition

The time-space tradeoff: pay O(n) memory to reduce O(2^n) computation to O(n). Each unique argument is computed exactly once; subsequent calls return the cached value immediately. The memo table is a `HashMap<K, V>` where K is the argument type and V is the return type.

For recursive memoization in Rust: pass `&mut HashMap` down through all recursive calls. This is zero-overhead — no locks, no reference counting — and the borrow checker ensures no aliasing. The `RefCell` pattern is for cases where you can't thread the `&mut` through (e.g., implementing a memoized method on a struct).

OCaml's `Hashtbl` has the same interface; OCaml closures can close over mutable state more naturally, making recursive memoization slightly simpler syntactically.

## How It Works in Rust

```rust
use std::collections::HashMap;

// Pattern 1: Pass &mut cache explicitly — cleanest Rust idiom
fn fib(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 { return n; }
    if let Some(&v) = cache.get(&n) { return v; }  // Cache hit: O(1)
    let v = fib(n - 1, cache) + fib(n - 2, cache);  // Recursive: each n computed once
    cache.insert(n, v);                               // Store result
    v
}

// Pattern 2: RefCell for interior mutability (when &mut threading is impractical)
use std::cell::RefCell;

fn fib_refcell(n: u64) -> u64 {
    thread_local! {
        static CACHE: RefCell<HashMap<u64, u64>> = RefCell::new(HashMap::new());
    }
    if n <= 1 { return n; }
    if let Some(&v) = CACHE.with(|c| c.borrow().get(&n).copied()) {
        return v;  // Borrow immutably to check
    }
    let v = fib_refcell(n - 1) + fib_refcell(n - 2);
    CACHE.with(|c| c.borrow_mut().insert(n, v));  // Borrow mutably to insert
    v
}

// Coin change: minimum coins to make `amount` — O(amount × |coins|)
fn coin_change(coins: &[u64], amount: u64) -> Option<u64> {
    let mut cache: HashMap<u64, Option<u64>> = HashMap::new();
    fn rec(coins: &[u64], amount: u64, cache: &mut HashMap<u64, Option<u64>>) -> Option<u64> {
        if amount == 0 { return Some(0); }
        if let Some(&v) = cache.get(&amount) { return v; }
        let result = coins.iter()
            .filter(|&&c| c <= amount)
            .filter_map(|&c| rec(coins, amount - c, cache).map(|n| n + 1))
            .min();
        cache.insert(amount, result);
        result
    }
    rec(coins, amount, &mut cache)
}
```

`thread_local!` + `RefCell` is the standard pattern when you want a memoized function without passing the cache explicitly — commonly used in competitive programming where you want global memoization of a recursive function.

## What This Unlocks

- **Dynamic programming without the DP state table**: Memoized recursion expresses the same recurrence as DP but in the natural recursive structure — often easier to derive and verify correctness.
- **Parsing and NLP**: CYK and Earley parsers memoize subparse results; chart parsing in NLP is memoization applied to formal grammars.
- **Game tree evaluation**: Minimax with memoization (transposition tables in chess engines) avoids re-evaluating the same board position from different move sequences — standard in game AI.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Hash table | `Hashtbl.create 16` | `HashMap::new()` |
| Lookup | `Hashtbl.find_opt tbl key` | `cache.get(&key)` → `Option<&V>` |
| Insert | `Hashtbl.replace tbl key v` | `cache.insert(key, v)` |
| Recursive memo | Close over `tbl` ref — natural | Pass `&mut cache` or use `RefCell` |
| Thread-local cache | Not applicable (single-threaded) | `thread_local! { static CACHE: RefCell<HashMap<...>> }` |

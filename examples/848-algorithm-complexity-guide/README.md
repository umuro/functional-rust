# 848: Big-O Reasoning and Complexity Analysis in Rust

**Difficulty:** 3  **Level:** Intermediate

A practical reference for algorithmic complexity in Rust: every complexity class illustrated with idiomatic code, plus Rust-specific constants that make O() analysis misleading.

## The Problem This Solves

Knowing an algorithm is O(n log n) is necessary but not sufficient. Two O(n log n) algorithms can differ by 10× in practice because of cache behavior, branch prediction, allocator overhead, and SIMD potential. Rust's zero-cost abstractions mean high-level iterator code is *often* as fast as hand-written C — but that "often" has precise conditions you need to understand.

This guide answers: when does O(n²) actually beat O(n log n)? (Answer: for n < ~32, insertion sort beats merge sort because of cache and branch predictor advantages.) When does O(1) HashMap::get actually degrade? (Answer: under adversarial keys with SipHash collision, though this is rare by design.) When should you use BTreeMap over HashMap? (When you need ordered iteration — BTreeMap's O(log n) with cache-friendly B-tree nodes can beat HashMap's O(1) for small maps.)

For Rust specifically: iterator chains are lazy and fuse into a single pass — `v.iter().filter(...).map(...).collect()` is O(n) with no intermediate allocations. Understanding this prevents unnecessary micro-optimizations.

## The Intuition

Complexity classes form a hierarchy: O(1) < O(log n) < O(n) < O(n log n) < O(n²) < O(2^n) < O(n!). For n = 10^6: O(log n) ≈ 20 ops, O(n) = 10^6 ops, O(n log n) ≈ 2×10^7, O(n²) = 10^12 — effectively impossible. The boundary between "fast" and "slow" is roughly O(n log n) for n up to 10^7 in a 1-second time limit.

The master theorem handles D&C recurrences: T(n) = a×T(n/b) + f(n). Three cases: when f dominates (Case 3), when log dominates (Case 1), when they're equal (Case 2, gives the n log n result for merge sort).

## How It Works in Rust

```rust
// O(1): Direct array access, HashMap lookup (amortized)
fn constant_access(v: &[i32], i: usize) -> i32 { v[i] }

// O(log n): Binary search — halves search space each step
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    arr.binary_search(&target).ok()  // std library: use this, not your own
}

// O(n): Single linear scan — iterate lazily, no intermediate alloc
fn linear_max(v: &[i32]) -> Option<i32> { v.iter().copied().max() }

// O(n log n): Sort — Rust uses pdqsort (unstable) or timsort (stable)
// pdqsort is O(n) for nearly-sorted, O(n log n) worst case
fn sort_demo(mut v: Vec<i32>) -> Vec<i32> {
    v.sort_unstable();  // Faster than sort() when stability not needed
    v
}

// O(n²): Insertion sort — optimal for n < ~32 due to cache/branch predictor
fn insertion_sort(v: &mut [i32]) {
    for i in 1..v.len() {
        let key = v[i];
        let mut j = i;
        while j > 0 && v[j - 1] > key { v[j] = v[j - 1]; j -= 1; }
        v[j] = key;
    }
}

// Rust-specific: iterator chains are O(n), NOT O(k×n) for k operations
// This is one pass, zero intermediate allocations:
fn process(v: &[i32]) -> Vec<i32> {
    v.iter()
        .filter(|&&x| x > 0)    // Lazy: no allocation
        .map(|&x| x * 2)         // Lazy: no allocation
        .collect()               // One allocation for final Vec
}
```

## What This Unlocks

- **Algorithm selection**: The complexity class table (O(1) through O(n!)) with concrete op-counts for n=10^6 tells you immediately which algorithms are viable for a given input size and time budget.
- **Rust performance model**: Zero-cost iterator chains, pdqsort vs timsort trade-offs, HashMap vs BTreeMap, `Vec::push` amortization — the constants behind the asymptotics.
- **Master theorem application**: Given any D&C recurrence, derive the complexity in 30 seconds — essential for designing new algorithms or analyzing contest problems.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sort stability | `List.sort` is stable | `sort` stable (timsort), `sort_unstable` faster (pdqsort) |
| HashMap | `Hashtbl` — open addressing | `HashMap` — SipHash by default, swap for `FxHashMap` if speed needed |
| Iterator chains | Eager by default; use `Seq` for lazy | Lazy by default — fuse without allocation |
| `Vec::push` | `Array.append` — O(1) amortized | Same amortized O(1); doubling strategy |
| BTreeMap | `Map` module (AVL tree) | `BTreeMap` — B-tree, better cache behavior than AVL for sequential access |

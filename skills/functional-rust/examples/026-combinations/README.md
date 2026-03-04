# 026: Combinations

**Difficulty:** 2  **Level:** Foundations

Generate all combinations of K distinct objects chosen from a list — order does not matter.

## The Problem This Solves

Combinations appear in probability, testing, and optimization: "which 3 of these 6 features do we enable?", "generate all possible 2-card hands from this deck", "find every subset of size K to test". Given `['a','b','c','d']` and `k=2`, produce all 6 pairs: `['a','b']`, `['a','c']`, `['a','d']`, `['b','c']`, `['b','d']`, `['c','d']`.

Most programmers write nested loops for this, which only works when `k` is fixed. For arbitrary `k`, you need recursion. The challenge is doing it *cleanly* — without duplicates, without tracking "which elements have been used", and without exponential memory.

Rust's recursive approach mirrors the mathematical definition: a combination either includes the first element or it doesn't. That binary choice generates all possibilities systematically.

## The Intuition

**The key insight:** Every combination of size `k` from a list either:
1. **Includes** the first element → need `k-1` more from the rest
2. **Excludes** the first element → need `k` from the rest (skipping the first)

These two recursive cases cover every possibility without overlap. It's the same decision tree you'd draw on paper.

In Python:
```python
from itertools import combinations
list(combinations(['a','b','c','d'], 2))
```

Rust doesn't have a built-in combinations iterator (though the `itertools` crate does). Understanding the recursive structure makes you independent of any library and clarifies *why* there are exactly C(n, k) = n! / (k! * (n-k)!) results.

## How It Works in Rust

```rust
fn combinations<T: Clone>(k: usize, lst: &[T]) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];  // one empty combination (base case)
    }
    if k > lst.len() {
        return vec![];  // impossible: can't choose more than we have
    }
    let head = &lst[0];
    let tail = &lst[1..];

    // Branch 1: combinations that include `head`
    let with_head: Vec<Vec<T>> = combinations(k - 1, tail)
        .into_iter()
        .map(|mut combo| {
            combo.insert(0, head.clone());  // prepend head to each sub-combo
            combo
        })
        .collect();

    // Branch 2: combinations that exclude `head`
    let without_head = combinations(k, tail);

    // Merge both branches
    let mut result = with_head;
    result.extend(without_head);
    result
}
```

**Trace for `combinations(2, ['a','b','c'])`:**
- Include 'a': need 1 more from ['b','c'] → `['a','b']`, `['a','c']`
- Exclude 'a': need 2 from ['b','c'] → `['b','c']`
- Result: `[['a','b'], ['a','c'], ['b','c']]` ✓

The bitmask version (`combinations_bitmask`) is an iterative alternative that enumerates all 2^n subsets and keeps those with exactly `k` bits set — elegant but limited to small `n` (≤ 63).

## What This Unlocks

- **Combinatorial testing** — test all K-combinations of feature flags or parameters.
- **Probability calculations** — enumerate sample spaces for exact probability computation.
- **Optimization** — evaluate all K-subsets when `n` and `k` are small enough for brute force.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive structure | Pattern match on `hd::tl` | `lst[0]` (head) + `lst[1..]` (tail) |
| Base case k=0 | Returns `[[]]` | Returns `vec![vec![]]` (one empty Vec) |
| Prepend element | `hd :: combo` | `combo.insert(0, head.clone())` |
| Merge branches | `@` (list concat) | `result.extend(without_head)` |
| Bitmask variant | Unusual in OCaml style | Natural with `u64` bit operations |

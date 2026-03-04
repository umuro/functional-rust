# 024: Lotto Draw

**Difficulty:** 1  **Level:** Beginner

Draw N distinct numbers from the range 1..M — the classic lottery algorithm.

## The Problem This Solves

The Dutch Staatsloterij, Powerball, and every other lottery works the same way: pick 6 distinct numbers from 1 to 49. This is so common it deserves its own clean abstraction separate from the general "random select" pattern.

The difference from example 023 is that here we're always sampling from a *range* (integers 1 to M), not an arbitrary list. That makes the intent crystal clear: `lotto_select(6, 49, seed)` reads exactly like the problem statement. In Python you'd write `random.sample(range(1, 50), 6)`. This example shows the Rust equivalent built from first principles.

It also demonstrates **assertion-based contracts**: if you ask for more numbers than the range contains, we `assert!` immediately rather than silently returning a shorter result. Fail loudly at the boundary condition.

## The Intuition

We generate the full range `[1, 2, 3, ..., M]` as a `Vec`, then use the same pool-shrinking trick from example 023: pick a random index, remove it from the pool, add it to results. After N steps, we have N distinct numbers from the range with uniform probability.

The key insight is that **generating the full range first** is clean but costs O(M) memory. For a 6/49 lottery that's fine. For "pick 10 from 1 million" you'd use a different approach (hash set tracking used numbers, or Knuth's algorithm). For learning purposes, the full-range approach is the clearest.

**Why distinct?** Lottery rules require distinct numbers — you can't win with [7, 7, 12, 33, 41, 49]. Removing from the pool after each pick enforces this structurally: it's *impossible* to select the same number twice.

## How It Works in Rust

```rust
fn lotto_select(n: usize, m: usize, seed: u64) -> Vec<usize> {
    assert!(n <= m, "cannot draw more numbers than the range");

    let mut rng = Lcg::new(seed);
    // Build the full range — this is our "pool"
    let mut pool: Vec<usize> = (1..=m).collect();
    let mut result = Vec::with_capacity(n);

    for _ in 0..n {
        let idx = rng.next_usize(pool.len());
        result.push(pool.remove(idx));  // remove guarantees no duplicates
    }
    result
}
```

The `(1..=m).collect()` creates `[1, 2, ..., M]`. Each `pool.remove(idx)` shrinks the pool, so by round K we're picking uniformly from M-K+1 remaining numbers. This is equivalent to a partial Fisher-Yates shuffle.

Verification that results are distinct:
```rust
let mut sorted = draw.clone();
sorted.sort();
sorted.dedup();
assert_eq!(sorted.len(), draw.len()); // no duplicates
```

## What This Unlocks

- **Any lottery system**: change N and M to model 5/90, 7/35, or any other lottery format.
- **Card games**: model a 52-card deck as range 1..52, deal hands as lotto draws.
- **Test data generation**: reproducible distinct IDs for unit test fixtures.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Range to list | `List.init m (fun i -> i + 1)` | `(1..=m).collect::<Vec<_>>()` |
| Remove nth element | Recursive pattern match on list | `Vec::remove(idx)` |
| Assertion | `assert (n <= m)` | `assert!(n <= m, "message")` |
| Distinct guarantee | Structural — list shrinks | Structural — Vec shrinks; same idea |

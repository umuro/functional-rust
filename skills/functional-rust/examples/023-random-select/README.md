# 023: Random Select

**Difficulty:** 1  **Level:** Beginner

Extract N randomly chosen elements from a list without replacement.

## The Problem This Solves

You need to pick random items from a collection — shuffle a playlist, sample test data, deal cards to players. Python programmers reach for `random.sample(my_list, n)`. This example shows how to build the same thing from scratch in Rust, and why the naive approach (just pick `index % length`) produces biased results.

The **modulo bias** problem is subtle: if you have 5 items and your random number is in `[0, 256)`, then indices 0–1 appear 52 times while indices 2–4 appear only 51 times. Over millions of draws, this skew matters in simulations, games, and cryptography.

In production you'd use the `rand` crate. Here we build a minimal **LCG** (Linear Congruential Generator) — the same algorithm used in most C standard libraries — so the code is self-contained and the mechanics are visible.

## The Intuition

An **LCG** (Linear Congruential Generator) is a pseudo-random number sequence produced by the formula: `next = (prev × multiplier + increment) mod 2^64`. The magic constants (from Numerical Recipes) are chosen so the sequence visits all 2^64 values before repeating. We discard the low bits (they have poor randomness) and use only the top 31 bits.

**Sampling without replacement** means once you pick an item, it can't be picked again. The cleanest way: copy the list into a mutable pool, pick a random index, *remove* that element (so it shrinks), repeat. This is the Fisher-Yates shuffle truncated to N steps.

**With replacement**: each pick is independent — re-index the full list every time. Useful when you want duplicates (e.g., bootstrap statistics).

## How It Works in Rust

```rust
struct Lcg { state: u64 }

impl Lcg {
    fn new(seed: u64) -> Self { Lcg { state: seed } }

    fn next_usize(&mut self, modulus: usize) -> usize {
        // LCG step: multiply + add (wrapping to stay in u64)
        self.state = self.state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        // Use top 31 bits — they have better statistical quality than low bits
        ((self.state >> 33) as usize) % modulus
    }
}
```

Sampling without replacement:
```rust
fn rand_select<T: Clone>(lst: &[T], n: usize, seed: u64) -> Vec<T> {
    let mut rng = Lcg::new(seed);
    let mut pool = lst.to_vec();          // mutable working copy
    let count = n.min(pool.len());        // clamp to available items
    let mut result = Vec::with_capacity(count);

    for _ in 0..count {
        let idx = rng.next_usize(pool.len());
        result.push(pool.remove(idx));    // remove shrinks the pool
    }
    result
}
```

`pool.remove(idx)` does two things: returns the element and closes the gap, so subsequent picks never re-select it.

## What This Unlocks

- **Reproducible randomness**: same seed → same draw. Essential for debugging games, tests, and simulations.
- **Card dealing / lottery**: pick N from M without repeats — exactly what Lotto example 024 builds on.
- **Bootstrap sampling**: with-replacement variant powers statistical resampling algorithms.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Random int | `Random.int n` (global state, no seed needed) | Custom LCG or `rand::thread_rng().gen_range(0..n)` |
| Remove from list | `List.filteri` or recursive rebuild (O(n)) | `Vec::remove(idx)` — shifts elements (also O(n)) |
| Seeded determinism | `Random.init seed` (global) | Seed stored in struct — no global state |
| Bias avoidance | Same modulo bias issue exists | Same; `rand` crate uses rejection sampling |

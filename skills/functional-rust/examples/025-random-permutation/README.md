# 025: Random Permutation

**Difficulty:** 2  **Level:** Foundations

Randomly shuffle a list using the Fisher-Yates algorithm — producing a valid permutation with every ordering equally likely.

## The Problem This Solves

Shuffling is everywhere: card games, A/B test assignment, randomized algorithms, playlist shuffle. The naive approach — pick random elements repeatedly — is biased: some orderings appear more often than others. The Fisher-Yates algorithm fixes this, producing a *uniform* shuffle where every possible ordering is equally probable.

In Python: `random.shuffle(lst)` handles this for you, but it mutates the list in place and relies on the hidden global random state. If you need deterministic shuffles (reproducible tests, seeded simulations), you have to manually seed `random` and hope nothing else reseeds it.

Rust makes the seed explicit. This example includes a simple Linear Congruential Generator (LCG) — a fast, seedable pseudo-random number generator — so you get deterministic shuffles that reproduce exactly given the same seed. No hidden global state.

## The Intuition

**Fisher-Yates in plain English:** Walk the list from right to left. At each position `i`, pick a random position `j` between 0 and `i` (inclusive), then swap elements at `i` and `j`. After processing every position, each element has been placed exactly once, with equal probability at each slot.

In Python: `random.shuffle(lst)` does this internally (Knuth shuffle = Fisher-Yates).

The LCG (Linear Congruential Generator) is a classic PRNG formula: `state = state * A + C`. The constants used here (from the Knuth MMIX parameters) produce good statistical properties for general use. For cryptography you'd use a proper CSPRNG — but for shuffling a playlist or test data, LCG is perfect.

## How It Works in Rust

```rust
// A simple seedable random number generator
struct Lcg { state: u64 }

impl Lcg {
    fn new(seed: u64) -> Self { Lcg { state: seed } }

    fn next_usize(&mut self, modulus: usize) -> usize {
        // LCG formula: multiply + add (Knuth MMIX constants)
        self.state = self.state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.state >> 33) as usize) % modulus  // extract upper bits
    }
}

fn permutation<T: Clone>(lst: &[T], seed: u64) -> Vec<T> {
    let mut rng = Lcg::new(seed);
    let mut result = lst.to_vec();
    let n = result.len();
    for i in (1..n).rev() {        // walk from back to front
        let j = rng.next_usize(i + 1);  // random index in [0, i]
        result.swap(i, j);         // swap in place
    }
    result
}
```

- `wrapping_mul` / `wrapping_add` — integer overflow is intentional in LCG; `wrapping_` prevents panic
- `>> 33` — shift off the lower bits, which have worse statistical properties in LCG
- `result.swap(i, j)` — no allocation, just swap two positions in the Vec
- Same seed → same output every time (deterministic)

## What This Unlocks

- **Reproducible tests** — fix a seed to get the same shuffle in every test run.
- **Card games and simulations** — implement draw/deal with fair randomness.
- **Randomized algorithms** — random sampling, random pivots in quicksort, random restarts.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Random number generation | `Random.int` (global state) | Explicit `Lcg` struct with seed |
| Shuffle approach | Remove-and-accumulate (like `permutation_remove`) | Fisher-Yates in-place swap |
| Mutation | OCaml lists immutable; build new | Clone first, then `Vec::swap` in place |
| Reproducibility | `Random.init seed` (global side effect) | `Lcg::new(seed)` — local, no side effects |
| Correctness guarantee | Same algorithm, different structure | Fisher-Yates: mathematically uniform |

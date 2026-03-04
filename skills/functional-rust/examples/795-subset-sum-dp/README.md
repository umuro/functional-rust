# 795. Subset Sum (DP with Bitset Optimisation)

**Difficulty:** 4  **Level:** Advanced

Determine whether a subset of integers sums to a target — with a bitset optimisation that packs 64 reachability bits into a single `u64`.

## The Problem This Solves

Subset sum is a fundamental NP-complete problem, but DP pseudo-polynomial solutions make it tractable for practical input sizes. It appears in partition problems (can this set be split into two equal-sum halves?), bin packing approximations, resource allocation feasibility checks, and cryptographic constructs like knapsack-based public-key systems.

The bitset optimisation is the key lesson here: instead of a `Vec<bool>` where each element stores one reachability bit, pack 64 bits into each `u64`. A left-shift-and-OR over the word array extends reachability for all 64 sums simultaneously. This is the same trick used in fast set intersection and in high-performance SAT solvers.

## The Intuition

`dp[s]` is true if any subset of the input numbers sums to `s`. For each number `x`, every previously-reachable sum `s` also makes `s+x` reachable — so iterate `s` right-to-left (same trick as 0/1 knapsack) and set `dp[s] = true` if `dp[s-x]` was true. The bitset variant represents the entire `dp` array as a bit vector: adding element `x` corresponds to shifting the bit vector left by `x` bits and OR-ing it with itself. In Rust, `Vec<u64>` is the natural bitset container — 64 booleans per word, with manual word-boundary handling for multi-word shifts.

## How It Works in Rust

```rust
// Standard DP — O(n × target) time, O(target) space
fn subset_sum_dp(nums: &[usize], target: usize) -> bool {
    let mut dp = vec![false; target + 1];
    dp[0] = true;
    for &x in nums {
        for s in (x..=target).rev() {  // right-to-left: each element used at most once
            if dp[s - x] { dp[s] = true; }
        }
    }
    dp[target]
}

// Bitset optimisation — same logic, but 64× denser
// bits[word] packs bits [word*64 .. word*64+63]
fn subset_sum_bitset(nums: &[usize], target: usize) -> bool {
    let words = (target / 64) + 1;
    let mut bits = vec![0u64; words];
    bits[0] = 1;   // sum 0 is reachable (bit 0 set)

    for &x in nums {
        // bits |= bits << x   (across word boundaries)
        let word_shift = x / 64;
        let bit_shift  = x % 64;
        for i in (0..words).rev() {
            let from = if i >= word_shift { i - word_shift } else { continue };
            let mut shifted = bits[from] << bit_shift;
            if bit_shift > 0 && from > 0 {
                shifted |= bits[from - 1] >> (64 - bit_shift);
            }
            bits[i] |= shifted;
        }
    }
    let word = target / 64;
    let bit  = target % 64;
    (bits[word] >> bit) & 1 == 1
}

// Full traceback: 2D dp table to reconstruct the actual subset
fn subset_find(nums: &[usize], target: usize) -> Option<Vec<usize>> {
    let mut dp = vec![vec![false; target + 1]; n + 1];
    dp[0][0] = true;
    // ... fill table, then walk backwards
}
```

The bitset cross-word shift requires careful handling: the high bits of one word spill into the low bits of the next. The `if bit_shift > 0 && from > 0` guard handles the carry from the lower word.

## What This Unlocks

- **Partition feasibility**: determine if a set of job durations can be split into two equal-length schedules, or if an array can be partitioned into two equal-sum halves.
- **Knapsack existence check**: before running the full knapsack algorithm, use subset sum to quickly verify that the target capacity is reachable at all.
- **Bitset techniques generally**: the `Vec<u64>` bitset pattern reappears in graph algorithms (adjacency matrices for dense graphs), SIMD-friendly interval queries, and fast set-intersection in query engines.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Boolean DP | `Array.make` of `bool` | `vec![false; target+1]` |
| Right-to-left iteration | `for s = target downto x do` | `for s in (x..=target).rev()` |
| Bitset | `Bytes.t` or `Bigarray` | `Vec<u64>` with manual word arithmetic |
| Word shift | Bit operations on `int` (63-bit) | Explicit `word_shift = x/64`, `bit_shift = x%64` |
| Traceback | Recursive with index | Reverse loop over 2D `Vec<Vec<bool>>` |

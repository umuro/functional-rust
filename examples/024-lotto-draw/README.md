📖 **[View on hightechmind.io →](https://hightechmind.io/rust/024-lotto-draw)**

---

# 024 — Lotto — Draw N Different Random Numbers from the Set 1..M
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A lotto draw selects k distinct numbers from 1 to m uniformly at random — for example, a 6/49 lottery draws 6 numbers from {1, ..., 49}. This is random selection without replacement from a generated range, combining examples 022 (range) and 023 (random select) into a single operation.

The problem appears in lottery systems, statistical sampling, hash table probing sequences (linear probing with random start), cryptographic nonce generation, and randomized algorithms like random QuickSort pivot selection. Understanding that "random draw from 1..m" is equivalent to "shuffle range(1, m+1), take first k" is the key insight.

## Learning Outcomes

- Generate a range `[1..=m]`, shuffle it, and take the first k elements
- Understand lotto draw as a special case of random selection without replacement
- Use `Vec::shuffle` from the `rand::seq::SliceRandom` trait
- Recognize that the result must be sorted only if required (lottery displays are sorted)
- Handle the constraint k <= m

## Rust Application

The idiomatic approach: generate `(1..=m).collect::<Vec<u32>>()`, call `v.shuffle(&mut rng)` from `rand::seq::SliceRandom`, then take `v[..k].to_vec()`. Sort if needed with `result.sort()`. Alternatively, use `random_select` from example 023 applied to `range(1, m)`. The `rand` crate's `choose_multiple` handles this directly. For large `m` and small `k`, the sparse approach (generate random numbers, retry on collision using a `HashSet`) is more efficient than shuffling the full range.

## OCaml Approach

OCaml's version: `let lotto_select n m = let range = List.init m (fun i -> i + 1) in random_select range n`. This reuses `random_select` from problem 23, passing the range `[1; 2; ...; m]` as the source list. OCaml's `List.init` generates the range eagerly. To sort the result for display: `List.sort compare selected`.

## Key Differences

1. **Reuse vs inline**: OCaml composes `range` (problem 22) + `random_select` (problem 23). Rust can do the same or use the built-in shuffle which is more efficient on `Vec`.
2. **`shuffle` vs `choose_multiple`**: `Vec::shuffle` rearranges all m elements (O(m)). `choose_multiple` uses a selection algorithm and is O(k) when k << m. For k=6, m=49 both are fast; for k=6, m=1_000_000 `choose_multiple` wins.
3. **Sorted output**: Lottery draws are typically displayed sorted. Rust: `result.sort()`. OCaml: `List.sort compare result`. Both are O(k log k) after selection.
4. **Collision-free guarantee**: Shuffle-based approach guarantees no repeats by construction. Rejection-sampling approach (pick random, retry if seen) needs a `HashSet` but is better for sparse k/m ratios.

## Exercises

1. **Multiple draws**: Write `simulate_lottery(draws: usize, k: usize, m: u32) -> Vec<Vec<u32>>` that performs `draws` independent lotto draws and returns them all sorted.
2. **Frequency analysis**: Run 100,000 draws of 6/49 lotto and verify each number appears approximately 100,000 * 6/49 ≈ 12,245 times. Check for bias in your RNG.
3. **Expected jackpot wait**: Using simulation, estimate how many 6/49 draws are needed on average to get the same 6 numbers as a target draw. Compare with the theoretical 1 in C(49,6) ≈ 1 in 13,983,816.

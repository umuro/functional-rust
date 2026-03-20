📖 **[View on hightechmind.io →](https://hightechmind.io/rust/023-random-select)**

---

# 023 — Extract a Given Number of Randomly Selected Elements
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Selecting k elements uniformly at random without replacement from a list (OCaml 99 Problems #23) is one of the fundamental sampling problems in computer science. It underlies A/B testing, bootstrapping in statistics, Monte Carlo simulations, shuffling in card games, and random forest sampling in machine learning.

The naive approach — pick a random index, remove it, repeat — is O(k·n) due to repeated removal from the middle. The efficient approach, Fisher-Yates partial shuffle, is O(k): shuffle only the first k positions of the array, then take them. This is the algorithm used in `numpy.random.choice` and most production sampling implementations.

## Learning Outcomes

- Use `rand::thread_rng()` and `Rng::gen_range` for uniform random index generation
- Implement random selection via repeated random removal (naive, O(k·n))
- Understand the Fisher-Yates partial shuffle as the O(k) alternative
- Handle the constraint k <= n (cannot select more items than exist)
- Return a new `Vec<T>` of the selected elements

## Rust Application

The naive approach: clone the list into a mutable `Vec`, then loop k times picking a random index with `rng.gen_range(0..remaining.len())` and using `swap_remove` (O(1) by swapping with last element). A cleaner version uses Fisher-Yates partial shuffle: shuffle only the first k positions, then return `v[..k].to_vec()`. The `rand` crate's `choose_multiple` implements this correctly. For reproducibility, seed the RNG with a fixed value.

## OCaml Approach

OCaml's version using `Random`: `let random_select lst n = let arr = Array.of_list lst in let len = Array.length arr in for i = 0 to n - 1 do let j = i + Random.int (len - i) in let tmp = arr.(i) in arr.(i) <- arr.(j); arr.(j) <- tmp done; Array.to_list (Array.sub arr 0 n)`. This is Fisher-Yates partial shuffle: swap the i-th element with a random element in `[i, len)`, building the selected prefix.

## Key Differences

1. **Random number generation**: Rust requires the `rand` crate (not in stdlib). OCaml's `Random` module is in the standard library. Both use `gen_range`-style APIs.
2. **Array vs list**: Fisher-Yates requires O(1) random access. Both implementations convert to array/Vec first. OCaml uses `Array.of_list`; Rust starts with `Vec`.
3. **`swap_remove`**: Rust's `Vec::swap_remove(i)` replaces element i with the last element — O(1) but changes order. This is the key to efficient random deletion.
4. **Seeding**: Rust's `rand::rngs::StdRng::seed_from_u64(seed)` produces deterministic output. OCaml uses `Random.init seed`. Both are important for reproducible tests.

## Exercises

1. **With replacement**: Write `random_select_with_replacement(v: &[i32], k: usize) -> Vec<i32>` that allows the same element to be selected multiple times. This is simpler — just pick k random indices independently.
2. **Weighted sampling**: Write `weighted_select(v: &[i32], weights: &[f64], k: usize) -> Vec<i32>` that samples proportionally to the given weights. Research the alias method or rejection sampling.
3. **Reservoir sampling**: Implement reservoir sampling, which selects k elements from a stream of unknown length in O(n) time and O(k) space. This is used in distributed log sampling.

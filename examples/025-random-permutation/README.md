📖 **[View on hightechmind.io →](https://hightechmind.io/rust/025-random-permutation)**

---

# 025 — Generate a Random Permutation of the Elements of a List

## Problem Statement

A random permutation (shuffle) rearranges all elements of a list with each of the n! arrangements equally likely. This is different from random selection: a permutation includes every element exactly once, just in a random order. The canonical algorithm is Fisher-Yates (Knuth) shuffle, which runs in O(n) time and is proven to produce a uniform distribution.

Random permutations appear in card shuffling, game AI (random move ordering in minimax), neural network weight initialization (shuffle before SGD), randomized QuickSort (shuffle before partitioning), and combinatorics testing (generate test cases). Non-uniform shuffles (like naively sorting with random comparators) are a well-known source of bias bugs.

## Learning Outcomes

- Use `Vec::shuffle` from `rand::seq::SliceRandom` for uniform random permutation
- Understand the Fisher-Yates algorithm: for i from 0 to n-2, swap i with random j >= i
- Prove informally why Fisher-Yates produces a uniform distribution
- Distinguish a shuffle (all elements, no repeats) from random selection (k of n)
- Implement Fisher-Yates manually to understand the invariant

- Use `Vec::shuffle(&mut rng)` from `rand::seq::SliceRandom` for uniform random permutation
- Understand that Fisher-Yates produces a uniform distribution: each of n! permutations is equally likely

## Rust Application

The idiomatic Rust approach: `let mut v = list.to_vec(); v.shuffle(&mut rand::thread_rng()); v`. The `rand::seq::SliceRandom` trait provides `shuffle` which implements Fisher-Yates. The manual implementation: `for i in (1..n).rev() { let j = rng.gen_range(0..=i); v.swap(i, j); }` — swap each element with a random element at or after its position, working backward. For reproducibility, use `rand::rngs::StdRng::seed_from_u64(42)`.

## OCaml Approach

OCaml's implementation uses the same algorithm with arrays: `let permutation lst = let arr = Array.of_list lst in let n = Array.length arr in for i = n - 1 downto 1 do let j = Random.int (i + 1) in let tmp = arr.(i) in arr.(i) <- arr.(j); arr.(j) <- tmp done; Array.to_list arr`. Alternatively: compose `range(1, n)` and `random_select(lst, n)` — select all n elements without replacement, which is a permutation by definition.

OCaml's random permutation using Fisher-Yates: `let shuffle list = let arr = Array.of_list list in let n = Array.length arr in for i = n - 1 downto 1 do let j = Random.int (i + 1) in let tmp = arr.(i) in arr.(i) <- arr.(j); arr.(j) <- tmp done; Array.to_list arr`. Note the conversion to array and back — linked lists don't support O(1) random access needed by Fisher-Yates.

## Key Differences

1. **`shuffle` in stdlib**: Rust's `SliceRandom::shuffle` is in the `rand` crate (not stdlib). OCaml has no standard `shuffle`; you implement it manually.
2. **Array conversion**: Both languages convert to array/Vec for O(1) random access needed by Fisher-Yates. OCaml: `Array.of_list`. Rust starts with `Vec`.
3. **Mutability**: Fisher-Yates is inherently imperative (swap operations). Rust uses mutable references (`&mut v`). OCaml mutates the array in place with `arr.(i) <- ...`.
4. **Functional alternative**: Both languages can implement permutation using `random_select` applied to the full list — selecting all n elements produces a permutation, but this is O(n²) naive vs O(n) Fisher-Yates.

1. **Array vs list:** Fisher-Yates requires O(1) random access. Rust's `Vec` supports this natively. OCaml must convert to `Array` first, then back to `List` — two O(n) conversions.
2. **`SliceRandom` trait:** Rust's `.shuffle(&mut rng)` is provided by the `rand` crate via `rand::seq::SliceRandom`. No equivalent in OCaml's stdlib.
3. **In-place:** Rust shuffles in place (mutable reference). OCaml's imperative version also mutates an Array in place. The functional wrapper converts back to a list.

## Exercises

1. **Verify uniformity**: Generate 1,000,000 random permutations of `[1, 2, 3]`. Count how often each of the 6 arrangements appears. They should each appear approximately 166,667 times.
2. **Derangement**: Write `derangement(v: &[i32]) -> Vec<i32>` that returns a permutation where no element appears in its original position. Use rejection sampling or the recursive algorithm.
3. **Named shuffle variants**: Research and implement Sattolo's algorithm (generates only derangements) and compare it with Fisher-Yates. What is different about the random index range?

4. **Seeded shuffle**: Implement `seeded_permutation(list: &[T], seed: u64) -> Vec<T>` that produces a deterministic permutation — useful for reproducible testing and data augmentation.
5. **k-permutations**: Implement `k_permutations<T: Clone>(list: &[T], k: usize) -> Vec<Vec<T>>` that generates all ordered selections of k elements (not combinations). The count is P(n,k) = n!/(n-k)!.

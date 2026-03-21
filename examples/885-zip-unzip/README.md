📖 **[View on hightechmind.io →](https://hightechmind.io/rust/885-zip-unzip)**

---

# 885-zip-unzip — Zip and Unzip
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Pairing two sequences element-by-element and splitting a sequence of pairs back into two separate sequences are fundamental data transformations. Mathematically, zip and unzip are inverses. Practically, zip is used for: combining coordinates with labels, computing dot products, running element-wise comparisons, and building key-value pairs. OCaml provides `List.combine` (zip) and `List.split` (unzip). Python has built-in `zip()`. Rust's `.zip()` adapter on iterators handles the pairing lazily, and `.unzip()` consumer splits pairs back. This example covers zip, zip_with (pairwise operations), zip_with_index, and zip_longest.

## Learning Outcomes

- Use `.zip()` to pair elements from two iterators, stopping at the shorter one
- Use `.unzip()` to split an iterator of pairs into two collections
- Implement `dot_product` and `pairwise_max` as zip-based operations
- Use `.enumerate()` as zip-with-index
- Implement `zip_longest` using explicit length comparison for padding

## Rust Application

`zip_vecs` uses `a.iter().zip(b.iter()).map(...).collect()`. `unzip_vecs` uses `.map(...).unzip()` to split pairs into two `Vec`s. `dot_product` is expressed as `a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()`. `pairwise_op` is a generic zip-with for any binary operation. `zip_with_index` wraps `.enumerate().map(|(i, x)| (i, x.clone()))`. `zip_longest` handles unequal lengths by comparing `a.len().max(b.len())` and using `.get(i).cloned().unwrap_or(default)` for each position.

## OCaml Approach

OCaml's `List.combine: 'a list -> 'b list -> ('a * 'b) list` is the eager zip. It raises `Invalid_argument` on unequal lengths — unlike Rust's silent truncation. `List.split: ('a * 'b) list -> 'a list * 'b list` is the unzip. `List.map2 f xs ys` is zip-with for same-length lists. `List.mapi (fun i x -> (i, x)) xs` is zip-with-index. OCaml lacks a standard zip_longest — it must be implemented with explicit recursion.

## Key Differences

1. **Length mismatch**: Rust `.zip()` silently stops at the shorter iterator; OCaml `List.combine` raises an exception on unequal lengths.
2. **Laziness**: Rust `.zip()` is lazy; OCaml `List.combine` is eager and allocates immediately.
3. **Unzip consumer**: Rust `.unzip()` works on any `Iterator<Item = (A, B)>`; OCaml's `List.split` only works on lists.
4. **zip_with**: Rust uses `.zip().map(|(a, b)| f(a, b))`; OCaml has dedicated `List.map2 f xs ys`.

## Exercises

1. Implement `weighted_sum(weights: &[f64], values: &[f64]) -> f64` using zip and fold.
2. Write `transpose_matrix<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>>` using zip over rows.
3. Implement `merge_sorted` using a peekable zip that merges two sorted slices maintaining sort order.

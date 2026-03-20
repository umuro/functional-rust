[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 091 — Zip and Unzip
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Use Rust's iterator `zip` to pair elements from two sequences, `.unzip()` to split a sequence of pairs back into two vectors, and `zip` + `map` to implement a pairwise `zip_with`. Compare with OCaml's recursive `zip`, `unzip`, and `zip_with` on lists.

## Learning Outcomes

- Use `.zip(other)` to combine two iterators into an iterator of pairs
- Understand that `zip` stops when the shorter iterator is exhausted
- Use `.unzip()` to split `Vec<(A, B)>` into `(Vec<A>, Vec<B>)` with type annotation
- Implement `zip_with` as `zip` + `map` for element-wise operations
- Map Rust's `zip` to OCaml's `List.combine` / recursive `zip`
- Recognise `unzip` as the categorical dual of `zip`

## Rust Application

`[1,2,3].iter().zip(["a","b","c"].iter()).collect()` produces `Vec<(&i32, &&str)>`. When one iterator is longer, `zip` stops at the shorter — the length-2 zip test demonstrates this. `.unzip()` requires type annotation `(Vec<i32>, Vec<&str>)` to determine the target collections. `zip` + `.map(|(a, b)| a + b)` implements `zip_with` without a separate function. These patterns appear frequently: zipping indices with values via `.enumerate()` is the most common use case.

## OCaml Approach

OCaml's `zip` is implemented with pattern matching: `match l1, l2 with | [], _ | _, [] -> [] | x::xs, y::ys -> (x,y) :: zip xs ys`. `unzip` uses `fold_right` to reconstruct two lists simultaneously. `zip_with f l1 l2` applies `f` element-wise. OCaml's standard library provides `List.combine` (equivalent to `zip`) and `List.split` (equivalent to `unzip`).

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Zip | `.zip(other)` adapter | `List.combine` / recursive |
| Unzip | `.unzip()` consumer | `List.split` |
| Zip-with | `zip + map` | `List.map2 f l1 l2` |
| Shorter list | Silently truncates | Same — stops at shorter |
| Type of result | `Vec<(&A, &B)>` (references) | `('a * 'b) list` (values) |
| Unzip annotation | `(Vec<A>, Vec<B>)` required | Inferred from usage |

`zip` and `unzip` are fundamental pairing and un-pairing operations. In Rust, they integrate naturally into the iterator chain. OCaml provides `List.combine`/`List.split` in the standard library for the same purpose. Both languages make `zip_with` trivially expressible.

## Exercises

1. Implement `zip3<A, B, C>(a: &[A], b: &[B], c: &[C]) -> Vec<(&A, &B, &C)>` using two `.zip` calls.
2. Use `enumerate()` (which is `zip` with a counter) to add indices to a vector of strings.
3. Write `dot_product(a: &[f64], b: &[f64]) -> f64` using `zip + map + sum`.
4. Implement `transpose(matrix: Vec<Vec<T>>) -> Vec<Vec<T>>` using repeated `zip` operations.
5. In OCaml, implement `zip_longest` that pads the shorter list with `None` values instead of truncating.

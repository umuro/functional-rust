📖 **[View on hightechmind.io →](https://hightechmind.io/rust/014-duplicate-elements)**

---

# 014 — Duplicate Elements

## Problem Statement

Duplicating every element of a list (OCaml 99 Problems #14) — transforming `[a, b, c]` into `[a, a, b, b, c, c]` — is an exercise in `flat_map`: the structure-expanding operation that maps each input to multiple outputs and concatenates the results. It is the simplest possible case of `flat_map` with a fixed expansion factor.

This operation appears in data augmentation pipelines (duplicating training examples), protocol framing (repeating sync bytes), audio processing (sample rate doubling by interpolation), and test harness generation. Understanding the `flat_map` pattern here prepares you for the generalized `replicate_n_times` in example 015.

## Learning Outcomes

- Use `flat_map` to expand each element into multiple output elements
- Understand that `flat_map` is equivalent to `map` followed by `flatten`
- Compare iterative (loop), functional (`flat_map`), and recursive implementations
- Pre-allocate output `Vec` with `Vec::with_capacity(len * 2)` for performance
- Recognize duplication as a degenerate case of `replicate(n=2)`

## Rust Application

`duplicate` uses `.flat_map(|x| vec![x.clone(), x.clone()])` — each element maps to a two-element vector, and `flat_map` concatenates them. `duplicate_iter` pre-allocates with `Vec::with_capacity(list.len() * 2)` and pushes each element twice — more cache-friendly than building intermediate vecs. The recursive `duplicate_recursive` matches on `split_first()`, prepends two copies of the head, and recurses on the tail. The fold-based version shows `flat_map` is a fold: `fold(vec![], |mut acc, x| { acc.push(x); acc.push(x); acc })`.

## OCaml Approach

OCaml's version is: `let duplicate lst = List.concat_map (fun x -> [x; x]) lst`. Alternatively with `List.fold_left`: `let duplicate lst = List.fold_left (fun acc x -> acc @ [x; x]) [] lst` — but this is O(n²) because `@` is O(n). The correct fold uses difference lists or reverses at the end: `List.rev (List.fold_left (fun acc x -> x :: x :: acc) [] lst)`.

## Key Differences

1. **`flat_map` vs `concat_map`**: Rust's `flat_map` and OCaml's `List.concat_map` are the same operation. Both exist in their respective standard libraries since recent versions.
2. **Clone necessity**: Rust requires `T: Clone` to duplicate elements. OCaml's GC shares the same pointer for both copies — no actual duplication of heap data.
3. **`@` vs `extend`**: OCaml's list append `@` is O(n). Rust's `Vec::extend` is O(k) where k is the new elements. Use `fold` carefully in OCaml to avoid accidental O(n²) behavior.
4. **Pre-allocation**: Rust's `Vec::with_capacity` avoids reallocations when output size is known. OCaml lists do not support pre-allocation (they are singly-linked).

## Exercises

1. **Triplicate**: Generalize to `triplicate<T: Clone>(list: &[T]) -> Vec<T>` that produces three copies of each element. Then generalize to `replicate<T: Clone>(list: &[T], n: usize) -> Vec<T>`.
2. **Interleave**: Write `interleave<T: Clone>(a: &[T], b: &[T]) -> Vec<T>` that produces `[a[0], b[0], a[1], b[1], ...]`. Use `zip` and `flat_map`.
3. **De-duplicate**: Write `dedup_consecutive<T: PartialEq + Clone>(list: &[T]) -> Vec<T>` that removes consecutive duplicates (the inverse of `duplicate`). Use `.windows(2)` to detect adjacent pairs.

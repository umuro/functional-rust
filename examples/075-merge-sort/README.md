📖 **[View on hightechmind.io →](https://hightechmind.io/rust/075-merge-sort)**

---

# 075 — Merge Sort

## Problem Statement

Merge sort (John von Neumann, 1945) is the canonical divide-and-conquer sorting algorithm: split the list in half, sort each half recursively, merge the sorted halves. It guarantees O(n log n) in all cases — unlike quicksort which degrades to O(n²) on sorted input. It is the algorithm used in Rust's `Vec::sort` (TimSort) and Java's `Arrays.sort` for objects.

Merge sort is naturally recursive and maps directly to functional programming patterns: the `merge` function is a fold over two sorted lists, and the sort function is a structural recursion on list halves. Understanding merge sort deeply unlocks intuition for all divide-and-conquer algorithms.

## Learning Outcomes

- Implement the two-phase merge sort: split, recurse, merge
- Write a `merge` function for combining two sorted slices in O(n+m)
- Implement the generic version with a custom comparator `F: Fn(&T, &T) -> Ordering`
- Understand why the base case is `len <= 1` (a list of 0 or 1 elements is already sorted)
- Compare recursive merge sort with Rust's built-in `sort` (TimSort)

## Rust Application

`merge(l1, l2)` uses two index pointers, picking the smaller element each step. `merge_sort(v)` splits at `v.len() / 2`, recursively sorts each half, then merges. `merge_sort_by<T, F>(v, cmp)` uses a comparator. The implementation clones slices — production code would use in-place operations. Rust's `Vec::sort` is stable TimSort; `Vec::sort_unstable` is pdqsort.

## OCaml Approach

OCaml's merge sort: `let rec merge_sort = function | ([] | [_]) as lst -> lst | lst -> let (left, right) = split lst (List.length lst / 2) in merge (merge_sort left) (merge_sort right)`. `merge` uses pattern matching on the two sorted lists: `let rec merge l1 l2 = match l1, l2 with | [], l | l, [] -> l | x :: t1, y :: t2 -> if x <= y then x :: merge t1 l2 else y :: merge l1 t2`.

## Key Differences

1. **Slice vs list**: Rust sorts slices (`&[i32]`) and produces new `Vec<i32>`. OCaml sorts `list` and produces new lists. List-based merge sort is purely functional; slice-based requires cloning.
2. **In-place option**: Rust can implement in-place merge sort using `Vec::split_at_mut` and `rotate_left`. OCaml's immutable lists cannot be sorted in place.
3. **Stability**: Rust's `sort` is stable (preserves relative order of equal elements). `sort_unstable` is faster but not stable. OCaml's `List.sort` is not stable; `List.stable_sort` is.
4. **`split_at`**: Rust's `v.split_at(mid)` is O(1). OCaml must traverse to the midpoint: `let rec split_at n = function ... | x :: t -> let (l, r) = split_at (n-1) t in (x :: l, r)` — O(n/2).

## Exercises

1. **In-place merge sort**: Implement an in-place merge sort for `&mut Vec<i32>` using `rotate_left` for the merge step. This avoids cloning but is O(n² log n) due to rotation cost.
2. **External sort**: Merge sort naturally extends to external sorting (data too large for RAM). Design an algorithm that reads sorted chunks from disk and merges them. What data structure manages the merge heap?
3. **Natural merge sort**: Detect pre-sorted runs and use them as the base for merging (this is the core of TimSort). Implement `timsort_lite` that detects ascending runs and merges them.

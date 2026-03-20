[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 1004 — Quicksort

## Problem Statement

Implement functional quicksort that mirrors the OCaml version: take a comparator `gt`, remove the pivot, partition the rest into smaller and larger halves, recursively sort both, and concatenate. Also provide an idiomatic Rust version using `Vec::sort`. Compare the allocation-heavy functional approach with the in-place standard library sort.

## Learning Outcomes

- Implement recursive quicksort using `.partition(|x| gt(&pivot, x))`
- Understand why the functional version allocates: each recursive call creates new `Vec`s
- Use `Vec::sort()` and `Vec::sort_by()` as the idiomatic production alternative
- Pass comparators as `F: Fn(&T, &T) -> bool + Copy` for recursive use
- Map Rust's `partition` to OCaml's `List.partition (gt x) xs`
- Recognise that `.sort()` is O(n log n) with O(log n) stack — far better than functional O(n²) worst case

## Rust Application

`quicksort(gt, xs)` removes the first element as pivot via `xs.remove(0)`, partitions the rest with `.into_iter().partition(|x| gt(&pivot, x))`, recursively sorts both halves, and concatenates. The `F: Copy` bound allows passing the comparator into recursive calls without cloning. The idiomatic `quicksort_idiomatic` simply calls `result.sort()` — the standard library's introsort (hybrid of quicksort, heapsort, insertion sort) handles worst cases. `quicksort_idiomatic_by` accepts `Fn(&T, &T) -> Ordering` for custom comparison.

## OCaml Approach

`let rec quicksort gt = function | [] -> [] | x::xs -> let ys, zs = List.partition (gt x) xs in (quicksort gt ys) @ (x :: (quicksort gt zs))` is the complete OCaml implementation. `List.partition (gt x)` creates two lists in one pass. The `@` operator concatenates the sorted halves. OCaml's pattern matching on lists is more concise than Rust's `remove(0)` approach.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Pivot extraction | `xs.remove(0)` (O(n)) | `x :: xs` pattern (O(1)) |
| Partition | `.into_iter().partition(pred)` | `List.partition (gt x) xs` |
| Concatenation | `left.push(pivot); left.append(&mut result)` | `ys @ (x :: zs)` |
| Comparator bound | `F: Copy` for recursion | Passed naturally |
| Idiomatic sort | `Vec::sort()` (in-place) | `List.sort compare lst` |
| Allocation | O(n log n) new Vecs | O(n log n) new lists |

The functional quicksort is educational — it shows the algorithm clearly. The standard library sort is for production. Rust's `Vec::sort` uses pattern-defeating quicksort (pdqsort), which is cache-friendly and avoids worst-case O(n²) behaviour.

## Exercises

1. Add a `quicksort_desc<T: Ord>(xs: Vec<T>) -> Vec<T>` that sorts in descending order using `.sort_by(|a, b| b.cmp(a))`.
2. Implement three-way partition quicksort that handles equal elements more efficiently.
3. Benchmark the functional `quicksort` vs `Vec::sort` on a 10,000-element random Vec and report the speedup.
4. Add a `quicksort_stable` using `sort_by` with index tie-breaking to maintain relative order of equal elements.
5. In OCaml, implement merge sort and compare its performance with `quicksort` on sorted and reverse-sorted inputs.

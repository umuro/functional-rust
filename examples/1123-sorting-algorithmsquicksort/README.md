📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1123-sorting-algorithmsquicksort)**

---

# 1123-sorting-algorithmsquicksort — Quicksort
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Quicksort, invented by Tony Hoare in 1959, is the most widely used sorting algorithm in practice. It achieves O(n log n) average-case time by partitioning around a pivot: elements less than the pivot go left, greater go right, and the sub-arrays are recursively sorted. The in-place variant is cache-friendly and uses O(log n) stack space.

Understanding quicksort deepens understanding of divide-and-conquer, average-case analysis, pivot selection strategies, and why Rust's `sort_unstable` (which uses a quicksort variant) is faster than `sort` (which guarantees stability using merge sort).

## Learning Outcomes

- Implement in-place quicksort using Lomuto or Hoare partition scheme
- Understand pivot selection and its effect on worst-case behavior
- Know why O(n²) worst case occurs (sorted input with naive pivot)
- Compare quicksort to merge sort: average O(n log n) vs worst-case O(n log n)
- Understand why Rust's `sort_unstable` uses a pattern-defeating quicksort (pdqsort)

## Rust Application

The canonical in-place quicksort implementation:

```rust
pub fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 { return; }
    let pivot_idx = partition(arr);
    quicksort(&mut arr[..pivot_idx]);
    quicksort(&mut arr[pivot_idx + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1];
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= pivot { arr.swap(i, j); i += 1; }
    }
    arr.swap(i, arr.len() - 1);
    i
}
```

Rust's `src/lib.rs` is a stub — the implementation above is the standard form. The `slice::sort_unstable` in the standard library uses pdqsort (pattern-defeating quicksort) which handles common patterns (sorted, reversed, many duplicates) efficiently.

## OCaml Approach

```ocaml
let rec quicksort = function
  | [] -> []
  | pivot :: rest ->
    let smaller = List.filter (fun x -> x <= pivot) rest in
    let larger = List.filter (fun x -> x > pivot) rest in
    quicksort smaller @ [pivot] @ quicksort larger
```

This functional quicksort is elegant but O(n) space per level (allocates new lists at each step). For OCaml arrays, an in-place variant uses the same Lomuto/Hoare partitioning as Rust.

## Key Differences

1. **In-place vs functional**: Rust's in-place quicksort uses O(log n) stack space; OCaml's functional version uses O(n) space per level for list copies.
2. **Stable sort**: Rust's `sort` is stable (merge sort based); `sort_unstable` (quicksort based) is faster. OCaml's `List.sort` is stable (merge sort).
3. **pdqsort**: Rust's standard library uses pdqsort which handles sorted, reverse-sorted, and many-duplicates inputs in O(n) — better than naive quicksort.
4. **Pivot selection**: Both naive implementations use the last element as pivot; median-of-three and random pivots reduce worst-case probability.

## Exercises

1. Implement quicksort with median-of-three pivot selection and benchmark it against the last-element pivot on sorted input.
2. Write a three-way partition (Dutch National Flag) that handles many duplicate elements efficiently.
3. Implement `par_quicksort(arr: &mut [i32])` using `rayon` for parallel sorting of the two sub-arrays.

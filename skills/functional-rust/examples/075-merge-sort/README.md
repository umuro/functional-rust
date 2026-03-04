# 075: Merge Sort — Functional Divide and Conquer

**Difficulty:** 3  **Level:** Intermediate

Split, sort recursively, merge — the canonical divide-and-conquer algorithm expressed with slices and `Vec`.

## The Problem This Solves

Merge sort is the textbook algorithm for stable, predictable O(n log n) sorting. It's especially valuable when you need: stability (equal elements preserve order), external sorting (data doesn't fit in memory), or parallel sorting (each half can be sorted independently).

More importantly for Rust learners, merge sort is where ownership patterns become visible. OCaml's list-based merge uses structural sharing and pattern matching on cons cells. Rust's slice-based merge owns its inputs and must manage allocation explicitly — you see exactly where memory is created and destroyed.

## The Intuition

Three steps:
1. **Split** the input in half.
2. **Sort** each half recursively (base case: length ≤ 1 is already sorted).
3. **Merge** the two sorted halves by comparing front elements.

Merging is the core operation: walk two sorted lists in lockstep, always taking the smaller front element. When one list is exhausted, append the rest of the other.

OCaml's merge uses list cons (`h1 :: merge ...`) which is O(1) per step with structural sharing. Rust's `Vec::push` and `extend_from_slice` allocate contiguously — fewer cache misses, but each level of recursion creates new allocations.

## How It Works in Rust

```rust
// Merge two sorted slices into one sorted Vec
pub fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len()); // pre-allocate
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {   // stable: left wins on tie
            result.push(left[i].clone()); i += 1;
        } else {
            result.push(right[j].clone()); j += 1;
        }
    }
    result.extend_from_slice(&left[i..]);   // append remaining
    result.extend_from_slice(&right[j..]);
    result
}

// Recursive sort: split at midpoint, sort each half, merge
pub fn merge_sort<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    if list.len() <= 1 { return list.to_vec(); }  // base case
    let mid = list.len() / 2;
    let left  = merge_sort(&list[..mid]);   // borrow left half
    let right = merge_sort(&list[mid..]);   // borrow right half
    merge(&left, &right)
}

// Iterator-based merge: more functional, uses Peekable for lookahead
pub fn merge_iter<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut li = left.iter().peekable();
    let mut ri = right.iter().peekable();
    loop {
        match (li.peek(), ri.peek()) {
            (Some(&l), Some(&r)) => {
                if l <= r { result.push(li.next().unwrap().clone()); }
                else       { result.push(ri.next().unwrap().clone()); }
            }
            (Some(_), None) => result.extend(li.cloned()),
            (None, _)       => { result.extend(ri.cloned()); break; }
        }
        if li.peek().is_none() { result.extend(ri.cloned()); break; }
    }
    result
}
```

For production sorting, prefer `slice.sort()` (Timsort) or `slice.sort_unstable()`. Merge sort here is pedagogical — it shows ownership patterns, recursion on slices, and the merge step cleanly.

## What This Unlocks

- **External sort**: merge-sort's structure extends naturally to file-based sorting (sort chunks, merge files).
- **Parallel sort**: each recursive half is independent — easy to parallelize with `rayon`.
- **Generic sort utilities**: `merge<T: Ord>` works on anything orderable.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Split | `List.filteri` or manual | `&list[..mid]` / `&list[mid..]` |
| Merge step | `h1 :: merge ...` (cons, O(1)) | `Vec::push` + `extend_from_slice` |
| Memory model | Structural sharing (GC) | Explicit allocation per level |
| Base case | `[] \| [_]` pattern match | `list.len() <= 1` |
| In-place option | No (immutable) | `slice.sort()` for in-place Timsort |

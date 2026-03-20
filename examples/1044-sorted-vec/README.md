📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1044-sorted-vec)**

---

# 1044-sorted-vec — Sorted Vec with Binary Search

## Problem Statement

When you need sorted iteration AND fast membership tests but do not need to insert and delete frequently, a sorted `Vec<T>` is more cache-efficient than a `BTreeSet`. Binary search over a contiguous array benefits from hardware prefetching, while pointer-heavy B-tree nodes fragment the cache. The `sorted_vec` and `bsearch` crates exploit this trade-off.

`partition_point` (Rust 1.52) and `binary_search` provide O(log n) insertion point and lookup over a sorted `Vec`.

## Learning Outcomes

- Maintain a sorted `Vec<T>` using `partition_point` for O(log n) insert position
- Use `binary_search` for O(log n) membership test
- Compare sorted `Vec` to `BTreeSet` for read-heavy vs write-heavy workloads
- Use `partition_point` for bisection (lower bound equivalent)
- Implement range queries on sorted `Vec` using `partition_point` on both ends

## Rust Application

`src/lib.rs` implements `SortedVec<T: Ord>` with `insert` using `data.partition_point(|x| x < &value)` to find the insertion point. `contains` uses `binary_search` for O(log n) membership. `range_inclusive` finds both endpoints via `partition_point` and slices the `Vec` — O(log n) setup and O(k) iteration.

Sorted vectors are used in `tantivy` (search engine), `roaring` bitmaps, and anywhere sorted iteration with fast lookup is needed on a mostly-static dataset.

## OCaml Approach

OCaml's arrays with `Array.blit` for insertion and binary search:

```ocaml
let binary_search arr target =
  let lo = ref 0 and hi = ref (Array.length arr - 1) in
  let result = ref None in
  while !lo <= !hi do
    let mid = (!lo + !hi) / 2 in
    if arr.(mid) = target then (result := Some mid; lo := !hi + 1)
    else if arr.(mid) < target then lo := mid + 1
    else hi := mid - 1
  done;
  !result
```

The `Base.Array.binary_search` function provides a one-liner. Sorted arrays are the standard for static lookup tables in OCaml.

## Key Differences

1. **`partition_point`**: Rust's `partition_point` is the canonical lower-bound bisection; OCaml's `Base.Array.binary_search` with `~how:` parameter provides equivalent.
2. **Insert cost**: Both languages have O(n) insert due to element shifting; `BTreeSet` / `Set.Make` are O(log n) insert.
3. **Cache efficiency**: Both benefit from contiguous memory access during binary search; sorted arrays beat tree structures for read-heavy workloads.
4. **Range iteration**: Rust's slice indexing after `partition_point` is zero-copy; OCaml's `Array.sub` allocates a new array.

## Exercises

1. Add `remove(&mut self, value: &T) -> bool` using `binary_search` to find the index and `Vec::remove` to remove it.
2. Write `count_in_range(lo: &T, hi: &T) -> usize` using two `partition_point` calls and subtraction.
3. Implement `merge_sorted(a: SortedVec<T>, b: SortedVec<T>) -> SortedVec<T>` in O(n) using the merge step from merge sort.

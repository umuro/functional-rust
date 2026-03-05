# Longest Increasing Subsequence — Comparison

## Core Insight
The patience sorting algorithm maintains a sorted `tails` array. For each element, binary search finds its position — either extending the longest subsequence or replacing an element to keep tails minimal. This achieves O(n log n) vs the naive O(n^2) DP.

## OCaml Approach
- Manual binary search with `ref` cells for lo/hi
- `Array.fold_left max 0` for finding maximum in O(n^2) version
- `Array.iter` with side effects for the patience sort
- No built-in binary search on arrays

## Rust Approach
- `slice::binary_search` returns `Ok(pos)` or `Err(insertion_point)` — perfect for patience sort
- `Vec` as dynamic tails array with `push` and indexing
- Iterator `fold` for a functional patience sort variant
- Pattern matching on `binary_search` result is elegant

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Binary search | Manual implementation | `slice::binary_search()` built-in |
| Search result | Returns index | `Result<Ok(pos), Err(pos)>` |
| Dynamic array | Fixed `Array` + length counter | `Vec` with `push` |
| Fold variant | Less natural (array mutation) | Clean `fold` over iterator |
| Max finding | `Array.fold_left max 0` | `iter().max().unwrap()` |

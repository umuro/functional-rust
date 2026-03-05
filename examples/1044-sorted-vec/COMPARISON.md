# Sorted Vec — Comparison

## Core Insight
A sorted vector trades O(n) insertion for O(log n) search and excellent cache locality. For small to medium collections, it often outperforms tree-based structures. Rust provides `binary_search`, `partition_point`, and slice-based range queries; OCaml uses manual binary search on arrays or sorted list insertion.

## OCaml Approach
- Sorted list: `sorted_insert` walks list to find position — O(n)
- Array binary search: manual implementation with `lo`/`hi`
- Merge of sorted lists: classic merge algorithm
- Deduplication on sorted data: skip consecutive equals
- No built-in `binary_search` on arrays

## Rust Approach
- `Vec::binary_search()`: returns `Ok(idx)` or `Err(insert_point)`
- `partition_point(|x| x < &val)`: first index where predicate fails
- `Vec::insert(pos, val)`: shifts elements right — O(n)
- Slice-based range queries via `partition_point` for both bounds
- `binary_search` for exact match, `partition_point` for bounds

## Comparison Table

| Feature | OCaml | Rust |
|---|---|---|
| Binary search | Manual | `binary_search()` / `partition_point()` |
| Insert | List walk O(n) | `insert(pos, val)` O(n) |
| Search result | Index | `Result<usize, usize>` |
| Range query | Manual slice | `partition_point` × 2 |
| Cache locality | List: poor / Array: good | Vec: excellent |
| Dedup | Manual walk | `dedup()` built-in |

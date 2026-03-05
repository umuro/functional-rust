## Core Insight

Merge sort: split into halves, recursively sort each half, merge sorted halves. O(n log n) guaranteed. The functional version is particularly elegant because merging sorted lists is a natural recursive operation.

## OCaml Approach
- Pattern match on list to split
- Recursive merge of two sorted lists
- No mutation — creates new lists at each step

## Rust Approach
- Split slice at midpoint
- Recursive sort on sub-slices
- Merge into new Vec — or use `sort()` (introsort)

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Split | Pattern match / `List.nth` | `split_at(mid)` |
| Merge | Recursive cons | Push to Vec |
| In-place | No (functional) | Possible but complex |
| Built-in | `List.sort` | `.sort()` (introsort) |

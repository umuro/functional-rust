# Permutations — Comparison

## Core Insight
Three classic approaches: swap-based (in-place), used-flags (separate buffer), and insertion-based (purely functional in OCaml) / Heap's algorithm (iterative in Rust). Each trades off between elegance and efficiency.

## OCaml Approach
- Swap-based: `Array` with index swapping
- Insertion-based: purely functional with `List.concat_map` — most idiomatic OCaml
- Used-flags: imperative with boolean array
- `List.rev` to maintain order

## Rust Approach
- Swap-based: `Vec::swap()` — clean and idiomatic
- Used-flags: `Vec<bool>` with `push`/`pop` on current
- Heap's algorithm: iterative, one swap per permutation — most efficient
- `clone()` needed to snapshot each permutation

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Swap | `let tmp = ...; ... <- ...` (manual) | `Vec::swap(i, j)` (built-in) |
| Functional style | `List.concat_map` + insertion | Not natural (would need `im` crate) |
| Heap's algorithm | Not shown | Iterative with `c` array |
| Snapshot | `Array.to_list` | `.clone()` |
| Space | O(n!) results + O(n) stack | O(n!) results + O(n) stack |

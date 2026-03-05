# Combination Sum — Comparison

## Core Insight
Combination sum is backtracking with an index parameter controlling reuse. Starting at `i` (not `i+1`) allows the same element to be picked again. Sorting enables pruning: if `candidates[i] > remaining`, all later candidates are too large.

## OCaml Approach
- `List.sort compare` for sorting candidates
- `List.iteri` with index filtering for start position
- Prepend `::` + `List.rev` for result building
- `ref` list for accumulating results

## Rust Approach
- `sort()` in-place on `Vec`
- `break` in sorted loop for pruning — cleaner than filtering
- `push`/`pop` on current for backtracking
- Variant: `continue` for duplicate skipping in Combination Sum II

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Sorting | `List.sort compare` (returns new list) | `.sort()` (in-place) |
| Pruning | Index filtering with `List.iteri` | `break` in sorted loop |
| Result building | Prepend `::` + `List.rev` | `push` + `pop` |
| Reuse control | Recurse with same index | Recurse with same `i` |
| Dedup (variant) | Would need explicit skip | `if i > start && arr[i] == arr[i-1]` |

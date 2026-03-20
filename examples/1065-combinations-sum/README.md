📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1065-combinations-sum)**

---

# 1065-combinations-sum — Combination Sum

## Problem Statement

Find all combinations of numbers from a given set that sum to a target, where each number may be used multiple times. This is a backtracking enumeration problem with applications in change-making (all ways to make an amount), spell correction (all word sequences summing to a score), and exam scheduling (all subsets hitting exactly N hours).

The key pruning: sort candidates and break early when the current candidate exceeds the remaining target — this transforms exponential worst case into practical tractability.

## Learning Outcomes

- Implement combination sum using backtracking with reuse
- Sort candidates to enable early termination (pruning)
- Understand the difference from the no-reuse variant (combinations II)
- Return all combinations in canonical form (no duplicates from different orderings)
- Connect to the complete search / generate-and-prune paradigm

## Rust Application

`src/lib.rs` implements `combination_sum` by sorting candidates and using `start` index to prevent revisiting smaller candidates (avoiding `[2, 3]` and `[3, 2]` as separate solutions). The inner loop breaks when `candidates[i] > remaining` — valid because the array is sorted. Each recursive call passes `i` (not `i+1`) as the start, allowing the same element to be reused.

The `start` index is the canonical trick for combination generation: advancing it to `i+1` (instead of `i`) for variants where reuse is not allowed.

## OCaml Approach

```ocaml
let combination_sum candidates target =
  let candidates = List.sort compare candidates in
  let results = ref [] in
  let rec backtrack start remaining current =
    if remaining = 0 then results := List.rev current :: !results
    else
      List.iteri (fun i c ->
        if i >= start && c <= remaining then
          backtrack i (remaining - c) (c :: current)
      ) candidates
  in
  backtrack 0 target [];
  !results
```

The structure is identical. OCaml's list prepend and `List.rev` at collection mirrors Rust's `Vec::push` and `Vec::clone`.

## Key Differences

1. **Mutation vs accumulation**: Rust uses `current.push(c)` / `current.pop()` in-place; OCaml prepends `c :: current` creating new lists at each step.
2. **Sorting**: Both sort before backtracking to enable pruning; Rust sorts in place with `candidates.sort()`, OCaml uses `List.sort compare`.
3. **Index tracking**: Rust uses an explicit `start: usize` parameter; OCaml uses `List.iteri` with an index comparison — less clean.
4. **`itertools`**: The `itertools` crate provides `(0..n).combinations(k)` for non-reuse combinations; Rust has no built-in for the reuse variant.

## Exercises

1. Implement `combination_sum_no_reuse(candidates: &mut [i32], target: i32) -> Vec<Vec<i32>>` where each number can be used at most once (sort + skip duplicates).
2. Write `combination_sum_count(candidates: &[i32], target: i32) -> usize` that counts the number of combinations without collecting them.
3. Add a `max_depth: usize` parameter that limits the maximum number of elements in any single combination.

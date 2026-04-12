📖 **[View on hightechmind.io →](https://hightechmind.io/rust/894-step-by)**

---

# 894-step-by — Step By, Enumerate, Rev
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Structured traversal — skipping elements at regular intervals, attaching position indices, or traversing in reverse — arises constantly in data processing. NumPy's `arange(start, stop, step)`, Python's `range(start, stop, step)`, and SQL's `ROW_NUMBER()` analytic function all serve structured traversal needs. Rust provides three zero-cost adapter methods for these: `.step_by(n)` for strided access, `.enumerate()` for index attachment, and `.rev()` for reversal. These compose cleanly: `.enumerate().rev()` gives reverse-indexed enumeration, and `.step_by(2).enumerate()` gives even-indexed positions.

## Learning Outcomes

- Use `.step_by(n)` to visit every nth element of an iterator
- Use `.enumerate()` to pair elements with zero-based indices
- Use `.rev()` to iterate in reverse over a DoubleEndedIterator
- Compose step_by, enumerate, and rev to produce complex traversal patterns
- Compare with OCaml's `List.filteri`, `List.mapi`, and `List.rev`

## Rust Application

`every_nth` uses `data.iter().step_by(n).copied().collect()`. `range_step` uses `(start..stop).step_by(step)`. `find_with_index` combines `.enumerate().find(|(_, x)| pred(x))`. `format_numbered` uses `.enumerate().map(|(i, s)| format!("{}. {}", i+1, s))` for 1-based display. `reverse_collect` uses `.rev().copied().collect()`. `rev_map` reverses while applying a transformation. Combined: `data.iter().step_by(2).enumerate().rev()` gives `(rev_idx, value)` pairs for even-indexed elements in reverse.

## OCaml Approach

OCaml's `List.filteri (fun i _ -> i mod n = 0) xs` implements step_by. `List.mapi (fun i x -> (i, f i x)) xs` is enumerate-and-map. `List.rev xs` reverses. For arrays: `Array.iteri f arr` provides index with each element. OCaml lacks a generic `step_by` on lists in the standard library — `List.filteri` is the idiomatic substitute. For `Seq`, `Seq.filter (fun _ -> ...)` with a mutable counter or `Seq.zip (Seq.ints 0)` provides indexed access.

## Key Differences

1. **Zero-cost composition**: Rust adapters compose without intermediate allocation; OCaml `List.filteri` followed by `List.mapi` allocates two intermediate lists.
2. **step_by on ranges**: Rust `(0..100).step_by(3)` is a built-in lazy range; OCaml requires explicit `List.init` with stride arithmetic.
3. **1-based indexing**: Rust enumeration is 0-based; OCaml `mapi` is also 0-based. Both require `+1` for 1-based display.
4. **rev composition**: Rust `.step_by(2).rev()` works because `StepBy` implements `DoubleEndedIterator`; OCaml requires `List.rev` before or after filtering.

## Exercises

1. Write `diagonal_elements(matrix: &[Vec<T>]) -> Vec<&T>` using `.enumerate()` to extract the main diagonal.
2. Implement `interleave(a: &[i32], b: &[i32]) -> Vec<i32>` using `.step_by(2)` and `.chain()` without explicit loops.
3. Use `.enumerate().rev()` to print a countdown with positions: `"3: c", "2: b", "1: a"` from input `["a", "b", "c"]`.

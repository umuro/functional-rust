[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 099 — Group Consecutive Equal Elements

## Problem Statement

Group consecutive equal elements of a slice into a `Vec<Vec<T>>`. Adjacent identical elements form one group; a new group starts when the element changes. Input `[1, 1, 2, 2, 2, 3, 1, 1]` produces `[[1,1], [2,2,2], [3], [1,1]]`. Compare with OCaml's accumulator-based recursive approach.

## Learning Outcomes

- Use `Vec<Vec<T>>` as the accumulator for grouped output
- Use `groups.last_mut().unwrap()` to extend the current group in-place
- Start a new group with `groups.push(vec![item.clone()])` when the element changes
- Apply `T: PartialEq + Clone` for equality comparison and group construction
- Map Rust's mutable `Vec` accumulator to OCaml's pure recursive accumulator
- Recognise this as the building block for RLE (run-length encoding)

## Rust Application

The function initialises `groups` with the first element's group. For each subsequent item, it compares with `groups.last().unwrap().last().unwrap()` — the last element of the last group. If equal, it pushes to the current group; if different, it starts a new group. The double `last()` / `last_mut()` pattern is the key idiom for appending to a growing nested vector. Both `T: PartialEq` (for comparison) and `T: Clone` (for copying into groups) are required.

## OCaml Approach

OCaml's `group_by` uses a recursive `aux current group acc` accumulator. `current` is the value being accumulated, `group` is the current run (reversed), and `acc` holds completed groups. At the end, `List.rev (current :: group)` reconstructs the final group in order. Each step either extends the current group or finalises it and starts a new one.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| State | Mutable `Vec<Vec<T>>` | Immutable `aux current group acc` |
| Current group | `last_mut().unwrap().push(x)` | Prepend to `group` list |
| New group | `push(vec![x])` | Recurse with new `current` |
| Reversal | Not needed | `List.rev` for final group |
| Clone | Required (`T: Clone`) | Value semantics (copy by default) |
| Pattern | Mutable accumulation | Tail-recursive accumulation |

Group-by consecutive is the inverse of run-length encoding. The data structure patterns here — nested mutable Vec, or accumulator-threaded recursion — appear throughout list processing and string parsing.

## Exercises

1. Implement `group_by_key<T, K: PartialEq, F: Fn(&T) -> K>(v: &[T], f: F) -> Vec<Vec<T>>` that groups by a derived key.
2. Combine `group_by` with `.map(|g| (g[0].clone(), g.len()))` to implement run-length encoding.
3. Write a version that returns `Vec<(T, usize)>` (value, count) instead of `Vec<Vec<T>>`.
4. Implement `group_by_windows(v: &[T], k: usize) -> Vec<&[T]>` that groups by non-overlapping windows of size `k`.
5. In OCaml, implement `group_by_seq : 'a Seq.t -> 'a list Seq.t` for lazy grouped output.

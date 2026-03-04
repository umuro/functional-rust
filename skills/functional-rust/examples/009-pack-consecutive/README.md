# 009: Pack Consecutive Duplicates

**Difficulty:** 2  **Level:** Beginner

Group consecutive equal elements of a list into sublists.

## The Problem This Solves

Raw sequences often contain repeated values in runs: log levels, sensor readings, characters in a string. Before you can do useful work — count runs, compress data, summarize patterns — you need to group consecutive duplicates.

Without a dedicated grouping function, you end up writing the same `if current == previous` loop everywhere, tracking a mutable "current group" variable. The logic is correct but verbose, and easy to mess up at the boundary when you flush the last group.

`pack` encapsulates the boundary handling: start a group with the first element, extend it when the next element matches, push it and start fresh when the next element differs. The result is a `Vec<Vec<T>>` where each inner vector contains one run.

## The Intuition

Imagine reading a sequence of colored cards one at a time. You put each card on a pile while the color matches. The moment the color changes, you set the pile aside and start a new one. At the end, you collect all the piles.

That's `pack`. The "pile" is `current`, the "set aside" is `result.push(current)`.

No look-ahead needed. No sorting. The algorithm is one pass, left to right.

## How It Works in Rust

```rust
fn pack<T: PartialEq + Clone>(list: &[T]) -> Vec<Vec<T>> {
    if list.is_empty() { return vec![]; }

    let mut result  = vec![];
    let mut current = vec![list[0].clone()];  // start first group

    for i in 1..list.len() {
        if list[i] == list[i - 1] {
            current.push(list[i].clone());    // same as previous: extend group
        } else {
            result.push(current);             // different: commit group
            current = vec![list[i].clone()]; // start fresh group
        }
    }
    result.push(current); // don't forget the last group
    result
}
```

`T: PartialEq` for comparison, `T: Clone` to copy values from the borrowed slice into owned `Vec`s. The key insight is the boundary: you must `push(current)` after the loop to capture the last run.

## What This Unlocks

- **Run-length encoding** — `pack` is the grouping step; counting each inner `Vec`'s length gives you the run lengths (see example 094).
- **Compression algorithms** — many string compression schemes group repeated elements first.
- **Change detection** — the "flush on change" pattern works for any sequence where you care about transitions, not just duplicates.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Accumulator style | Recursive with `current` + `acc` args | Imperative loop with `Vec::push` |
| List prepend | `x :: acc` then `List.rev` | `Vec::push` (append, no reversal needed) |
| Borrowing | Not distinguished | `&[T]` input, `T: Clone` to own the output |
| Slice references | No equivalent | `Vec<&[T]>` gives zero-copy grouping |
| Memory layout | Chain of cons cells | Contiguous blocks (`Vec<Vec<T>>`) |

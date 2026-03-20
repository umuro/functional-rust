[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 100 — Step By

## Problem Statement

Use Rust's `.step_by(n)` to iterate over a range with a custom step size, producing `[start, start+n, start+2n, …]`. Demonstrate steps of 2, 5, 25, and 1 on integer ranges. Compare with OCaml's recursive `step_by` and `Seq.unfold`-based `range_step`.

## Learning Outcomes

- Use `(start..end).step_by(n)` for stepped ranges
- Understand that `.step_by(n)` is an iterator adapter returning `StepBy<Range<T>>`
- Combine `.step_by` with `.collect::<Vec<_>>()` to materialise the sequence
- Map Rust's `.step_by` to OCaml's recursive list builder and `Seq.unfold`
- Recognise `Seq.unfold` as the general anamorphism for lazy sequence generation
- Understand the difference between range steps and array strides

## Rust Application

`(0..10).step_by(2)` produces `[0, 2, 4, 6, 8]`. The range `0..10` provides the bounds; `.step_by(2)` advances by 2 on each `next()`. `.step_by(1)` is a no-op (equivalent to the unstepped range). The adapter is lazy — no values are computed until collected. Using `.step_by` on a range is the idiomatic Rust replacement for C-style `for (int i = 0; i < n; i += step)`.

## OCaml Approach

OCaml's `step_by start stop step` uses a tail-recursive accumulator. `range_step` uses `Seq.unfold (fun n -> if n >= stop then None else Some(n, n + step))` — the anamorphism that produces a lazy sequence from a seed and step function. `Seq.unfold` is more general than `step_by`: it can produce any sequence where the next element depends on the current state.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Built-in | `(start..end).step_by(n)` | Manual recursive or `Seq.unfold` |
| Laziness | Lazy adapter | `Seq.unfold` is lazy; list is eager |
| Step of 1 | Identity (same as range) | Same |
| Exclusive end | `start..end` | Manual `n >= stop` |
| General unfold | `std::iter::successors` | `Seq.unfold` |
| Reverse step | Not directly (would need `.rev()`) | Same |

`Seq.unfold` is a more general primitive than `.step_by`: it can produce sequences where the step size changes, terminates conditionally, or derives the next state from the current in arbitrary ways. Rust's equivalent is `std::iter::successors` or a custom struct iterator.

## Exercises

1. Generate Fibonacci numbers using `std::iter::successors(Some((0u64, 1u64)), |&(a, b)| Some((b, a + b))).map(|(a, _)| a)`.
2. Use `.step_by` to iterate over every third element of a slice via `(0..v.len()).step_by(3).map(|i| v[i])`.
3. Implement `decreasing_step(start: i32, stop: i32, step: usize) -> Vec<i32>` using `.rev()` after `.step_by`.
4. Use `Seq.unfold` in OCaml to implement a sequence that doubles the step on each iteration: 0, 1, 3, 7, 15, …
5. Compare `.step_by(1)` performance vs a plain range — verify they compile to the same output with `cargo bench`.

# List.fold_left — Accumulate a Result

## Problem Statement
Use `List.fold_left` to accumulate a list of integers into a single result: sum, product, and maximum. Implement a generic `fold_left` that mirrors OCaml's standard library function.

## Learning Outcomes
- How OCaml's `List.fold_left f acc xs` maps to Rust's `Iterator::fold(init, f)`
- The left fold pattern: thread an accumulator through a list left-to-right
- Operator sections in OCaml (`( + )`, `( * )`) vs. explicit closures in Rust

## Rust Application
`fold_left<T, U, F>(items: &[T], init: U, f: F) -> U` wraps `Iterator::fold`. The concrete helpers `sum`, `product`, and `max_val` show idiomatic usage. `max_val` uses `reduce` (fold without an initial value) returning `Option<i64>` to handle the empty case.

## OCaml Approach
`List.fold_left ( + ) 0 numbers` uses operator sections as the folding function — `( + )` is the addition operator in prefix position. `List.fold_left max min_int numbers` uses `max` as a two-argument function. The fold is strict and tail-recursive in OCaml's stdlib.

## Key Differences
1. **Operator sections:** OCaml writes `( + )` to use an operator as a first-class function; Rust uses closures `|acc, &x| acc + x`
2. **Empty case:** OCaml's `List.fold_left max min_int` uses `min_int` as the identity; Rust's `reduce` returns `None` for empty input — more principled
3. **Argument order:** OCaml: `f acc x`; Rust's closure: `|acc, x|` — same order, different syntax

## Exercises
1. Implement `running_sum` that returns a `Vec<i64>` of prefix sums using `fold_left`
2. Implement `flatten` using `fold_left` that turns `Vec<Vec<T>>` into `Vec<T>`
3. Implement `count_if` using `fold_left` that counts elements satisfying a predicate

# Example 1123: List.fold_left — Accumulate a Result
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Use `List.fold_left` to reduce a list of integers into a single accumulated result: sum, product, and maximum. Implement a generic `fold_left` that mirrors OCaml's standard library function signature, and demonstrate how the same fold pattern expresses all three computations by varying only the combining function and the initial accumulator value.

## Learning Outcomes

- How OCaml's `List.fold_left f acc xs` maps directly to Rust's `Iterator::fold(init, f)` — same argument order, same semantics, different syntax
- What "left fold" means: the accumulator is updated left-to-right, so `fold_left (+) 0 [1;2;3]` computes `((0+1)+2)+3`
- How OCaml's operator sections (`( + )`, `( * )`, `max`) pass built-in operators as first-class functions, and how Rust uses closures to achieve the same effect
- Why `max_val` uses `reduce` (fold without an initial value) returning `Option<i64>` rather than `fold` with `i64::MIN` — the `Option` makes the empty-list case explicit and type-safe
- How the generic `fold_left<T, U, F>` signature in Rust expresses the same higher-kinded abstraction as OCaml's polymorphic `List.fold_left`

## OCaml Approach

OCaml's standard library exports `List.fold_left : ('a -> 'b -> 'a) -> 'a -> 'b list -> 'a`, a tail-recursive left fold. Operator sections like `( + )` and `( * )` are the addition and multiplication operators in prefix position, usable as any other two-argument function. `List.fold_left max min_int numbers` passes the built-in `max` function directly; `min_int` serves as the identity element for the maximum operation over integers. All three computations in `example.ml` are single expressions — the composability of `fold_left` with first-class operators is the key idiomatic feature.

## Rust Application

`fold_left<T, U, F>(items: &[T], init: U, f: F) -> U` wraps `Iterator::fold`, accepting the slice by reference and the combining function as a generic `Fn(U, &T) -> U`. The concrete helpers `sum` and `product` fold over the slice with closures `|acc, &x| acc + x` and `|acc, &x| acc * x`. `max_val` uses `Iterator::reduce` instead of `fold` because there is no meaningful identity for maximum over an arbitrary integer type — returning `None` for an empty slice is more principled than silently returning `i64::MIN`. All helpers operate on `&[i64]` slices, which are idiomatic for read-only sequence processing in Rust.

## Key Differences

1. **Operator sections:** OCaml writes `( + )` to lift an infix operator into a first-class two-argument function; Rust uses explicit closures `|acc, &x| acc + x` — more verbose but unambiguous about argument types and binding
2. **Empty-list handling:** OCaml's `List.fold_left max min_int` uses `min_int` as the identity (always returns a valid integer); Rust's `reduce` returns `None` for an empty slice, surfacing the absence case in the type rather than hiding it behind a sentinel value
3. **Argument order:** OCaml: `fold_left f acc list`; Rust: `fold(init, |acc, x| ...)` — the accumulator comes first in both, then the combining function in Rust's closure; the conceptual mapping is 1:1
4. **Slice vs. list:** OCaml operates on linked lists (`'a list`); Rust uses contiguous slices (`&[T]`), which allows bounds-checked random access and is cache-friendly, but the fold abstraction looks identical from the caller's perspective

## Exercises

1. Implement `running_sum` using `fold_left` that returns a `Vec<i64>` of prefix sums — after folding `[1, 2, 3, 4]` the result should be `[1, 3, 6, 10]`
2. Implement `flatten` using `fold_left` that turns a `Vec<Vec<T>>` into a flat `Vec<T>` by accumulating with `extend`
3. Implement `count_if` using `fold_left` that counts elements satisfying a predicate `fn(&T) -> bool`, and verify it handles the empty-slice case correctly
4. Implement `fold_right` (right fold) and compare its behavior on a non-associative operation like subtraction against `fold_left` — show that `fold_right (-) [1;2;3] 0` gives a different result than `fold_left (-) 0 [1;2;3]`

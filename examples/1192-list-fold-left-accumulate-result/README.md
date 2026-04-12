# Example 1192: List.fold_left — Accumulate a Result

**Difficulty:** ⭐⭐
**Category:** Lists & HOF
**OCaml Source:** OCaml standard library `List.fold_left`

## Problem Statement

Reduce a list to a single value by applying a combining function and an initial accumulator left-to-right. `List.fold_left` is the foundational higher-order accumulation function in OCaml. This example demonstrates how sum, product, and maximum are all expressed as fold applications, and implements a generic `fold_left` mirroring OCaml's standard library signature.

## Learning Outcomes

- How OCaml's `List.fold_left f acc xs` maps to Rust's `Iterator::fold(init, f)` — same semantics, same left-to-right order, different argument order at the call site
- Why Rust's `reduce` is preferable to `fold` with `i64::MIN` for maximum: `reduce` returns `None` on empty input, making the empty case visible in the type rather than hiding it behind a sentinel value
- How OCaml's operator sections `( + )` and `( * )` lift infix operators to first-class functions, and how Rust uses explicit closures `|acc, &x| acc + x` to achieve the same
- How a generic `fold_left<T, U, F>` in Rust captures the same polymorphism as OCaml's `List.fold_left : ('a -> 'b -> 'a) -> 'a -> 'b list -> 'a`

## OCaml Approach

OCaml's `List.fold_left f acc xs` is a tail-recursive function that applies `f` to the accumulator and each element left to right. Operator sections like `( + )` and `( * )` make passing arithmetic operators as first-class functions concise — `List.fold_left ( + ) 0 numbers` reads almost like math notation. The same function handles sum, product, and maximum by changing only the combining function and the identity value.

## Rust Approach

`Iterator::fold(init, f)` is Rust's direct equivalent. The generic wrapper `fold_left<T, U, F>(items: &[T], init: U, f: F) -> U` mirrors OCaml's argument order (accumulator before list) and operates on borrowed slices. Concrete helpers `sum` and `product` are single-expression wrappers that pass closures for the combining function. `max_val` uses `reduce` to return `Option<i64>` for safe empty-slice handling, rather than relying on a sentinel value like `i64::MIN`.

## Key Differences

1. **Operator sections:** OCaml writes `( + )` to use a built-in infix operator as a first-class value; Rust uses explicit closures `|acc, &x| acc + x` — more verbose, but unambiguous about binding and argument types.
2. **Empty-list safety:** OCaml's `List.fold_left max min_int` silently returns `min_int` for an empty list; Rust's `reduce` surfaces the empty case as `None`, making the caller handle it explicitly.
3. **Argument order:** OCaml: `fold_left f acc list`; Rust's `Iterator::fold`: `iter.fold(init, f)` — the closure comes after the initial value in Rust.
4. **Borrowing:** Rust's `fold` closure receives `&T` (a reference) because the iterator yields references from the slice, requiring explicit dereferencing in patterns like `|acc, &x| acc + x`.

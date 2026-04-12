# Example 1191: List.filter — Select Elements by Predicate

**Difficulty:** ⭐
**Category:** Lists & HOF
**OCaml Source:** OCaml standard library `List.filter`

## Problem Statement

Filter a collection by a boolean predicate, keeping only the elements for which the predicate returns true. `List.filter` is one of the most frequently used higher-order list functions in OCaml, and its Rust equivalent `Iterator::filter` is equally central to idiomatic Rust code.

## Learning Outcomes

- How OCaml's `List.filter pred xs` maps directly to Rust's `Iterator::filter(|x| pred(x))`
- Why Rust requires a `Clone` bound when converting from a borrowed `&[T]` slice to an owned `Vec<T>` output
- How to pass predicates as generic `Fn` trait objects in Rust, and why the borrow in `Fn(&T) -> bool` appears
- How slice pattern matching `[head, rest @ ..]` mirrors OCaml's `x :: rest` for writing explicit recursive filter

## OCaml Approach

OCaml's `List.filter : ('a -> bool) -> 'a list -> 'a list` accepts a first-class function and a list, and returns a new list containing only elements for which the function returns true. The predicate is a normal value — an anonymous function `fun x -> x mod 2 = 0` is passed directly without any special wrapping. Because OCaml lists are immutable, the original list is never modified; a fresh list is built by walking the spine. The recursive version shows this explicitly: `if pred x then x :: filter_rec pred rest else filter_rec pred rest`.

## Rust Approach

Rust's `Iterator::filter` is the idiomatic tool: `items.iter().filter(|x| pred(x)).cloned().collect()`. The iterator produces `&&T` references (the iterator yields `&T` and the filter closure receives `&&T`), so the predicate should take `&T`. The `.cloned()` adapter handles the `T: Clone` requirement to produce an owned `Vec<T>` from borrowed slice elements. A recursive version using slice patterns (`[head, rest @ ..]`) closely mirrors OCaml's list pattern matching.

## Key Differences

1. **Predicate argument type:** OCaml predicates receive `'a` values directly; Rust predicates receive `&T` references because the slice items are borrowed, not owned.
2. **Clone requirement:** OCaml's GC handles value copying invisibly when constructing the filtered list; Rust must explicitly `.cloned()` to produce owned values from a borrowed slice.
3. **Lazy vs. eager:** OCaml's `List.filter` immediately allocates and returns the result list; Rust's `Iterator::filter` is lazy — no work is done until `.collect()` is called.
4. **Recursion:** Both languages express the recursive filter naturally — OCaml via `function` pattern matching, Rust via slice patterns `[head, rest @ ..]`.

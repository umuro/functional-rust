# Example 025: List.map — Transform Every Element
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Apply a transformation function to every element of a list, producing a new list of the same length and same order. This is the foundational higher-order function in functional programming: `map` separates the "what to do to each element" from the "how to iterate," making it the first pattern to master in any functional language. The example provides two Rust implementations — an idiomatic iterator version and a recursive structural version — so you can see both styles side by side.

## Learning Outcomes

- How OCaml's `List.map f xs` maps directly to Rust's `iter().map(f).collect()` iterator pipeline
- How to express structural recursion over a slice using Rust's slice pattern syntax `[head, tail @ ..]`
- Why `[head, tail @ ..]` is the precise analogue of OCaml's `x :: xs` cons-cell decomposition
- The performance trade-off between the iterator form (single allocation at `collect`) and the recursive form (one `Vec` per call frame)
- Why the idiomatic Rust form takes `&[T]` (a slice) rather than `Vec<T>`, preserving flexibility for callers

## OCaml Approach

OCaml ships `List.map` as part of the standard library: `List.map (fun x -> x * 2) [1;2;3;4;5]` gives `[2;4;6;8;10]`. The standard recursive definition is `let rec map f = function | [] -> [] | x :: xs -> f x :: map f xs`. Both forms are idiomatic — library use for production code, explicit recursion for teaching. OCaml's linked-list representation makes pattern matching on `x :: xs` natural: the cons cell structurally separates head from tail with no overhead.

## Rust Application

`map_transform<T, U>(items: &[T], f: impl Fn(&T) -> U) -> Vec<U>` is the idiomatic version: `items.iter().map(f).collect()` builds a lazy chain and materializes it once. The generic signature handles any input element type `T` and any output type `U`, matching OCaml's polymorphism. `map_recursive` mirrors the OCaml recursive definition using slice patterns: `[head, tail @ ..] =>` binds the first element to `head` and the rest of the slice to `tail`, exactly as `x :: xs` does. The closure is passed as `&impl Fn` by reference so it can be re-used across each recursive call without being consumed.

## Key Differences

1. **Canonical idiom:** Rust's standard form uses `iter().map().collect()`; OCaml uses `List.map` or `let rec map`; both abstract over the loop but via different mechanisms
2. **Pattern syntax:** Rust's `[head, tail @ ..]` and OCaml's `x :: xs` are structurally equivalent decompositions, but Rust works on contiguous slice memory while OCaml destructs a heap-allocated cons cell
3. **Allocation model:** The recursive Rust version allocates a new `Vec` at each call frame and extends it; OCaml's recursion allocates cons cells one at a time; the iterator version collects once
4. **Closure passing:** `map_recursive` passes `f` as `&impl Fn` to borrow without consuming; OCaml closures are always heap-allocated references and can be passed freely

## Exercises

1. Implement `map_indexed` that passes both the element and its zero-based index to the transformation function: `f: impl Fn(usize, &T) -> U`
2. Implement `flat_map` (OCaml's `List.concat_map`) that applies a function returning `Vec<U>` to each element and flattens the results into a single `Vec<U>`
3. Implement `filter_map` that applies a function returning `Option<U>` and collects only the `Some` values, then verify it produces the same result as chaining `.filter().map()` on the iterator

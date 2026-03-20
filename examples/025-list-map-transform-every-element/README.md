# List.map — Transform Every Element

## Problem Statement
Apply a transformation function to every element of a list, producing a new list of the same length. Implement both the idiomatic Rust iterator approach and a recursive structural version that mirrors OCaml's `List.map`.

## Learning Outcomes
- How OCaml's `List.map f xs` maps to Rust's `iter().map(f).collect()` pattern
- Structural recursion on slices using Rust's slice pattern matching (`[head, tail @ ..]`)
- The difference between idiomatic Rust (iterator chains) and OCaml-style explicit recursion

## Rust Application
`map_transform<T, U>(items: &[T], f: impl Fn(&T) -> U) -> Vec<U>` uses the iterator chain idiom. `map_recursive` uses slice patterns: `[head, tail @ ..] =>` matches the first element and the rest, mirroring OCaml's `x :: xs` cons-cell decomposition.

## OCaml Approach
`List.map (fun x -> x * 2) numbers` is a standard library function. The recursive definition is `let rec map f = function | [] -> [] | x :: xs -> f x :: map f xs`. Both forms are idiomatic in OCaml.

## Key Differences
1. **Idiom:** Rust's canonical form uses iterator adapters; OCaml uses either `List.map` or explicit `let rec`
2. **Slice patterns:** Rust's `[head, tail @ ..]` is a direct analogue to OCaml's `x :: xs` — both destructure the head from the rest
3. **Performance:** The recursive Rust version allocates intermediate `Vec`s per call; the iterator version collects once. OCaml's linked-list cons cells make recursion O(n) with no extra allocation.

## Exercises
1. Implement `map_indexed` that passes both the element and its index to the function: `f: impl Fn(usize, &T) -> U`
2. Implement `flat_map` (OCaml's `List.concat_map`) that applies a function returning `Vec<U>` to each element and flattens the results
3. Implement `filter_map` that applies a function returning `Option<U>` and collects only the `Some` values

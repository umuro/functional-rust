# List.map — Transform Every Element

## Problem Statement
Apply a transformation function to every element of a list, producing a new list of the same length. Implement both the idiomatic Rust iterator approach and a recursive structural version that mirrors OCaml's `List.map`.

`List.map` is the first function taught in every functional programming course — it abstracts the universal pattern of "do this to every element". Understanding `map` deeply means understanding: (1) it preserves the length of the input, (2) it applies its function to elements independently (enabling parallelism), (3) it is the functor law (identity maps to identity, composition distributes). These properties make `map` the safe, composable workhorse of data transformation pipelines in Spark, React, NumPy, and every functional language's standard library.

## Learning Outcomes
- How OCaml's `List.map f xs` maps to Rust's `iter().map(f).collect()` pattern
- Structural recursion on slices using Rust's slice pattern matching (`[head, tail @ ..]`)
- The difference between idiomatic Rust (iterator chains) and OCaml-style explicit recursion

- Understand that `map` always preserves length: the result has exactly as many elements as the input
- Recognize `.iter().map(f).collect()` as the canonical Rust form and `map_recursive` as the OCaml-style structural version
- Use `map_indexed` (`.enumerate().map(|(i, x)| ...)`) when the index is needed alongside the element

## Rust Application
`map_transform<T, U>(items: &[T], f: impl Fn(&T) -> U) -> Vec<U>` uses the iterator chain idiom. `map_recursive` uses slice patterns: `[head, tail @ ..] =>` matches the first element and the rest, mirroring OCaml's `x :: xs` cons-cell decomposition.

## OCaml Approach
`List.map (fun x -> x * 2) numbers` is a standard library function. The recursive definition is `let rec map f = function | [] -> [] | x :: xs -> f x :: map f xs`. Both forms are idiomatic in OCaml.

`List.map (fun x -> x * 2) numbers` is a standard library function. The recursive definition is `let rec map f = function | [] -> [] | x :: xs -> f x :: map f xs`. Both forms are idiomatic in OCaml. The `function` keyword is shorthand for `fun x -> match x with`, making the recursive definition concise. OCaml's `List.map` is not tail-recursive for large lists — use `List.rev_map` (reversed) followed by `List.rev` for large inputs.

## Key Differences
1. **Idiom:** Rust's canonical form uses iterator adapters; OCaml uses either `List.map` or explicit `let rec`
2. **Slice patterns:** Rust's `[head, tail @ ..]` is a direct analogue to OCaml's `x :: xs` — both destructure the head from the rest
3. **Performance:** The recursive Rust version allocates intermediate `Vec`s per call; the iterator version collects once. OCaml's linked-list cons cells make recursion O(n) with no extra allocation.

1. **Idiom:** Rust's canonical form uses iterator adapters; OCaml uses either `List.map` or explicit `let rec`
2. **Slice patterns:** Rust's `[head, tail @ ..]` is a direct analogue to OCaml's `x :: xs` — both destructure the head from the rest
3. **Performance:** The recursive Rust version allocates intermediate `Vec`s per call; the iterator version collects once. OCaml's linked-list cons cells make recursion O(n) with no extra allocation per step.
4. **Tail recursion:** OCaml's `List.map` is not tail-recursive. Rust's iterator `.map()` is implemented as a lazy adapter — no recursion, no stack growth.

5. **Order preservation:** Both `List.map` in OCaml and `.iter().map()` in Rust preserve the original element order — they process elements sequentially from first to last.

## Exercises
1. Implement `map_indexed` that passes both the element and its index to the function: `f: impl Fn(usize, &T) -> U`
2. Implement `flat_map` (OCaml's `List.concat_map`) that applies a function returning `Vec<U>` to each element and flattens the results
3. Implement `filter_map` that applies a function returning `Option<U>` and collects only the `Some` values

4. **Map with index**: Implement `map_indexed<T, U>(items: &[T], f: impl Fn(usize, &T) -> U) -> Vec<U>` — equivalent to OCaml's `List.mapi`. Use `.iter().enumerate().map(...)`.
5. **Parallel map**: Replace `iter()` with `par_iter()` from the `rayon` crate to implement parallel map. The interface is identical — only the import changes. Explain when this is faster than sequential map.

[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 1000 — List Map
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Apply a function to every element of a list and collect the results. Implement both an iterator-based `map_iter` (idiomatic Rust) and a recursive `map_recursive` (functional style mirroring OCaml). Compare with OCaml's `List.map` and a custom tail-recursive implementation.

## Learning Outcomes

- Use `xs.iter().map(f).collect()` as the idiomatic Rust map operation
- Understand that the iterator chain is lazy — no values are computed until collected
- Implement a tail-recursive accumulator version: build result in reverse, then reverse at end
- Distinguish `&T` from `T` in iterator closures: `map(|x| f(x))` where `x: &T`
- Map Rust's `Iterator::map` to OCaml's `List.map f lst`
- Recognise that `map` is the functor operation: pure transformation without side effects

## Rust Application

`map_iter<T, U, F>(xs: &[T], f: F) -> Vec<U>` wraps `xs.iter().map(f).collect()`. The closure receives `&T` (a reference from `.iter()`), so if ownership is needed, use `.into_iter()` instead. The recursive `map_recursive` uses a helper that appends to a `Vec` accumulator — tail-recursive but less idiomatic. The iterator version is preferred: it preallocates via `collect`'s size hint and can be fused with other adapters.

## OCaml Approach

`List.map (fun x -> x * 2) numbers` is the standard call. The custom `let rec map_recursive f = function | [] -> [] | x :: xs -> f x :: map_recursive f xs` mirrors the Rust recursive version. OCaml's `List.map` is not tail-recursive for large lists (use `List.rev_map` + `List.rev` for tail-recursive mapping). The iterator version in Rust avoids this issue entirely.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Idiomatic | `.iter().map(f).collect()` | `List.map f lst` |
| Element type | `&T` from `.iter()` | `T` (value) |
| Tail recursion | Iterative (iterator) | `List.rev_map` + `List.rev` |
| Lazy | Yes (iterator chain) | No (`List.map` is eager) |
| Type inference | Both `T` and `U` inferred | Fully inferred |
| Ownership | `.iter()` borrows; `.into_iter()` consumes | Value semantics |

`map` is the foundational list transformation. In Rust, it integrates into the iterator chain allowing further adapters before collecting. In OCaml, it returns a new list immediately. Both represent the same mathematical functor: applying a morphism `f: A -> B` to all elements.

## Exercises

1. Map over a `Vec<String>` to produce a `Vec<usize>` of string lengths using `.map(|s| s.len())`.
2. Implement `map_with_index<T, U>(xs: &[T], f: impl Fn(usize, &T) -> U) -> Vec<U>` using `.enumerate()`.
3. Write `map_option<T, U>(xs: &[Option<T>], f: impl Fn(&T) -> U) -> Vec<Option<U>>`.
4. Chain `map` with `filter`: keep only elements whose mapped value is `Some`.
5. In OCaml, implement `map2 : ('a -> 'b -> 'c) -> 'a list -> 'b list -> 'c list` (equivalent to `List.map2` from the standard library).

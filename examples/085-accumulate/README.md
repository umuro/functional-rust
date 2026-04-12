[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 085 — Accumulate (Custom Map)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement a custom `accumulate` function equivalent to `map` — applying a function to every element of a list and collecting the results. Provide three versions: naive recursive, tail-recursive with an accumulator, and idiomatic iterator-based. Compare with OCaml's `List.map` and tail-recursive alternative.

## Learning Outcomes

- Use slice pattern matching `[head, tail @ ..]` for destructuring in Rust
- Understand why naive recursion risks stack overflow on large inputs
- Implement tail-recursive accumulation with `Vec::push` + pre-allocation
- Recognise that `lst.into_iter().map(f).collect()` is the idiomatic Rust equivalent
- Map the OCaml tail-recursive accumulator pattern (`go (f h :: acc) t`) to Rust
- Compare eager list operations in OCaml with iterator laziness in Rust

## Rust Application

The recursive `accumulate` uses a nested `inner` function with `&dyn Fn` to avoid borrow checker issues with a recursive closure. Pattern matching on `[head, tail @ ..]` mirrors OCaml's `h :: t`. The tail-recursive version uses an explicit `Vec` accumulator with `push`, avoiding stack growth. The iterator version `lst.into_iter().map(f).collect()` is the idiomatic Rust equivalent — internally it preallocates and iterates without recursion.

## OCaml Approach

OCaml's `List.map` is defined recursively as `f h :: accumulate f t`. The tail-recursive version builds `f h :: acc` in reverse and calls `List.rev` at the end. OCaml's native `List.map` uses continuation-based tail calls in newer versions. The OCaml code is shorter because list cons `::` is syntactic and pattern matching on lists is built into the language.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| List destructuring | `[head, tail @ ..]` slice pattern | `h :: t` |
| Tail recursion | Iterative `push` in a loop | `go (f h :: acc) t` + `List.rev` |
| Idiomatic version | `.iter().map(f).collect()` | `List.map f lst` |
| Stack safety | Recursive version unsafe on large input | Same — `List.map` may stack overflow |
| Pre-allocation | `Vec::with_capacity(lst.len())` | Not possible with `list` |
| Ownership | `&T` in closure for borrow / `T` for move | Value semantics throughout |

The recursive version illustrates how OCaml idioms translate to Rust, but the take-away is that Rust's iterator protocol handles this cleanly without recursion. Use the iterator version in production; use the recursive version only for educational comparison.

## Exercises

1. Implement `accumulate_filter_map<T, U>(lst: &[T], f: impl Fn(&T) -> Option<U>) -> Vec<U>` that maps and filters in a single pass.
2. Write `accumulate_indexed<T, U>(lst: &[T], f: impl Fn(usize, &T) -> U) -> Vec<U>` that passes the index to the function.
3. Benchmark the three versions for a 1,000,000-element slice of `i32`. Document the results.
4. Implement `accumulate` using `fold_right` (right-associative fold) and explain why it is not tail-recursive.
5. In OCaml, implement `accumulate_lazy` using the `Seq` module so transformation is deferred until materialized.

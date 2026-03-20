[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 081 — Difference List

## Problem Statement

Implement a difference list data structure that provides O(1) append by representing a list as a function `Vec<T> -> Vec<T>`. Compare with OCaml's natural `'a list -> 'a list` function type, and also show a practical `VecBuilder` alternative that amortizes allocation across many append operations.

## Learning Outcomes

- Understand difference lists as function composition rather than data concatenation
- Use `Box<dyn FnOnce(Vec<T>) -> Vec<T>)` to store an owned, one-shot closure
- Recognise why `T: 'static` is required when boxing a closure that captures `T`
- See that `append` is just function composition: `|v| f(g(v))`
- Compare with `VecBuilder` which trades theoretical elegance for practical clarity
- Map Rust's heap-boxed closure to OCaml's native first-class `'a list -> 'a list`

## Rust Application

`DList<T>` wraps `Box<dyn FnOnce(Vec<T>) -> Vec<T>>`. `empty` is the identity closure; `singleton(x)` inserts `x` at position 0 of the tail; `from_vec` prepends a whole vector. `append` composes two `DList`s by nesting: `|v| self.f(other.f(v))` — constant time regardless of list size. `to_vec` materializes the chain by applying the accumulated function to an empty `Vec`. The practical `VecBuilder` collects `Vec` chunks, then builds a final `Vec` with a single capacity allocation — often faster in practice.

## OCaml Approach

OCaml represents a difference list as a plain type alias `type 'a dlist = 'a list -> 'a list`. `empty` is `Fun.id`; `singleton x` is `fun rest -> x :: rest`; `append a b` is `fun rest -> a (b rest)`. No boxing or lifetime annotation needed — functions are first-class values with reference-counted closures. The OCaml version is trivially three lines. Both representations are semantically identical: a difference list is a partially applied list-building function.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Type | `Box<dyn FnOnce(Vec<T>) -> Vec<T>>` | `'a list -> 'a list` |
| Lifetime | `T: 'static` required | No annotation needed |
| `append` | Nested closure in `Box::new` | `fun rest -> a (b rest)` |
| Reuse | One-shot (`FnOnce`) | Multi-use (function values) |
| Practical alt | `VecBuilder` | `Buffer` module |
| Code length | ~50 lines | ~8 lines |

The `FnOnce` constraint is the key difference: Rust closures that capture owned values can only be called once. OCaml closures capture by reference and can be called multiple times. For a difference list used only to materialize once, `FnOnce` is correct; for a reusable combinator, `Fn` (requiring `Clone`) would be needed.

## Exercises

1. Change `DList` to use `Fn` instead of `FnOnce` by requiring `T: Clone`. Verify that the same `DList` can be materialized multiple times.
2. Implement `concat: Vec<DList<T>> -> DList<T>` that folds a vector of difference lists using `append`.
3. Benchmark `DList::append` + `to_vec` vs direct `Vec::extend` for appending 10,000 lists of 100 elements each.
4. Implement a `map<U, F: Fn(T) -> U>(self, f: F) -> DList<U>` on `DList<T>`.
5. In OCaml, implement a difference list for strings and use it to build a comma-separated string of 10,000 items, measuring time against `String.concat`.

[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 097 — Flatten and Flat Map
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Use Rust's `.flatten()` to collapse an iterator of iterables into a single flat stream, and `.flat_map(f)` to map then flatten in one step. Demonstrate on nested vectors, optional values (`Vec<Option<T>>`), and a transformation that expands each element into a mini-sequence. Compare with OCaml's `List.concat` and `List.concat_map`.

## Learning Outcomes

- Use `.flatten()` on any iterator whose `Item: IntoIterator`
- Use `.flat_map(f)` as shorthand for `.map(f).flatten()`
- Flatten `Vec<Option<T>>` to filter out `None` values (since `Option` implements `IntoIterator`)
- Understand that both are lazy — no intermediate collections are allocated
- Map Rust's `.flatten()` to OCaml's `List.concat` and `flat_map` to `List.concat_map`
- Recognise flat_map as the monadic bind for the list/iterator monad

## Rust Application

`vec![vec![1,2], vec![3,4]].into_iter().flatten().collect()` concatenates nested vectors. `.flat_map(|&x| vec![x, x*10])` expands each element into a pair. The `Option` test shows that `.iter().flatten()` on `&[Option<T>]` filters out `None`s — `Option<T>` implements `IntoIterator` with 0 or 1 elements. All operations are lazy: the inner iterators are consumed one at a time.

## OCaml Approach

`List.concat` concatenates a list of lists. `List.concat_map f lst` maps `f` and concatenates. `Seq.flat_map` does the lazy equivalent. OCaml's approach is simpler syntactically; Rust's `.flatten()` is more general — it works on any `Item: IntoIterator`, not just nested lists.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Flatten | `.flatten()` | `List.concat` |
| Flat map | `.flat_map(f)` | `List.concat_map f` |
| Option filter | `.flatten()` on `Option<T>` iter | `List.filter_map` |
| Laziness | Lazy (no intermediate collection) | `List.concat` is eager |
| Generality | Any `IntoIterator` | Lists only (without Seq) |
| Monadic bind | `flat_map` = bind for Iterator | Same semantics |

`flat_map` is the monadic bind operation for the iterator/list monad. Any time you write `.map(f)` where `f` returns a `Vec` or `Option`, and then immediately `.flatten()`, you can replace both with `.flat_map(f)`.

## Exercises

1. Use `flat_map` to split a sentence into individual words: `sentences.iter().flat_map(|s| s.split_whitespace())`.
2. Implement `my_flatten<T>(v: Vec<Vec<T>>) -> Vec<T>` without using `.flatten()` — use `fold` instead.
3. Use `.flatten()` on `Vec<Result<T, E>>` — note it only works with `iter_ok`-style logic; investigate why.
4. Write `expand_range(ranges: &[(i32, i32)]) -> Vec<i32>` that flattens each range into its elements.
5. In OCaml, implement `flat_map_seq : ('a -> 'b Seq.t) -> 'a Seq.t -> 'b Seq.t` for lazy flat mapping.

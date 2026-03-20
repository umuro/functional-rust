[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 1001 — List Filter

## Problem Statement

Keep only elements of a list that satisfy a predicate. Implement three approaches: iterator-based `filter_iter` (idiomatic), `filter_in_place` using `Vec::retain`, and `filter_recursive` (functional style). Compare with OCaml's `List.filter` and a recursive implementation.

## Learning Outcomes

- Use `items.iter().filter(pred).cloned().collect()` for filter-into-new-collection
- Use `items.retain(pred)` for in-place filtering without allocation
- Implement recursive filter with slice patterns and `T: Clone`
- Understand that `.filter(pred)` passes `&&T` when called on `iter()` — predicate receives `&T`
- Map Rust's `retain` to OCaml's `List.filter` (both O(n))
- Distinguish immutable filter (returns new vec) from mutable retain (modifies in place)

## Rust Application

`filter_iter` borrows the slice, filters with `.filter(|x| predicate(x))` (note: `x` is `&&T` here — `predicate` takes `&T`), and `.cloned().collect()` to materialise. `filter_in_place` calls `items.retain(|x| predicate(x))` on a mutable `Vec<T>` — no allocation, O(n) single pass. The recursive version matches on slice patterns and returns a new `Vec`. In practice, use `retain` for in-place, `filter + collect` for functional pipeline.

## OCaml Approach

`List.filter (fun x -> x mod 2 = 0) numbers` is the standard call. The custom recursive `let rec filter_recursive pred lst = match lst with | [] -> [] | head :: tail -> if pred head then head :: filter_recursive pred tail else filter_recursive pred tail` mirrors the logic exactly. OCaml lists are immutable, so all filtering creates a new list — no in-place equivalent exists.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Idiomatic | `.filter(pred).collect()` | `List.filter pred lst` |
| In-place | `Vec::retain(pred)` | Not possible (immutable lists) |
| Predicate arg | `&T` (double ref from iter) | `T` (value) |
| Allocation | `collect` allocates; `retain` does not | Always allocates |
| Laziness | `filter` is lazy | `List.filter` is eager |
| `filter_map` | `.filter_map(f)` | `List.filter_map f` |

`Vec::retain` is a unique Rust capability — filtering in place with O(n) time and O(1) auxiliary space. It is the correct choice when you own the `Vec` and don't need to preserve the original.

## Exercises

1. Use `filter_map` to simultaneously filter and transform: keep only positive numbers and double them.
2. Implement `partition_iter<T: Clone>(xs: &[T], pred: impl Fn(&T)->bool) -> (Vec<T>, Vec<T>)` using `filter + collect` twice. Then compare with `.partition(pred)` for efficiency.
3. Write `remove_duplicates<T: Eq + Hash>(xs: &[T]) -> Vec<&T>` using a `HashSet` for membership tracking.
4. Chain `filter` + `map` + `take(5)` on an infinite range to find the first 5 primes.
5. In OCaml, implement `filter_seq : ('a -> bool) -> 'a Seq.t -> 'a Seq.t` for lazy filtering.

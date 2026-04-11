📖 **[View on hightechmind.io →](https://hightechmind.io/rust/060-option-safe-max)**

---

# Example 060: Option Type — Safe List Maximum

**Difficulty:** ⭐
**Category:** Error Handling
**OCaml Source:** [CS3110 — Options](https://cs3110.github.io/textbook/chapters/data/options.html)

## Problem Statement

Implement `list_max` that returns the maximum element of a list wrapped in `Option`/`Some`, returning `None` for empty lists instead of raising an exception. Also implement `safe_head` and `option_map`.

## Learning Outcomes

- Map OCaml's `option` type to Rust's `Option<T>` (they're nearly identical)
- Use pattern matching on `Some`/`None` in both languages
- Chain Option operations with `map` instead of nested matching
- Compare recursive and iterator-based approaches to optional results
- See how both languages eliminate null pointer errors at the type level

## OCaml Approach

OCaml's `option` type (`Some x | None`) is the idiomatic way to represent partial functions. Pattern matching makes handling both cases explicit and compiler-checked.

## Rust Approach

1. **Idiomatic:** `iter().copied().max()`, `first().copied()`, `Option::map`
2. **Recursive:** Slice pattern matching `[head, tail @ ..]` mirroring OCaml's `h :: t`
3. **Fold-based:** `split_first()` + `fold` for iterative maximum

## Key Differences

1. **Almost identical:** `Option<T>` in Rust ≈ `'a option` in OCaml — same constructors, same philosophy
2. **Method richness:** Rust's `Option` has 40+ methods (`map`, `and_then`, `unwrap_or`, `?` operator); OCaml's is simpler
3. **The `?` operator:** Rust can propagate `None` with `?` (like `split_first()?`); OCaml uses `match` or `Option.bind`
4. **Copying:** Rust needs `.copied()` to go from `Option<&T>` to `Option<T>`; OCaml doesn't distinguish
5. **Iterator integration:** Rust's `max()` returns `Option` natively — no need for a custom function

1. **Empty collection safety:** `max()` on an empty iterator returns `None` — safe. OCaml's `List.fold_left max min_int []` would return `min_int` — not safe if the list might be legitimately empty vs accidentally empty.
2. **`Option` as safe max:** `iter.max()` returns `Option<T>` — the caller must decide what to do with an empty collection. This forces correct handling instead of silently returning a sentinel.
3. **`Iterator::max()` is lazy:** It processes elements one by one — no intermediate allocation. OCaml's `List.fold_left (fun acc x -> max acc x) (List.hd list) (List.tl list)` is equivalent but more verbose.
4. **Custom max by key:** `iter.max_by_key(|x| x.score)` finds the maximum by a derived key. OCaml: `List.fold_left (fun acc x -> if x.score > acc.score then x else acc) first rest`.

## Exercises

1. Implement `safe_min` alongside `safe_max`, then write `safe_range` that returns `Option<(T, T)>` with the min and max in a single pass.
2. Write `safe_max_by_key` that accepts a key extraction function `f: &T -> K` and returns the element with the greatest key, returning `None` for an empty list.
3. Implement `top_n` that returns the `n` largest elements from a slice as a sorted `Vec<T>`, using `Option` throughout to handle edge cases (empty slice, `n > len`).

4. **Top-k**: Implement `top_k<T: Ord>(iter: impl Iterator<Item = T>, k: usize) -> Vec<T>` that returns the k largest elements, using a min-heap of size k (`BinaryHeap` in Rust can be adapted).
5. **Safe statistics**: Implement `safe_stats(data: &[f64]) -> Option<(f64, f64, f64)>` returning `(min, max, mean)` — return `None` for empty input, using `Iterator::min_by`, `max_by`, and `sum` / `count`.

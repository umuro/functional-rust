📖 **[View on hightechmind.io →](https://hightechmind.io/rust/060-option-safe-max)**

---

# Example 060: Option Type — Safe List Maximum
**Difficulty:** ⭐  
**Category:** Functional Programming  



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

## Exercises

1. Implement `safe_min` alongside `safe_max`, then write `safe_range` that returns `Option<(T, T)>` with the min and max in a single pass.
2. Write `safe_max_by_key` that accepts a key extraction function `f: &T -> K` and returns the element with the greatest key, returning `None` for an empty list.
3. Implement `top_n` that returns the `n` largest elements from a slice as a sorted `Vec<T>`, using `Option` throughout to handle edge cases (empty slice, `n > len`).

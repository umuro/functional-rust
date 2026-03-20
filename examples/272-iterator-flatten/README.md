📖 **[View on hightechmind.io →](https://hightechmind.io/rust/272-iterator-flatten)**

---

# 272: One-Level Flattening with flatten()

## Problem Statement

Nested collections are ubiquitous: a list of lists of results, a tree traversal producing sub-lists per node, or an iterator of options where only `Some` values matter. The `flatten()` adapter removes exactly one level of nesting — turning `Iterator<Item = Iterator<Item = T>>` into `Iterator<Item = T>`. It is the foundation for monadic composition and list comprehensions, and is the building block behind `flat_map()`.

## Learning Outcomes

- Understand `flatten()` as exactly one level of de-nesting for any `IntoIterator`-yielding iterator
- Use `flatten()` on `Vec<Vec<T>>` to produce a flat sequence
- Apply `flatten()` on `Vec<Option<T>>` to skip `None` values and collect `T`s
- Recognize `flat_map(f)` as `map(f).flatten()` — the fundamental identity

## Rust Application

`Iterator::flatten()` requires the iterator's `Item` to implement `IntoIterator`. It yields each element from each inner iterable in sequence:

```rust
// Flatten nested Vec
let nested = vec![vec![1i32, 2], vec![3, 4]];
let flat: Vec<i32> = nested.into_iter().flatten().collect();
// [1, 2, 3, 4]

// Flatten Options: None values produce zero elements
let opts: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
let values: Vec<i32> = opts.into_iter().flatten().collect();
// [1, 3]

// Option::flatten: two levels of Option -> one
assert_eq!(Some(Some(42i32)).flatten(), Some(42));
assert_eq!(Some(None::<i32>).flatten(), None);
```

## OCaml Approach

OCaml provides `List.concat` for flattening a list of lists, and `List.filter_map Fun.id` for filtering options. The `Seq` module provides `Seq.flat_map` for lazy flattening:

```ocaml
let flat = List.concat [[1;2]; [3;4]]
(* [1; 2; 3; 4] *)

let values = List.filter_map Fun.id [Some 1; None; Some 3]
(* [1; 3] *)
```

## Key Differences

1. **Generic nesting**: Rust's `flatten()` works on any `IntoIterator` item — `Vec`, `Option`, `Result`, ranges, custom types; OCaml's `List.concat` is list-of-lists specific.
2. **Option flattening**: Rust's `Option::flatten()` method (not the iterator adapter) collapses `Option<Option<T>>` — a different but related operation.
3. **One level only**: Both `flatten()` and `List.concat` remove exactly one level of nesting — for deeper nesting, compose multiple `flatten()` calls.
4. **Performance**: Both implementations are lazy in Rust (iterator) and strict in OCaml (list operations build new lists).

## Exercises

1. Flatten a `Vec<Vec<String>>` of paragraphs into a flat `Vec<String>` of sentences.
2. Use `flatten()` on `Vec<Option<i32>>` to collect only the `Some` values, then compute their sum.
3. Implement a tree traversal that yields child nodes at each level using `flat_map()`, then replace `flat_map` with explicit `map().flatten()` to verify equivalence.

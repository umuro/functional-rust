📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1124-listfilter-select-elements-by-predicate)**

---

# 1124-listfilter-select-elements-by-predicate — List.filter: Select Elements by Predicate

## Problem Statement

`filter` selects elements from a collection that satisfy a predicate, discarding the rest. It is the second pillar of functional data processing (alongside `map` and `fold`), appearing in every query language, data pipeline, and UI list filtering operation.

OCaml's `List.filter` and Rust's `Iterator::filter` both express this operation, but with different evaluation models and memory characteristics.

## Learning Outcomes

- Use `Iterator::filter` with a predicate closure to select elements
- Implement recursive filter mirroring OCaml's `List.filter` structure
- Combine `filter` with `map` in a pipeline
- Understand the difference between `filter` (returns references) and `filter_map` (transforms while filtering)
- Apply filter to real data: removing None values, filtering by field value

## Rust Application

The idiomatic Rust implementation:

```rust
pub fn filter_evens(list: &[i32]) -> Vec<i32> {
    list.iter().filter(|&&x| x % 2 == 0).copied().collect()
}
```

Note the double reference dereference `&&x` — `filter` gives `&&T` when applied to `iter()` which yields `&T`. Using `.copied()` converts `&i32` to `i32` before collecting. `filter_map` combines filter and map in one step, avoiding `None` values.

## OCaml Approach

```ocaml
let rec filter f = function
  | [] -> []
  | x :: rest -> if f x then x :: filter f rest else filter f rest

(* Standard library version *)
let filter_evens lst = List.filter (fun x -> x mod 2 = 0) lst
```

OCaml's `List.filter` traverses the list once, building a new list with only matching elements.

## Key Differences

1. **Double reference**: Rust's `iter().filter(|&&x| ...)` requires double dereference due to the reference layer added by `iter`; using `iter().copied().filter(|&x| ...)` or `iter().filter(|x| **x % 2 == 0)` are alternatives.
2. **Laziness**: Rust's `filter` is lazy — it does not evaluate until consumed; OCaml's `List.filter` traverses the list immediately.
3. **`filter_map`**: Rust provides `filter_map(|x| ...)` for combined filter+transform; OCaml has `List.filter_map` in `Base` (not stdlib).
4. **Memory**: Rust's filter produces a new `Vec` after `.collect()`; OCaml's `List.filter` builds a new linked list.

## Exercises

1. Write `partition_by<T, F: Fn(&T) -> bool>(list: &[T], pred: F) -> (Vec<T>, Vec<T>)` using `Iterator::partition`.
2. Implement `filter_map_result<T, E, F: Fn(T) -> Result<T, E>>(list: Vec<T>, f: F) -> (Vec<T>, Vec<E>)`.
3. Chain `filter` and `map` to extract all valid email domains from a list of strings: filter valid emails, map to extract the domain after `@`.

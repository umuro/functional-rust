📖 **[View on hightechmind.io →](https://hightechmind.io/rust/259-iterator-flat-map)**

---

# 259: Flattening with flat_map()

## Problem Statement

Many real-world transformations are one-to-many: splitting a sentence into words, expanding a range for each element, or parsing optional values from strings. A plain `map()` yields `Iterator<Item = Iterator<...>>` — a nested structure that requires an additional `flatten()` call to linearize. The `flat_map()` combinator (also known as `bind` or `>>=` in monadic contexts) fuses these two operations, mapping each element to an iterable and immediately flattening the result into a single stream.

## Learning Outcomes

- Understand `flat_map(f)` as equivalent to `map(f).flatten()` — the monadic bind for iterators
- Use `flat_map()` to expand one element into multiple values
- Filter while transforming by returning empty iterators for rejected elements
- Recognize `flat_map()` as the same as Haskell's `concatMap` and OCaml's `List.concat_map`

## Rust Application

`flat_map(f)` calls `f` on each element and expects `f` to return something `IntoIterator`. The results are yielded element by element as if they came from a single flat sequence:

```rust
// Expand: each number n expands to range 0..n
let result: Vec<i32> = [1i32, 2, 3].iter().flat_map(|&n| 0..n).collect();
// [0, 0, 1, 0, 1, 2]

// Filter-parse: skip parse errors by returning empty iter on failure
let nums: Vec<i32> = ["1", "x", "2"].iter()
    .flat_map(|s| s.parse::<i32>())
    .collect();
// [1, 2] — "x" produces Err, which iterates to zero elements
```

## OCaml Approach

OCaml provides `List.concat_map f xs` (or `List.map f xs |> List.concat` for older versions), which is exactly `flat_map`. The monadic bind `>>=` for the list monad is defined as `fun xs f -> List.concat_map f xs`:

```ocaml
let words = List.concat_map String.split_on_char [' '] ["hello world"; "foo bar"]
(* ["hello"; "world"; "foo"; "bar"] *)
```

## Key Differences

1. **Name**: Rust calls it `flat_map`; Haskell calls it `concatMap`; OCaml uses `List.concat_map`; all express the same monadic bind operation.
2. **Return type flexibility**: Rust's `flat_map` accepts any `IntoIterator`, including `Option` and `Result` which iterate to 0 or 1 elements — enabling inline filtering.
3. **Laziness**: Rust processes lazily; OCaml's `List.concat_map` builds a new list eagerly.
4. **Error handling integration**: Returning `Result::ok()` or `Option` from `flat_map` naturally filters failures without a separate `filter()` call.

## Exercises

1. Split a `Vec<String>` of sentences into individual words using `flat_map()` and `split_whitespace()`.
2. Use `flat_map()` with `Option` to look up each key in a map and collect only the found values into a `Vec`.
3. Implement `flat_map` from scratch using only `map()` and `flatten()` and verify it produces identical results.

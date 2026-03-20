📖 **[View on hightechmind.io →](https://hightechmind.io/rust/903-iterator-flat-map)**

---

# 903-iterator-flat-map — Iterator flat_map

## Problem Statement

Many operations produce zero, one, or multiple outputs per input: tokenizing a sentence produces multiple words, parsing produces either a value or nothing, expanding a range produces many numbers. `.flat_map(f)` = `.map(f).flatten()` handles all three cases in one operation. In category theory, this is the monadic bind (`>>=`) — the mechanism that lets you chain computations where each step can produce zero or more results. OCaml has `List.concat_map`. Haskell has `concatMap` and `(>>=)` for lists. This is one of the most powerful iterator operations.

## Learning Outcomes

- Use `.flat_map(f)` as the composition of map and flatten
- Produce zero outputs (silently drop) by mapping to empty iterators
- Produce multiple outputs by mapping to iterators with multiple elements
- Use `.flat_map(|s| s.parse::<i32>())` to parse-and-filter in one pass
- Compare with OCaml's `List.concat_map` and Haskell's `concatMap`

## Rust Application

`words_from_sentences` uses `.flat_map(|s| s.split_whitespace())` — each sentence produces multiple words, all collected flat. `expand_ranges` maps each `n` to `0..n` (a range with n elements) and flattens. `parse_valid` uses `.flat_map(|s| s.parse::<i32>())` — `Result<i32, _>` implements `IntoIterator` (0 or 1 elements), so failed parses produce zero outputs and successful ones produce one. `concat_map` is the explicit generic version mirroring OCaml's `List.concat_map`.

## OCaml Approach

`List.concat_map: ('a -> 'b list) -> 'a list -> 'b list` (since 4.10) is the direct equivalent. Before 4.10: `List.concat (List.map f xs)`. For optional results: `List.filter_map: ('a -> 'b option) -> 'a list -> 'b list` is more idiomatic than flat_map with option. `String.split_on_char ' ' s |> List.concat_map (String.split_on_char '\n')` chains multiple splits. For sequences: `Seq.flat_map` provides lazy flat_map.

## Key Differences

1. **Result as iterator**: Rust `Result<T, E>` implements `IntoIterator` (Ok yields T, Err yields nothing), enabling parse-and-filter with `.flat_map(|s| s.parse())`; OCaml uses `filter_map` with `option`.
2. **Laziness**: Rust `.flat_map()` is lazy; OCaml `List.concat_map` is eager.
3. **Monadic identity**: Both languages express the list monad's bind as flat_map; Rust has no trait for this abstraction, OCaml uses it via explicit function application.
4. **Zero outputs**: Both languages naturally drop zero-output cases — Rust via empty iterators, OCaml via returning `[]` or `None`.

## Exercises

1. Use `.flat_map()` to implement `expand_template(template: &str, vars: &HashMap<&str, Vec<&str>>) -> Vec<String>` that expands each `{var}` into all possible values.
2. Write `parse_and_validate<T, E, F>(strings: &[&str], parse: F) -> Vec<T>` where F returns `Result<T, E>`, silently dropping errors.
3. Implement `ngrams(text: &str, n: usize) -> Vec<Vec<&str>>` using `.flat_map()` to generate all n-gram windows across sentences.

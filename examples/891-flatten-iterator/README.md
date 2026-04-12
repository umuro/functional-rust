📖 **[View on hightechmind.io →](https://hightechmind.io/rust/891-flatten-iterator)**

---

# 891-flatten-iterator — Flatten Iterator
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Nested data structures are pervasive: lists of lists, optional values that may or may not be present, results that may succeed or fail. Flattening removes exactly one level of nesting, collapsing `Vec<Vec<T>>` into `Vec<T>`, `Option<Option<T>>` into `Option<T>`, or `Vec<Option<T>>` into `Vec<T>`. This is formally the monadic `join` operation. In Haskell, `concat` and `>>=` (bind) express this. OCaml has `List.concat` and `Option.join`. Rust's `.flatten()` and `.flat_map()` (which is `map` followed by `flatten`) are the standard tools. This example covers all the flatten patterns across nested containers.

## Learning Outcomes

- Use `.flatten()` to remove exactly one level of iterator nesting
- Use `.flat_map(f)` as the composition of `.map(f).flatten()`
- Flatten `Vec<Option<T>>` to keep only `Some` values (equivalent to `filter_map`)
- Understand `.flatten()` as the monadic `join` operation
- Compare with OCaml's `List.concat`, `Option.join`, and `List.concat_map`

## Rust Application

`flatten_vecs` converts `Vec<Vec<T>>` using `.into_iter().flatten().collect()`. `words_in_sentences` uses `.flat_map(|s| s.split_whitespace())` to extract all words across sentences. `flatten_options` flattens `Vec<Option<T>>` — each `Option<T>` is itself an iterator (0 or 1 elements). `flatten_option_option` calls `.flatten()` on `Option<Option<T>>`. `deep_flatten` chains two flattens for two levels of nesting. The key principle: `.flatten()` removes exactly one level; compose multiple for deeper nesting.

## OCaml Approach

OCaml's `List.concat: 'a list list -> 'a list` flattens one level. `List.concat_map: 'a list -> ('a -> 'b list) -> 'b list` is `flat_map`. `Option.join: 'a option option -> 'a option` flattens nested options. `List.filter_map: ('a -> 'b option) -> 'a list -> 'b list` flattens `list of options` while mapping. For sequences, `Seq.flat_map` provides lazy flattening. OCaml lacks a single `flatten` that works uniformly across container types — each has its own.

## Key Differences

1. **Uniform interface**: Rust `.flatten()` works on any `Iterator<Item = IntoIterator>`; OCaml has separate `List.concat`, `Option.join`, `Array.concat` per type.
2. **Option as iterator**: Rust `Option<T>` implements `IntoIterator` (0 or 1 elements), enabling `.flatten()` on `Vec<Option<T>>`; OCaml requires `List.filter_map`.
3. **Monadic identity**: Both express `join = flatten` from category theory, but Rust makes it syntactically uniform across container types.
4. **Laziness**: Rust `.flatten()` is lazy when chained; OCaml `List.concat` is always eager.

## Exercises

1. Use `.flat_map()` to implement `combinations(xs: &[i32], k: usize) -> Vec<Vec<i32>>` generating all k-element subsets.
2. Implement `flatten_result_ok<T, E>(results: Vec<Result<Vec<T>, E>>) -> Result<Vec<T>, E>` that flattens on success or returns the first error.
3. Write `cartesian_product(a: &[i32], b: &[i32]) -> Vec<(i32, i32)>` using a single `.flat_map()` and `.map()` chain.

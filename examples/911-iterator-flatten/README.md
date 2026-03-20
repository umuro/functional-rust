📖 **[View on hightechmind.io →](https://hightechmind.io/rust/911-iterator-flatten)**

---

# 911-iterator-flatten — Iterator Flatten
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Nested collections arise naturally: a document is a list of paragraphs, each paragraph is a list of sentences, each sentence is a list of words. Collapsing one level of this nesting — "all words in the document" — is the flatten operation. Formally, it is the monadic `join` from category theory: `join :: m (m a) -> m a`. Haskell's `concat`, OCaml's `List.concat`, and Rust's `.flatten()` all implement this. Understanding flatten as `join` reveals why `flat_map` = `map` + `flatten` = monadic bind (`>>=`): it is the fundamental operation of every monad.

## Learning Outcomes

- Use `.flatten()` to remove exactly one level of iterator nesting
- Understand that `.flatten()` is the monadic `join` and `.flat_map()` is monadic bind
- Apply `.flatten()` to `Vec<Option<T>>` using Option's IntoIterator implementation
- Use `Option::flatten()` to collapse `Option<Option<T>>` to `Option<T>`
- Compare with OCaml's `List.concat` and `Option.join`

## Rust Application

`flatten_vecs` uses `nested.into_iter().flatten().collect()` — the `Vec<T>` IntoIterator implementation yields one element per inner element. `flatten_options` uses the same `.flatten()` on `Vec<Option<T>>` because `Option<T>` implements `IntoIterator` (0 or 1 elements). `flatten_option_option` calls `.flatten()` on `Option<Option<T>>` directly (the `Option::flatten` method, not iterator flatten). `words_to_chars` uses `.flat_map(|w| w.chars())` — an explicit flat_map is more readable than `.map(...).flatten()` for character extraction.

## OCaml Approach

`List.concat: 'a list list -> 'a list` flattens one level. `Option.join: 'a option option -> 'a option` (since 4.08) flattens nested options. `List.filter_map Some xs = xs` is the identity — OCaml uses `List.filter_map` for optional values rather than "Option as list." `List.concat_map f xs = List.concat (List.map f xs)` is the flat_map equivalent.

## Key Differences

1. **Option as iterable**: Rust `Option<T>` implements `IntoIterator` (0 or 1 elements), enabling uniform `.flatten()` over `Vec<Option<T>>`; OCaml requires `List.filter_map id`.
2. **One-level guarantee**: Both `.flatten()` and `List.concat` remove exactly one level — no recursive deep-flattening.
3. **join identity**: Both express monadic join; Rust via the Iterator trait's implementation for iterable items; OCaml via the specific `join`/`concat` functions per type.
4. **Laziness**: Rust `.flatten()` is lazy; OCaml `List.concat` allocates eagerly.

## Exercises

1. Write `flatten_result_values<T, E: Clone>(results: &[Result<Vec<T>, E>]) -> Result<Vec<T>, E>` that concatenates all Ok vectors or returns the first Err.
2. Implement `flatten_n_levels<T: Clone>(nested: Vec<Vec<Vec<T>>>) -> Vec<T>` using two consecutive `.flatten()` calls.
3. Use `.flatten()` to implement `optional_chain<A, B, C>(fa: Option<A>, f: impl Fn(A) -> Option<B>, g: impl Fn(B) -> Option<C>) -> Option<C>`.

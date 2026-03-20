📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1009-collecting-results)**

---

# 1009-collecting-results — Collecting Results
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

When processing a list of inputs that each produce a `Result`, you face a design decision: stop at the first error, or collect all values (or all errors). The "stop at first error" pattern is the most common, and Rust's standard library makes it a one-liner through the `FromIterator` implementation for `Result`.

Calling `.collect::<Result<Vec<T>, E>>()` on an `Iterator<Item = Result<T, E>>` will short-circuit at the first `Err`, returning it immediately. All `Ok` values are accumulated. If all items succeed, you get `Ok(Vec<T>)`. This mirrors the `sequence` operation from functional programming.

## Learning Outcomes

- Use `.collect()` to transform an iterator of `Result`s into a single `Result<Vec<T>, E>`
- Understand the short-circuit behaviour and its implications for laziness
- Compare the `collect()` approach to `try_fold` and explicit loops
- Know when to use partition instead of collect when you want to keep partial results
- Appreciate how `FromIterator<Result<T, E>>` is implemented in the standard library

## Rust Application

`src/lib.rs` provides three implementations of the same operation. `parse_all` uses `.collect()` — the idiomatic single-expression approach. `parse_all_manual` uses a `for` loop with `?` for clarity. `parse_all_fold` uses `try_fold` explicitly. All three short-circuit on the first bad input.

The test `test_short_circuit_behavior` illustrates lazy evaluation: the iterator may stop processing elements after encountering an `Err`, potentially never visiting later items.

## OCaml Approach

OCaml lacks a direct equivalent but the pattern is expressible with `List.fold_left`:

```ocaml
let sequence results =
  List.fold_left (fun acc r ->
    match acc, r with
    | Ok xs, Ok x -> Ok (xs @ [x])
    | Error e, _ -> Error e
    | _, Error e -> Error e
  ) (Ok []) results
```

The `Base` library provides `List.map ~f |> Or_error.all` for the same effect.

## Key Differences

1. **Trait-based dispatch**: Rust's `collect` is driven by `FromIterator`, a trait impl in std; OCaml needs explicit library functions or custom code.
2. **Laziness**: Rust iterators are lazy, so `collect` on a `map` pipeline does not build an intermediate `Vec<Result>`; OCaml `List.map` is strict.
3. **Short-circuit guarantee**: Rust's `FromIterator<Result>` is guaranteed to stop at first `Err`; OCaml implementations vary by library.
4. **Type inference**: Rust requires a turbofish or type annotation to select the `Result<Vec<T>, E>` interpretation; OCaml infers from context.

## Exercises

1. Write a `parse_all_keep_errors` function that returns `(Vec<i64>, Vec<String>)` — all successes and all errors — using `Iterator::partition`.
2. Modify `parse_all` to skip errors silently with `filter_map(|r| r.ok())` instead of short-circuiting. Compare the signatures.
3. Implement a `collect_first_n_ok(inputs: &[&str], n: usize) -> Result<Vec<i64>, String>` that succeeds only when at least `n` inputs parse successfully.

📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1020-try-fold)**

---

# 1020-try-fold — try_fold

## Problem Statement

`fold` (reduce) is the fundamental operation of functional programming: reduce a sequence to a single value by combining elements with an accumulator. When the combining function is fallible — parsing, validation, arithmetic with overflow — you need a variant that short-circuits on the first error.

`Iterator::try_fold` is Rust's answer. It combines the ergonomics of `?` with the power of fold, stopping as soon as the combining function returns `Err`. This is the engine behind `Iterator::try_for_each`, `Iterator::sum` (for types that can overflow), and the `collect::<Result<Vec<_>, _>>()` implementation.

## Learning Outcomes

- Use `Iterator::try_fold` to fold with short-circuit on error
- Understand how `try_fold` relates to `fold` and explicit `for`/`?` loops
- Apply `try_fold` to accumulate state while validating inputs
- Recognize `try_fold` as the primitive that `collect::<Result<...>>` is built on
- Handle overflow-safe arithmetic using `checked_*` methods inside `try_fold`

## Rust Application

`src/lib.rs` shows three use cases. `sum_positive` rejects any negative number while summing. `concat_limited` builds a string but fails if the result would exceed a maximum length. `product_no_overflow` uses `i64::checked_mul` inside `try_fold` to detect integer overflow and returns `Err` rather than panicking.

The `?` operator inside the `try_fold` closure is syntactic sugar for the `Try` trait, which is the same mechanism the standard library uses internally to implement `collect::<Result<...>>()`.

## OCaml Approach

OCaml's equivalent is `List.fold_left` with an explicit `Result` accumulator:

```ocaml
let try_fold f init list =
  List.fold_left (fun acc x ->
    match acc with
    | Error _ -> acc
    | Ok state -> f state x
  ) (Ok init) list
```

This continues iterating even after an error (just passing the error through), which is slightly less efficient than Rust's early termination. A fully lazy version requires a recursive approach.

## Key Differences

1. **Early termination**: Rust's `try_fold` stops the iterator immediately on `Err`; the OCaml fold-with-accumulator pattern visits all elements even after failure.
2. **Lazy vs strict**: Rust iterators are lazy so `try_fold` short-circuits work correctly; OCaml lists are strict so the loop always runs to completion.
3. **`Try` trait integration**: Rust's `try_fold` works with any type implementing `Try` (including `Option`); OCaml's manual approach requires separate versions per monad.
4. **`checked_*` methods**: Rust has `checked_add`, `checked_mul`, etc. returning `Option`; OCaml uses exception-based overflow detection or the `Zarith` library.

## Exercises

1. Write a `try_fold_with_index` function that wraps `try_fold` to pass the current index along with each element to the combining function.
2. Implement `parse_csv_row(row: &str, expected_cols: usize) -> Result<Vec<i64>, String>` using `try_fold` to parse each comma-separated field.
3. Use `try_fold` to implement `all(pred) -> Result<bool, E>` for a fallible predicate — returning `Err` if any predicate call fails, `Ok(true)` if all pass, `Ok(false)` if any return false.

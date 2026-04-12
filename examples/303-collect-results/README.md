📖 **[View on hightechmind.io →](https://hightechmind.io/rust/303-collect-results)**

---

# 303: Collecting Iterator<Result<T>> into Result<Vec<T>>
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Parsing a batch of inputs where each might fail presents a choice: fail fast on the first error, or collect all errors. The `collect::<Result<Vec<T>, E>>()` pattern implements fail-fast: if any element produces `Err`, the entire collection short-circuits and returns that `Err`. This is the most common pattern for validating a batch of inputs — parse all or fail on the first invalid one.

## Learning Outcomes

- Understand that `collect::<Result<Vec<T>, E>>()` short-circuits on the first `Err`
- Use this pattern to validate that all inputs in a batch are well-formed
- Recognize the difference from `filter_map(|r| r.ok())` which silently drops errors
- Combine with `?` to propagate batch validation errors cleanly

## Rust Application

When collecting into `Result<Vec<T>, E>`, the iterator stops at the first `Err` and returns it:

```rust
// Parse all strings — fails on first error
pub fn parse_all(inputs: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    inputs.iter().map(|s| s.parse::<i32>()).collect()
}
// parse_all(&["1", "2", "3"]) -> Ok([1, 2, 3])
// parse_all(&["1", "x", "3"]) -> Err(parse error for "x")

// Double all values — same pattern with transformation
pub fn double_all(inputs: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    inputs.iter().map(|s| s.parse::<i32>().map(|n| n * 2)).collect()
}
```

## OCaml Approach

OCaml requires an explicit fold for this pattern:

```ocaml
let parse_all inputs =
  List.fold_right (fun s acc ->
    match int_of_string_opt s with
    | None -> Error (Printf.sprintf "not a number: %s" s)
    | Some n -> Result.map (fun lst -> n :: lst) acc
  ) inputs (Ok [])
```

The fold short-circuits naturally when `acc` is `Error _` — matching Rust's behavior.

## Key Differences

1. **Conciseness**: Rust's `collect::<Result<Vec<_>, _>>()` is a single method call; OCaml requires an explicit fold.
2. **Short-circuit position**: Rust stops at the first error in iteration order (left to right); the OCaml `fold_right` stops from the right — order matters.
3. **vs partition**: `collect::<Result<Vec<_>>>` gives all-or-nothing; `partition(Result::is_ok)` gives both lists — choose based on requirements.
4. **Type inference**: Rust infers the error type from the `map` transformation; the turbofish `::<Result<Vec<i32>, _>>` is rarely needed.

## Exercises

1. Implement a CSV parser that parses each row's fields, collecting into `Result<Vec<Row>, ParseError>` and short-circuiting on the first malformed row.
2. Compare `collect::<Result<Vec<_>>>` with `filter_map(Result::ok).collect::<Vec<_>>()` on a mixed input — show they produce different results.
3. Use `collect::<Result<Vec<_>>>()` in a function returning `Result`, then propagate the collection error with `?`.

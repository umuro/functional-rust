📖 **[View on hightechmind.io →](https://hightechmind.io/rust/320-fallible-iterator)**

---

# 320: Fallible Iterators

## Problem Statement

Iterators over external data sources — file lines, network streams, database cursors — may fail during iteration. The standard `Iterator` trait doesn't accommodate per-element errors. The solution is an iterator yielding `Result<T, E>` items, combined with the `collect::<Result<Vec<_>, _>>()` short-circuit pattern or `filter_map(Result::ok)` for best-effort collection. This is the standard pattern for parsing streams and processing external data.

## Learning Outcomes

- Implement iterators that yield `Result<T, E>` items for fallible element sources
- Use `collect::<Result<Vec<T>, E>>()` for fail-fast batch processing
- Use `filter_map(|r| r.ok())` for best-effort processing that ignores errors
- Understand the tradeoffs: fail-fast vs best-effort vs error collection

## Rust Application

Two strategies for handling per-element errors:

```rust
// Fail-fast: stop at first error
pub fn parse_all(inputs: &[&str]) -> Result<Vec<i64>, String> {
    inputs.iter().map(|s| parse_int(s)).collect()
}

// Best-effort: skip errors
pub fn parse_best_effort(inputs: &[&str]) -> Vec<i64> {
    inputs.iter().filter_map(|s| parse_int(s).ok()).collect()
}

// With default: replace errors with a fallback value
pub fn parse_with_default(inputs: &[&str], default: i64) -> Vec<i64> {
    inputs.iter().map(|s| parse_int(s).unwrap_or(default)).collect()
}
```

## OCaml Approach

OCaml handles this with `Seq.filter_map` for best-effort and `List.fold_right` for fail-fast:

```ocaml
(* Best-effort: *)
let parse_best_effort inputs =
  Seq.filter_map (fun s -> int_of_string_opt s) (List.to_seq inputs)
  |> List.of_seq

(* Fail-fast: *)
let parse_all inputs =
  List.fold_right (fun s acc ->
    let* lst = acc in
    match int_of_string_opt s with
    | None -> Error ("not a number: " ^ s)
    | Some n -> Ok (n :: lst)
  ) inputs (Ok [])
```

## Key Differences

1. **Iterator type**: Rust iterators yielding `Result<T, E>` compose naturally with all iterator adapters; OCaml requires explicit fold-based handling.
2. **Short-circuit collect**: `collect::<Result<Vec<T>, E>>()` is the idiomatic Rust one-liner for fail-fast; OCaml requires explicit fold.
3. **Fallible iterator crate**: The `fallible-iterator` crate provides a `FallibleIterator` trait with `map_err`, `and_then`, and `collect` for cleaner error handling on stream-like sources.
4. **Stream sources**: BufRead::lines() returns `Iterator<Item = io::Result<String>>` — the standard library models fallible iteration this way.

## Exercises

1. Implement a CSV line parser iterator that yields `Result<Row, ParseError>` per line, then collect all lines or fail on the first parse error.
2. Process a stream of log lines where some are malformed, using `filter_map` to skip malformed lines and count how many were skipped.
3. Implement `parse_all_errors(inputs: &[&str]) -> (Vec<i64>, Vec<String>)` that collects both successes and error messages in a single pass.

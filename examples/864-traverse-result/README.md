📖 **[View on hightechmind.io →](https://hightechmind.io/rust/864-traverse-result)**

---

# Traverse with Result
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

When processing a list of inputs that might each fail with an error, you often want to either collect all successful results or report the first error. This is traverse for Result: `Vec<T> → (T → Result<U,E>) → Result<Vec<U>,E>`. Rust's `Iterator::collect::<Result<Vec<T>,E>>()` is exactly this — it short-circuits on the first `Err` and returns it, or collects all `Ok` values. Real applications: batch parsing user inputs, processing CSV rows (fail on first bad row), executing a list of database updates (rollback on first failure), and validating a sequence of configuration values.

## Learning Outcomes

- Understand traverse for Result: short-circuits on first `Err`, collects all `Ok` on success
- Recognize `Iterator::collect::<Result<Vec<U>,E>>()` as the idiomatic Rust traverse for Result
- Implement using `try_fold` to understand the mechanics
- Distinguish traverse-Result from traverse-Option: Result carries error information; Option just signals absence
- Compare with Validated traversal: Result short-circuits; Validated accumulates all errors

## Rust Application

```rust
pub fn traverse_result<T, U, E, F: Fn(&T) -> Result<U, E>>(xs: &[T], f: F) -> Result<Vec<U>, E> {
    xs.iter().map(f).collect()
}
// Try_fold version:
pub fn traverse_result_manual<T, U, E, F: Fn(&T) -> Result<U, E>>(
    xs: &[T], f: F
) -> Result<Vec<U>, E> {
    xs.iter().try_fold(Vec::with_capacity(xs.len()), |mut acc, x| {
        acc.push(f(x)?);
        Ok(acc)
    })
}
```

`collect::<Result<Vec<U>,E>>()` short-circuits on the first `Err` — subsequent elements are never processed. This is the standard library's implementation of traverse. `try_fold` exposes the short-circuit explicitly: `f(x)?` returns `Err(e)` from `try_fold` when `f(x)` is `Err(e)`. The capacity hint `with_capacity(xs.len())` pre-allocates assuming success — a valid optimization since we process all elements on success.

## OCaml Approach

OCaml's traverse for Result: `let traverse f xs = List.fold_right (fun x acc -> match f x, acc with Ok y, Ok ys -> Ok (y :: ys) | Error e, _ -> Error e | _, Error e -> Error e) xs (Ok [])`. Alternatively using `Result.bind`: `let traverse f xs = List.fold_left (fun acc x -> acc |> Result.bind (fun ys -> f x |> Result.map (fun y -> ys @ [y]))) (Ok []) xs`. OCaml's `let* y = f x in Ok (y :: ys)` with `let%bind` reads cleanly.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Idiomatic | `collect::<Result<Vec<_>,_>>()` | `List.fold_right` |
| Short-circuit | First `Err` terminates | Same |
| Error info | Preserved in `Err(e)` | Preserved |
| vs. Validated | Short-circuits | Accumulates all errors |
| Manual | `try_fold` with `?` | `Result.bind` in fold |
| Type | `Result<Vec<U>, E>` | `('u list, 'e) result` |

## Exercises

1. Use `traverse_result` to parse a CSV line into typed fields, returning the first parse error with context.
2. Implement a batch database insert using `traverse_result`: insert all rows or return the first constraint violation.
3. Compare `traverse_result` (first error) with `traverse_validated` (all errors) on the same input and show the difference.
4. Implement `traverse_result` that accumulates ALL errors using `Validated` internally, then converts to `Result`.
5. Write property tests verifying that `traverse_result(xs, f)` is equivalent to `xs.iter().map(f).collect()`.

📖 **[View on hightechmind.io →](https://hightechmind.io/rust/865-sequence-monadic)**

---

# Sequence Monadic

## Problem Statement

`sequence` converts a collection of monadic values into a monadic collection: `Vec<Option<T>> → Option<Vec<T>>` or `Vec<Result<T,E>> → Result<Vec<T>,E>`. While `traverse` applies a function and sequences simultaneously, `sequence` is pure rearrangement — it just flips the containers. If you already have a `Vec<Option<T>>` from mapping over a collection, `sequence` collects them into `Option<Vec<T>>`. Rust's `Iterator::collect` achieves this for `Option` and `Result`. The pattern appears when: you have pre-computed individual results and need to combine them, or when working with futures (`Vec<Future<T>> → Future<Vec<T>>` via `futures::future::join_all`).

## Learning Outcomes

- Understand `sequence` as a special case of `traverse` with the identity function
- Implement `sequence_option(Vec<Option<T>>) -> Option<Vec<T>>` using `collect`
- Implement `sequence_result(Vec<Result<T,E>>) -> Result<Vec<T>,E>` using `collect`
- Recognize the connection to futures: `join_all` is `sequence` for futures
- Distinguish sequence from `flat_map`: `flat_map` chains and flattens; sequence flips the container

## Rust Application

```rust
pub fn sequence_option<T>(xs: Vec<Option<T>>) -> Option<Vec<T>> {
    xs.into_iter().collect()
}
pub fn sequence_result<T, E>(xs: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    xs.into_iter().collect()
}
// sequence_option(vec![Some(1), Some(2), Some(3)]) = Some(vec![1, 2, 3])
// sequence_option(vec![Some(1), None, Some(3)])    = None
// sequence_result(vec![Ok(1), Err("bad"), Ok(3)])  = Err("bad")
```

Both functions are one-liners in Rust because `collect()` implements sequence for `Option` and `Result`. The `into_iter()` consumes the outer Vec, and `collect::<Option<Vec<T>>>()` does the accumulation. The one failing element terminates the whole sequence. This leverages the `FromIterator<Option<T>>` implementation for `Option<Vec<T>>` in the standard library.

## OCaml Approach

OCaml's `sequence_option`: `let sequence xs = List.fold_right (fun x acc -> match x, acc with Some y, Some ys -> Some (y :: ys) | _ -> None) xs (Some [])`. The `sequence_result` is analogous with `Result`. OCaml's `Option.all` (in some versions) or manual fold implements this. For futures, `Lwt.all` sequences a list of Lwt promises. The relationship to traverse: `sequence xs = traverse Fun.id xs`.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| For Option | `collect::<Option<Vec<_>>>()` | `List.fold_right` or `Option.all` |
| For Result | `collect::<Result<Vec<_>,_>>()` | Same fold pattern |
| One-liner | Yes (built-in FromIterator) | Requires manual fold |
| Futures | `futures::join_all` | `Lwt.all` |
| vs. traverse | `traverse(xs, id)` = sequence | `traverse Fun.id xs` |
| Short-circuit | First `None`/`Err` | Same |

## Exercises

1. Implement `sequence_option` using `try_fold` (without `collect`) to understand the manual accumulation.
2. Show that `sequence(map(xs, f)) == traverse(xs, f)` with a concrete test.
3. Implement `sequence` for a `Vec<Vec<T>>` — what does flipping the containers mean here? (It's `transpose`.)
4. Use `sequence` to collect results from parallel IO operations: generate `Vec<Result<T,E>>` then sequence.
5. Implement `unsequence: Option<Vec<T>> -> Vec<Option<T>>` and verify it's the inverse when all elements are `Some`.

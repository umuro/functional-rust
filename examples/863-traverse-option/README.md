📖 **[View on hightechmind.io →](https://hightechmind.io/rust/863-traverse-option)**

---

# Traverse with Option

## Problem Statement

Given a list and a function that might fail for each element, how do you apply the function to all elements and either get all results or fail at the first problem? `map` produces `Vec<Option<U>>` — a list of maybes. `traverse` flips this to `Option<Vec<U>>` — either all succeeded or the whole thing failed. This is the "all or nothing" pattern: validate a batch of inputs, parse a list of strings, look up a list of keys — succeed if all succeed, fail on first problem. Rust achieves this via `Iterator::collect::<Option<Vec<T>>>()`, which is traverse for Option built into the language.

## Learning Outcomes

- Understand traverse: `[T] -> (T -> Option<U>) -> Option<[U]]`
- Recognize that `xs.iter().map(f).collect::<Option<Vec<U>>>()` is traverse in Rust
- Implement traverse manually using fold to understand the mechanics
- Understand the "flip container" semantic: `Vec<Option<U>>` → `Option<Vec<U>>`
- Distinguish traverse from sequence: sequence flips without applying a function

## Rust Application

```rust
pub fn traverse_option<T, U, F: Fn(&T) -> Option<U>>(xs: &[T], f: F) -> Option<Vec<U>> {
    xs.iter().map(f).collect()
    // collect::<Option<Vec<U>>>() is Rust's built-in traverse!
}
// Manual implementation using fold:
pub fn traverse_option_manual<T, U, F: Fn(&T) -> Option<U>>(xs: &[T], f: F) -> Option<Vec<U>> {
    xs.iter().try_fold(Vec::with_capacity(xs.len()), |mut acc, x| {
        acc.push(f(x)?);
        Some(acc)
    })
}
```

Rust's `collect::<Option<Vec<U>>>()` is a standard library implementation of traverse — it short-circuits on the first `None` and returns `None`, or collects all `Some(u)` values into `Vec<U>`. This is not just a convenience: it's the categorical traverse operation. `try_fold` is the explicit version: fold with early exit. The `?` inside `try_fold` propagates `None` from `f(x)?`, causing `try_fold` to return `None` immediately.

## OCaml Approach

OCaml's `traverse` for `option`: `let traverse f xs = List.fold_right (fun x acc -> match f x, acc with Some y, Some ys -> Some (y :: ys) | _ -> None) xs (Some [])`. The right fold ensures left-to-right evaluation order with a right-to-left accumulation — produces the correct order. `List.filter_map` is `traverse` that drops failures rather than short-circuiting. OCaml's `Option.bind` inside a fold naturally implements the early-exit behavior.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Idiomatic traverse | `.collect::<Option<Vec<_>>>()` | `List.fold_right` or `filter_map` |
| Short-circuit | Automatic via `collect` | Manual with `None` arm |
| Manual version | `try_fold` with `?` | `List.fold_right` |
| Order | Left-to-right (iter) | Right-fold builds correct order |
| Drop failures | `filter_map` | `List.filter_map` |
| Type signature | `Option<Vec<U>>` | `'u list option` |

## Exercises

1. Use `traverse_option` to parse a `Vec<&str>` of integers, returning `None` if any string fails to parse.
2. Implement `traverse_option` using explicit `and_then` chains and verify it matches `collect`.
3. Implement `traverse_option` that reports the first failing element along with its index.
4. Distinguish traverse from `filter_map`: show that `traverse` fails on None while `filter_map` skips None.
5. Implement a batch database lookup using traverse: given a list of IDs, return all users or None if any ID is missing.

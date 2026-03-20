📖 **[View on hightechmind.io →](https://hightechmind.io/rust/205-lens-modify)**

---

# Lens Modify

## Problem Statement

`over` (also called `modify`) is the key derived operation of a lens: apply a function to the focused field without extracting and re-inserting manually. `over(lens, f, s) = lens.set(f(lens.get(s)), s)`. This operation is so central to lens usage that it deserves its own example. Composing `over` with other lens operations (modify a sub-field, accumulate changes, apply validation) shows the lens as a reusable update combinator.

## Learning Outcomes

- Implement and use `over` (modify) as the primary lens update operation
- See how `over` composes with other transformations: `modify_each`, `modify_if`
- Understand `set` as a special case of `over`: `set(lens, a, s) = over(lens, |_| a, s)`
- Practice chaining multiple `over` calls to apply independent updates to a structure

## Rust Application

`over(lens, f, s)` is defined as `lens.set(f(lens.get(s)), s)`. The code demonstrates: incrementing a numeric field (`over(&age_lens, |a| a + 1, person)`), transforming a string field (`over(&name_lens, |n| n.to_uppercase(), person)`), and modifying a nested field via composed lenses (`over(&full_lens, |p| p + 10, config)` where `full_lens` focuses on a deeply nested port number). Each `over` call returns a new structure without mutating the original.

## OCaml Approach

OCaml's `over` (often called `modify` or `update`) is identical:
```ocaml
let over l f s = l.set (f (l.get s)) s
(* Usage: *)
let new_config = over port_lens (fun p -> p + 1) config
```
Haskell's `lens` library uses `%~` as the infix operator for `over`: `config & port_lens %~ (+1)`. OCaml's `(|>)` provides a similar pipeline: `config |> over port_lens ((+) 1)`.

## Key Differences

1. **Operator syntax**: Haskell's `%~` makes lens modification declarative; OCaml and Rust use function calls — less concise but clearer for learners.
2. **Multiple modifications**: Applying multiple `over` calls is equivalent to composing functions and applying once; lazy lenses can fuse these.
3. **Shared structure**: Each `over` call clones unchanged fields via `..record` destructuring; shared persistent data structures avoid this copying.
4. **Batch updates**: `modify_all(lens, f, list)` maps `over(lens, f)` over a list — a common pattern for bulk updates.

## Exercises

1. Implement `set_via_over(lens, a, s) -> S` using only `over` (no direct access to `lens.set`).
2. Write `modify_if(lens, pred, f, s)` that applies `f` only if the focused value satisfies `pred`.
3. Chain three `over` calls on the same config, each modifying a different field, and verify the result is correct.

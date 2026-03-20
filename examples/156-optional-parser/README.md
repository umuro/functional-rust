📖 **[View on hightechmind.io →](https://hightechmind.io/rust/156-optional-parser)**

---

# Optional Parser

## Problem Statement

Many grammar elements are optional: a sign before a number (`-42` vs `42`), a trailing comma, a default parameter value. The `opt` combinator wraps a parser in `Option`: if the parser succeeds, `opt` returns `Some(value)`; if it fails, `opt` returns `None` (consuming no input). This makes optional grammar elements explicit and composable, avoiding nested `if/else` in hand-written parsers.

## Learning Outcomes

- Understand `opt` as the standard way to handle optional grammar elements
- Learn how `opt` relates to `many0`: `opt` is `many0` limited to at most one result
- See how `opt` enables parsing optional signs, suffixes, and modifiers
- Practice combining `opt` with `map` to provide default values

## Rust Application

`opt<T>(parser: Parser<T>) -> Parser<Option<T>>` runs the inner parser. If it succeeds, the result is wrapped in `Some`. If it fails, the original input is restored (no consumption) and `None` is returned wrapped in `Ok`. The key invariant: on failure, `opt` does not consume any input — backtracking is essential. Combining with `map`: `opt(sign_parser).map(|s| s.unwrap_or('+'))` provides a default for absent values.

## OCaml Approach

OCaml's angstrom provides `option : 'a -> 'a t -> 'a t` (provides a default value) and `optional : 'a t -> 'a option t` (equivalent to Rust's `opt`). The `<?>` operator adds a human-readable label. Backtracking in angstrom requires the `?>>` or `commit` combinators — angstrom does not backtrack by default, requiring explicit backtracking markers for non-trivial alternatives.

## Key Differences

1. **Backtracking default**: Rust's simple `opt` implementation always backtracks on failure (tries, fails, restores input); angstrom requires explicit `option` combinators to allow backtracking.
2. **Default values**: Rust uses `opt(...).map(|o| o.unwrap_or(default))`; angstrom's `option default parser` provides the default directly.
3. **Error recovery**: Both return `None`/`option default` on failure; neither preserves the inner error message (failure is expected and silent).
4. **Combinability**: Both `opt` variants combine equally with `map`, `flat_map`, and `sequence` combinators.

## Exercises

1. Build a signed integer parser: `opt(char_parser('-')).map(...)` combined with a digit sequence.
2. Implement `with_default<T: Clone>(default: T, p: Parser<T>) -> Parser<T>` using `opt` and `map`.
3. Write a parser for optional trailing commas in a list: `"[1, 2, 3,]"` and `"[1, 2, 3]"` both succeed.

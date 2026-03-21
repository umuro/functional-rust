📖 **[View on hightechmind.io →](https://hightechmind.io/rust/041-option-basics)**

---

# 041 — Option Basics
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

`Option<T>` is Rust's solution to the null pointer problem. Where Java uses `null` and C uses `NULL` or sentinel values, Rust forces you to acknowledge that a value may not exist at the type level. `Option<T>` is an enum with two variants: `Some(T)` when a value exists, and `None` when it does not.

This eliminates the entire class of null-pointer exceptions at compile time. The type system enforces that you handle both cases before using the value. `Option` originates from Haskell's `Maybe` type and OCaml's `option` type — both over 30 years old.

## Learning Outcomes

- Construct and destructure `Option<T>` with `Some(x)` and `None`
- Use `match` to handle both cases exhaustively
- Use `.is_some()`, `.is_none()`, `.unwrap_or(default)`, and `.unwrap_or_else(f)` for common patterns
- Understand why `unwrap()` panics and when it is acceptable
- Convert `Option` to a value with a default using `unwrap_or`

## Rust Application

Creating options: `Some(42)` wraps a value; `None` represents absence. `match opt { Some(x) => ..., None => ... }` is the exhaustive handler. `opt.unwrap_or(0)` returns the value or a default. `opt.unwrap_or_else(|| expensive_computation())` defers the default computation. `if let Some(x) = opt { ... }` is convenient when only the `Some` case matters. These are the building blocks for all subsequent Option-chaining examples.

## OCaml Approach

OCaml's `option` type: `type 'a option = None | Some of 'a`. Usage: `match opt with | None -> ... | Some x -> ...`. `Option.value opt ~default:0` returns value or default (requires OCaml 4.08+). `Option.get opt` is like `unwrap()` — raises `Invalid_argument` if `None`. The `|>` pipe with `Option.map`, `Option.bind` makes chains readable.

## Key Differences

1. **`unwrap` vs `Option.get`**: Both panic/raise on `None`. Prefer `unwrap_or`, `unwrap_or_else`, or `?` in production code.
2. **`if let` syntax**: Rust's `if let Some(x) = opt` is syntactic sugar for a match with one arm. OCaml uses `match` or `Option.iter` for single-arm handling.
3. **`?` operator**: Rust has the `?` operator for propagating `None` in functions returning `Option`. OCaml requires explicit match or monadic style with `let*`.
4. **No implicit null**: Rust has no null reference for any type except `Option<&T>`. OCaml also has no null pointer at the language level — only the `option` type.

## Exercises

1. **Safe minimum**: Write `safe_min(v: &[i32]) -> Option<i32>` that returns `None` for empty slices and `Some(min)` otherwise. Use `v.iter().min().copied()`.
2. **Default chain**: Write `first_some(opts: &[Option<i32>]) -> Option<i32>` that returns the first `Some` value in the slice. Use `opts.iter().copied().find(Option::is_some).flatten()`.
3. **Option math**: Write `add_options(a: Option<i32>, b: Option<i32>) -> Option<i32>` that returns `Some(a + b)` only if both are `Some`. Use `match (a, b)`.

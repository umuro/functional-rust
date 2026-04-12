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

- Distinguish `unwrap()` (panics on None) from `unwrap_or(default)` and `expect("message")` (panics with context)
- Use `if let Some(x) = opt` as shorthand when only the `Some` case matters

## Rust Application

Creating options: `Some(42)` wraps a value; `None` represents absence. `match opt { Some(x) => ..., None => ... }` is the exhaustive handler. `opt.unwrap_or(0)` returns the value or a default. `opt.unwrap_or_else(|| expensive_computation())` defers the default computation. `if let Some(x) = opt { ... }` is convenient when only the `Some` case matters. These are the building blocks for all subsequent Option-chaining examples.

## OCaml Approach

OCaml's `option` type: `type 'a option = None | Some of 'a`. Usage: `match opt with | None -> ... | Some x -> ...`. `Option.value opt ~default:0` returns value or default (requires OCaml 4.08+). `Option.get opt` is like `unwrap()` — raises `Invalid_argument` if `None`. The `|>` pipe with `Option.map`, `Option.bind` makes chains readable.

## Key Differences

1. **`unwrap` vs `Option.get`**: Both panic/raise on `None`. Prefer `unwrap_or`, `unwrap_or_else`, or `?` in production code.
2. **`if let` syntax**: Rust's `if let Some(x) = opt` is syntactic sugar for a match with one arm. OCaml uses `match` or `Option.iter` for single-arm handling.
3. **`?` operator**: Rust has the `?` operator for propagating `None` in functions returning `Option`. OCaml requires explicit match or monadic style with `let*`.
4. **No implicit null**: Rust has no null reference for any type except `Option<&T>`. OCaml also has no null pointer at the language level — only the `option` type.

1. **`null` replacement:** `Option<T>` encodes "may be absent" in the type system. Java's `null` is an implicit extra value for every reference type — the compiler cannot distinguish "this might be null" from "this is never null".
2. **`unwrap()` vs safe alternatives:** `unwrap()` panics on `None` — only use it in tests or when `None` is provably impossible (document why). In production code, prefer `unwrap_or`, `unwrap_or_else`, `expect`, or `match`.
3. **`Option.get` in OCaml:** OCaml 4.08+ provides `Option.get opt` which raises `Invalid_argument "option is None"`. Equivalent to Rust's `unwrap()`. Avoid in production for the same reasons.
4. **`if let` pattern:** `if let Some(x) = opt { use x }` is Rust's convenient syntax for "do something only if there's a value". OCaml uses `match opt with Some x -> ... | None -> ()`.

## Exercises

1. **Safe minimum**: Write `safe_min(v: &[i32]) -> Option<i32>` that returns `None` for empty slices and `Some(min)` otherwise. Use `v.iter().min().copied()`.
2. **Default chain**: Write `first_some(opts: &[Option<i32>]) -> Option<i32>` that returns the first `Some` value in the slice. Use `opts.iter().copied().find(Option::is_some).flatten()`.
3. **Option math**: Write `add_options(a: Option<i32>, b: Option<i32>) -> Option<i32>` that returns `Some(a + b)` only if both are `Some`. Use `match (a, b)`.

4. **Option chaining**: Write `first_even_doubled(v: &[i32]) -> Option<i32>` that finds the first even number and doubles it, returning `None` if no even number exists. Use only `find`, `map`, no explicit `match`.
5. **Nested option**: Implement `flatten_option(opt: Option<Option<T>>) -> Option<T>` — equivalent to OCaml's `Option.join`. Then implement it as a one-liner using `.and_then(|x| x)`.

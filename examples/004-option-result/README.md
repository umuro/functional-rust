📖 **[View on hightechmind.io →](https://hightechmind.io/rust/004-option-result)**

---

# 004 — Option and Result
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Null references were famously called "the billion-dollar mistake" by their inventor Tony Hoare. Languages like Java, C, and C++ use `null`/`NULL` to represent missing values, which causes `NullPointerException`, segfaults, and unchecked error codes at runtime. Functional languages solved this with algebraic types: `Option` (sometimes called `Maybe`) wraps a value that may or may not exist, and `Result` (or `Either`) wraps a value that may have failed with an error.

These types make the possibility of absence or failure explicit in the type signature, forcing callers to handle both cases. They also compose via `map` and `and_then` (monadic bind), enabling clean pipelines of fallible operations without nested `if-let` chains.

## Learning Outcomes

- Use `Option<T>` for values that may not exist, avoiding null
- Use `Result<T, E>` for operations that may fail with a typed error
- Chain operations with `.map()` and `.and_then()` to avoid nested matching
- Understand the monadic structure: `and_then` is "do this next, but only if the previous step succeeded"
- Convert between `Option` and `Result` with `.ok_or()` and `.ok()`

## Rust Application

`safe_div` returns `Option<i32>` instead of relying on a caller to check for zero. `double_head` chains with `.map(|x| x * 2)` — if the head is `None`, map short-circuits. `chain_lookups` uses `and_then` to sequence two fallible operations: only if the first succeeds does it attempt the second. The `MyError` enum and `Result`-based variants show how to carry structured error information — the same pattern used throughout the Rust standard library and in crates like `anyhow` and `thiserror`.

## OCaml Approach

OCaml's `option` type (`None | Some x`) and `result` type (`Ok x | Error e`) work identically. `Option.map` and `Option.bind` correspond to Rust's `.map()` and `.and_then()`. The `|>` pipe makes chaining natural: `safe_head lst |> Option.bind (fun idx -> ...)`. OCaml also has `let*` (monadic let) for sequential binding without nesting: `let* x = safe_div a b in let* y = safe_sqrt x in Ok (x + y)`.

## Key Differences

1. **`?` operator**: Rust's `?` in a function returning `Result` is syntactic sugar for `and_then`/early return. OCaml uses `let*` (ppx_let) or explicit `match` — `?` does not exist in standard OCaml.
2. **Error types**: Rust's `Result<T, E>` is generic over the error type. OCaml's `result` type is also `('a, 'b) result` but idiomatic OCaml often uses polymorphic variants for errors.
3. **`ok_or`**: Rust provides `.ok_or(err)` to convert `Option<T>` to `Result<T, E>`. OCaml uses `Option.to_result ~none:err`.
4. **Unwrap**: Both languages provide an "unwrap or panic" escape hatch (Rust: `.unwrap()`, OCaml: `Option.get`). Both should be avoided in production code.

## Exercises

1. **Safe index**: Write `safe_get(v: &[i32], i: usize) -> Option<i32>` and chain it with `safe_div` to implement `divide_at_index(nums: &[i32], i: usize, divisor: i32) -> Option<i32>`.
2. **Collect options**: Write `all_or_none(opts: &[Option<i32>]) -> Option<Vec<i32>>` that returns `Some` only if all inputs are `Some`, using `.collect::<Option<Vec<_>>>()`.
3. **Error enrichment**: Write a function that parses a string to an integer, divides it by another parsed integer, and returns a `Result<i32, String>` with a descriptive error message at each step.

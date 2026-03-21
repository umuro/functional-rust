📖 **[View on hightechmind.io →](https://hightechmind.io/rust/047-result-bind)**

---

# 047 — Result Bind (and_then)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

`Result::and_then` (bind) sequences two fallible operations: apply a function that returns `Result` to the `Ok` value, propagating `Err` automatically. It is the monadic sequencing operation for `Result` — "do this, then do that, stopping at the first failure". This enables clean pipelines of fallible operations without nested match statements.

The classic example is a multi-step pipeline: validate input → parse → compute → format. Each step may fail; if any fails, the pipeline stops and propagates the error. Without `and_then`, this requires nested `match` blocks or repeated null-checking. With `and_then`, the happy path reads linearly.

## Learning Outcomes

- Use `result.and_then(|v| fallible_transform(v))` to chain fallible operations
- Understand the difference from `map`: `and_then` applies a function returning `Result`
- Build multi-step pipelines where each step can independently fail
- Recognize `and_then` as the result monad's bind operation
- Connect `and_then` to the `?` operator: `f()?` desugars to an `and_then` chain

## Rust Application

`parse_int(s).and_then(|n| safe_div(n, 2)).and_then(|n| Ok(n.to_string()))` — parse, divide (may fail with division by zero), convert. Each `and_then` only runs if the previous step succeeded. The `?` operator is syntax sugar: `let n = parse_int(s)?; let d = safe_div(n, 2)?; Ok(d.to_string())` is equivalent and more readable for long chains. Both error types must be compatible (same `E`, or convertible via `From`).

## OCaml Approach

OCaml's `Result.bind r f`: `let bind r f = match r with Ok x -> f x | Error e -> Error e`. Pipe style: `parse_int s |> Result.bind safe_div |> Result.bind (fun n -> Ok (string_of_int n))`. With `let*` (ppx_let): `let* n = parse_int s in let* d = safe_div n 2 in Ok (string_of_int d)` — this reads like imperative code with automatic error propagation.

## Key Differences

1. **`?` vs `let*`**: Rust's `?` is built into the language. OCaml's `let*` requires the `ppx_let` preprocessor. The `?` operator is more ergonomic for long chains.
2. **Error type unification**: Rust's `?` operator can automatically convert error types via the `From` trait — `?` on `Result<T, IoError>` in a function returning `Result<T, AppError>` calls `AppError::from(e)`. OCaml requires explicit `Result.map_error`.
3. **`and_then` vs `>>=`**: Haskell uses `>>=` (pronounced "bind"). Rust calls it `and_then`. OCaml calls it `bind`. All are the same operation.
4. **Error channel**: `and_then` only threads through the success channel. Use `or_else` for the error channel: `result.or_else(|e| fallback(e))`.

## Exercises

1. **Config parser**: Write a function that reads a string `"host:port"`, splits on `:`, parses the port as `u16`, and validates the host is non-empty. Use `and_then` at each step.
2. **Database query chain**: Simulate `find_user(id).and_then(|u| find_posts(u.id)).and_then(|posts| render(posts))` with stub functions. Handle the case where each step might return `Err("not found")`.
3. **Equivalent forms**: Rewrite a 3-step `and_then` chain using (a) nested `match`, (b) `and_then`, (c) `?` operator. Verify all three produce the same results. Discuss readability.

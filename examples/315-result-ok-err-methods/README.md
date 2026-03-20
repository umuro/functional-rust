📖 **[View on hightechmind.io →](https://hightechmind.io/rust/315-result-ok-err-methods)**

---

# 315: Result ok() and err() Methods

## Problem Statement

The full `Result<T, E>` method surface is large — over 20 methods covering query, transformation, combination, and extraction. Many Rust developers use only `?`, `unwrap()`, and `map()`, missing more specialized tools like `ok()` (discard error, get `Option<T>`), `err()` (discard success, get `Option<E>`), `map_or()`, and `and()`/`or()`. This reference covers the complete method set and their appropriate use cases.

## Learning Outcomes

- Use `ok()` to convert `Result<T, E>` to `Option<T>` discarding the error
- Use `err()` to convert `Result<T, E>` to `Option<E>` discarding the success
- Understand `map_or(default, f)` and `map_or_else(err_f, ok_f)` for default values
- Use `and(other)` and `or(other)` for boolean-like result chaining

## Rust Application

Selected methods from the `Result` API:

```rust
let ok: Result<i32, &str> = Ok(5);
let err: Result<i32, &str> = Err("bad");

// Query
ok.is_ok();   // true
err.is_err(); // true

// Convert to Option
ok.ok();   // Some(5) — error discarded
err.err(); // Some("bad") — success discarded
ok.err();  // None

// Transform
ok.map(|x| x * 2);                   // Ok(10)
err.map_err(|e| format!("err: {}", e)); // Err("err: bad")
ok.map_or(0, |x| x + 1);             // 6

// Combine
ok.and(Ok::<i32, &str>(10));          // Ok(10) — ok.and(x) = x if ok is Ok
err.or(Ok::<i32, &str>(42));          // Ok(42) — err.or(x) = x if self is Err
```

## OCaml Approach

OCaml's `Result` module provides `Result.is_ok`, `Result.is_error`, `Result.map`, `Result.map_error`, and `Result.fold` — a subset of Rust's methods. `Result.ok` (convert to `Option`) exists as `Result.to_option`:

```ocaml
Result.is_ok (Ok 5)                   (* true *)
Result.to_option (Ok 5)               (* Some 5 *)
Result.fold ~ok:(fun x -> x+1) ~error:(fun _ -> 0) (Ok 5)  (* 6 *)
```

## Key Differences

1. **Method surface**: Rust has ~20+ methods on `Result`; OCaml's standard `Result` module has fewer, with `Base.Result` providing more.
2. **`ok()` as filter**: `result.ok()` is the idiomatic way to silently drop errors when only the success case matters — used extensively in `filter_map`.
3. **`and`/`or` logic**: `and(other)` propagates `Err` from self; `or(other)` propagates `Ok` from self — they model error "and"/"or" logic.
4. **Exhaustive coverage**: Knowing all methods prevents reinventing them with `match` — a `map_or` is cleaner than a `match` with two branches.

## Exercises

1. Use `ok()` and `filter_map()` together to collect only successfully parsed values from a `Vec<Result<i32, _>>`.
2. Use `and()` to chain two `Result` values: only proceed if both are `Ok`, return the first `Err` otherwise.
3. Use `map_or_else(|e| log_and_default(e), |v| v)` to log errors and provide fallback values in a single expression.

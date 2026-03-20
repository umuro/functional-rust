📖 **[View on hightechmind.io →](https://hightechmind.io/rust/855-monad-result)**

---

# Result Monad
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Error handling with `match` on every `Result` is verbose and obscures the happy path. The Result monad — Rust's `Result::and_then` and the `?` operator — chains fallible operations so that the first error short-circuits the chain and is returned immediately. This is the foundation of Rust's ergonomic error handling: `parse()?.compute()?.serialize()?` reads like a straight pipeline yet properly propagates errors. The same pattern is OCaml's `Result.bind`, Haskell's `Either` monad, and Scala's `for`-comprehensions over `Either`. Understanding it as a monad explains why `map` and `and_then` behave differently and how to write clean, composable error-handling code.

## Learning Outcomes

- Understand `and_then` for Result: if `Ok(x)`, apply `f(x)` returning `Result<U, E>`; if `Err(e)`, return `Err(e)`
- Recognize `?` as syntactic sugar: `expr?` = `match expr { Ok(x) => x, Err(e) => return Err(e.into()) }`
- Chain `and_then` calls to build pipelines that propagate errors without nested matches
- Distinguish `map` (transform Ok value, keep same error type) from `and_then` (can return new Err)
- Apply to: multi-step parsing pipelines, file I/O chains, network request sequences

## Rust Application

```rust
fn parse_int(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|_| format!("Not an integer: {}", s))
}
fn double_if_positive(n: i32) -> Result<i32, String> {
    if n > 0 { Ok(n * 2) } else { Err(format!("Not positive: {}", n)) }
}
fn pipeline(s: &str) -> Result<i32, String> {
    parse_int(s).and_then(double_if_positive)
}
```

Each function returns `Result<T, String>` — a concrete error type. `and_then` chains them: if `parse_int` succeeds, pass the value to `double_if_positive`; if `parse_int` fails, skip `double_if_positive` and return the parse error. The `?` operator version: `fn pipeline(s: &str) -> Result<i32, String> { let n = parse_int(s)?; double_if_positive(n) }` — identical semantics, different syntax. `map_err` converts the parse library error into the function's error type.

## OCaml Approach

OCaml's `Result.bind` has the signature `('a, 'e) result -> ('a -> ('b, 'e) result) -> ('b, 'e) result`. The infix `let ( >>= ) = Result.bind` enables pipelines. OCaml's `let open Result in parse_int s >>= double_if_positive` reads naturally. The `let* x = parse_int s in double_if_positive x` syntax (with ppx_let or OCaml 4.08+) provides do-notation. `Result.map_error` converts error types, mirroring Rust's `map_err`.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Bind | `Result::and_then` | `Result.bind` |
| Syntax sugar | `?` operator | `let*` (ppx_let or 4.08+) |
| Error conversion | `map_err` / `From` trait | `Result.map_error` |
| Error propagation | `?` calls `.into()` for type conversion | Manual `Result.map_error` |
| Ok path | Right-bias | Right-bias |
| Multiple error types | `Box<dyn Error>` or `anyhow` | `string` or polymorphic variant |

## Exercises

1. Implement a multi-step file processing pipeline: read file → parse lines → validate each line → transform → write output, using `?` for error propagation.
2. Implement `Result::and_then` from scratch using `match` and verify it matches the stdlib behavior.
3. Use `map_err` to convert between different error types in a pipeline involving multiple library functions.
4. Implement a pipeline using `and_then` chains (not `?`) and compare readability with the `?` version.
5. Demonstrate the short-circuit behavior: add logging to each step and show that steps after the first failure don't execute.

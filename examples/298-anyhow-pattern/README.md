📖 **[View on hightechmind.io →](https://hightechmind.io/rust/298-anyhow-pattern)**

---

# 298: The anyhow Pattern — Boxed Errors
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Application code (as opposed to library code) often doesn't need to classify errors precisely — it just needs to propagate them to a top-level handler that logs or displays them. Defining a custom error enum for every function that calls multiple libraries is over-engineering. The `anyhow` pattern uses `Box<dyn Error + Send + Sync>` as a universal error container — any error can be boxed and propagated without defining wrapper types.

## Learning Outcomes

- Understand `Box<dyn Error + Send + Sync>` as a type-erased error container
- Implement a `Context` wrapper that adds descriptive messages to errors
- Recognize when to use `anyhow` (applications) vs `thiserror` (libraries)
- Traverse the error chain via `source()` to display full context

## Rust Application

The `AnyResult<T>` type alias is the foundation of the `anyhow` pattern:

```rust
pub type AnyResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

// Any error can be boxed and propagated:
fn process(s: &str) -> AnyResult<i32> {
    let n: i32 = s.parse()?;  // ParseIntError boxes automatically
    Ok(n * 2)
}

// WithContext wraps any error with a descriptive message:
pub fn with_context<E: Error + Send + Sync + 'static>(
    result: Result<(), E>,
    msg: impl Into<String>
) -> AnyResult<()> {
    result.map_err(|e| Box::new(WithContext { context: msg.into(), source: Box::new(e) }) as _)
}
```

## OCaml Approach

OCaml uses `result` with string errors for simple cases, or `Printexc` for exceptions. The `Error_monad` from Tezos and `Lwt`'s error handling provide richer composable errors, but there is no standard `Box<dyn Error>` equivalent:

```ocaml
(* Simple approach: use string as universal error *)
type 'a result_with_context = ('a, string) result

let with_context ctx = function
  | Error e -> Error (ctx ^ ": " ^ e)
  | Ok v -> Ok v
```

## Key Differences

1. **Type erasure**: `Box<dyn Error>` erases the concrete error type at runtime; the dynamic dispatch allows uniform handling of any error.
2. **Context chaining**: `anyhow` (and this pattern) preserves the original error as `source()` — the context wraps but doesn't replace.
3. **Application vs library**: `anyhow`/boxed errors are appropriate for applications; libraries should use precise error types via `thiserror`.
4. **Thread safety**: `Send + Sync` bounds enable using errors across async tasks and threads — essential for concurrent applications.

## Exercises

1. Implement a `context(msg)` extension method on `Result<T, E>` that wraps any error in a `WithContext` struct.
2. Write a function that calls five different operations each returning different error types, using `AnyResult<T>` to unify them without any `map_err`.
3. Traverse the full error chain of a nested `WithContext` error and display each level with its message.

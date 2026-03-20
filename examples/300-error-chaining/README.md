📖 **[View on hightechmind.io →](https://hightechmind.io/rust/300-error-chaining)**

---

# 300: Chaining Errors with source()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Errors in real systems are causal chains: a configuration loading failure is caused by a file read failure, which is caused by a permissions denial. Displaying only the top-level error loses the root cause. The `Error::source()` method creates a linked list of errors from high-level to low-level, enabling tools and users to see the complete causal chain. This is the Rust equivalent of Java's exception chaining (`getCause()`) and Python's `raise X from Y`.

## Learning Outcomes

- Implement `Error::source()` to expose a wrapped inner error as the cause
- Traverse an error chain using the `source()` method iteratively
- Build a `print_error_chain` function that displays the full causal hierarchy
- Understand the ownership model: `source()` returns `&(dyn Error + 'static)`

## Rust Application

Each error in the chain implements `source()` pointing to the next lower-level error:

```rust
pub struct AppError {
    pub message: String,
    pub source: Option<Box<dyn Error + Send + Sync>>,
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

// Traverse the chain:
fn print_error_chain(e: &dyn Error) {
    println!("Error: {}", e);
    let mut source = e.source();
    while let Some(cause) = source {
        println!("Caused by: {}", cause);
        source = cause.source();
    }
}
```

## OCaml Approach

OCaml has no standard error chaining. Exceptions have a `Printexc.raise_with_backtrace` for preserving stack traces, but error values in `Result` require explicit nesting:

```ocaml
type 'a with_cause = { error: 'a; cause: exn option }
(* Custom traversal required; no standard chain protocol *)
```

## Key Differences

1. **Standard protocol**: `source()` is a standard trait method — any library that implements it participates in the chain automatically.
2. **Traversal**: Rust provides no built-in chain display; users write `while let Some(s) = e.source()` loops.
3. **Ownership**: `source()` returns a borrowed reference `&(dyn Error + 'static)` — the chain is borrowed, not owned, preventing double-free issues.
4. **Future**: The `Backtrace` type (stabilized in Rust 1.73) captures stack traces at error creation, complementing the causal chain.

## Exercises

1. Build a three-level error chain (`AppError -> ConfigError -> IoError`) and implement `source()` at each level to expose the next.
2. Write a `collect_error_chain(e: &dyn Error) -> Vec<String>` function that collects all error messages in the chain as a vector.
3. Implement an `error_root_cause(e: &dyn Error) -> &dyn Error` function that traverses `source()` links until it reaches the last error with no source.

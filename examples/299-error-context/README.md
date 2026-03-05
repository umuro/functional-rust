📖 **[View on hightechmind.io →](https://hightechmind.io/rust/299-error-context)**

---

# 299: Adding Context to Errors

**Difficulty:** 3  **Level:** Advanced

Attach "where and why" information to errors without losing the original cause.

## The Problem This Solves

A production error log shows: `"not found"`. Which file? Which operation? There's no way to know without adding context at the point of failure — but if you just convert the error to a string, you lose the original structured error that callers might want to inspect programmatically.

Context wrapping solves this. Instead of swallowing the original error, you wrap it: the outer error carries a human-readable message explaining *what you were trying to do*, and the inner error (accessible via `source()`) preserves the original cause. Callers get both: a readable message and a traversable causal chain.

This is what `anyhow::Context` does, and what production code needs. A bare `file not found` is useless. `loading config from '/etc/app/config.toml': file not found` tells an engineer exactly what to fix.

## The Intuition

Context wrapping is a linked-list prepend: add a new "what I was doing" node to the front of the error chain, while preserving the original error as `source()`.

## How It Works in Rust

```rust
// A generic context wrapper — holds a message and the original error
#[derive(Debug)]
struct Context<E> {
    message: String,
    source: E,
}

impl<E: fmt::Display> fmt::Display for Context<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)  // show the context message, not the inner error
    }
}

impl<E: Error + 'static> Error for Context<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)  // the original error is still accessible
    }
}

// Extension trait: adds .context("...") to any Result
trait WithContext<T, E> {
    fn context(self, msg: &str) -> Result<T, Context<E>>;
}

impl<T, E: Error> WithContext<T, E> for Result<T, E> {
    fn context(self, msg: &str) -> Result<T, Context<E>> {
        self.map_err(|e| Context { message: msg.to_string(), source: e })
    }
}

// Usage: context at every layer adds "what you were doing"
fn load_config(path: &str) -> Result<String, Context<IoError>> {
    read_file(path).context(&format!("loading config from '{}'", path))
}
```

Walk the chain: `println!("{}", e)` prints the context message. `e.source()` gives the original `IoError`. A logging framework can walk `source()` repeatedly to print the full causal history.

## What This Unlocks

- **Debuggable production errors** — every error tells you what was being attempted, not just what went wrong
- **Preserved structure** — the original error is still accessible via `source()` for programmatic inspection or further wrapping
- **anyhow compatibility** — this is exactly the pattern `anyhow::Context::context()` uses; understanding it means you understand `anyhow`

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Context wrapping | Manual tuple type or string annotation | Wrapper struct implementing `Error::source()` |
| Chain traversal | Manual recursion over cause field | Standard `source()` linked list |
| Ergonomics | Verbose at every call site | `.context()` extension method on `Result` |
| Message vs cause | Combined or separate | Separate: `Display` = message, `source()` = cause |

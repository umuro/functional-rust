📖 **[View on hightechmind.io →](https://hightechmind.io/rust/300-error-chaining)**

---

# 300: Chaining Errors with source()

**Difficulty:** 3  **Level:** Advanced

Build a traversable causal chain — reconstruct the full story of what went wrong.

## The Problem This Solves

An application fails at startup. The error message says "application startup failed." That tells you nothing useful. The actual cause — a missing config file — is buried three layers down in an error that was wrapped and re-wrapped as it propagated up through the call stack.

Without error chaining, each wrapper discards the inner error to produce a new message. You get the high-level "what" but lose the low-level "why." In production, you need both: the context at each layer to understand the sequence of events, and the root cause to know what actually needs to be fixed.

`Error::source()` creates a singly-linked list. Each error points to the error that caused it. A log formatter can walk this chain — `StartupError → ConfigError → FileError` — and print every layer. `anyhow`'s pretty-printer does exactly this. So does a well-written error reporter in any production codebase.

## The Intuition

`source()` links each error to its cause like nodes in a linked list — walk it from front to back to reconstruct "startup failed because config failed because file not found."

## How It Works in Rust

```rust
// Layer 1: root cause — no source
#[derive(Debug)]
struct FileError { path: String }
impl Error for FileError {}  // source() returns None by default

// Layer 2: wraps FileError
#[derive(Debug)]
struct ConfigError { source: FileError }
impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)  // points to FileError
    }
}

// Layer 3: wraps ConfigError
#[derive(Debug)]
struct StartupError { source: ConfigError }
impl Error for StartupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)  // points to ConfigError
    }
}

// Traverse the chain — standard pattern for logging/debugging
fn print_chain(e: &dyn Error) {
    println!("Error: {}", e);
    let mut cause = e.source();
    while let Some(c) = cause {
        println!("  Caused by: {}", c);
        cause = c.source();
    }
}

// Collect chain into Vec<String> for structured logging
fn error_chain(e: &dyn Error) -> Vec<String> {
    let mut chain = vec![e.to_string()];
    let mut cause = e.source();
    while let Some(c) = cause { chain.push(c.to_string()); cause = c.source(); }
    chain
}
```

The root cause is always `chain.last()`. The depth of the chain tells you how many layers of abstraction the error crossed.

## What This Unlocks

- **Full diagnostic context** — every `?` that wraps an error can preserve the causal chain; no information is lost
- **Root cause analysis** — `chain.last()` gives the lowest-level error; no more guessing which file or which operation
- **Log-friendly** — structured error chains map naturally to log fields: `error`, `caused_by`, `root_cause`

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Exception chaining | Manual `cause` field, no standard | `Error::source()` — stdlib standard |
| Chain traversal | Manual recursion | `while let Some(c) = e.source()` loop |
| Root cause | `Option.value (exn.cause)` recursively | `chain.last()` after collecting |
| Pretty printing | Manual format | Traverse `source()` chain in a loop |

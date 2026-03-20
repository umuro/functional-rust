📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1026-error-display)**

---

# 1026-error-display — Custom Error Display and Source Chain

## Problem Statement

Structured error hierarchies lose their value if they cannot be rendered as human-readable messages. The `std::error::Error` trait provides a `source()` method for linking errors in a chain — root cause, intermediate wrapper, outer context — and `Display` for rendering each layer. Walking this chain produces messages like `"startup failed: config error: file not found: /etc/app.conf"`.

This chain-walking pattern is what `anyhow` and `eyre` automate. Understanding it from scratch explains what those crates provide and when to build your own.

## Learning Outcomes

- Implement `Display` for nested error types
- Implement `Error::source()` to link errors in a causal chain
- Write a chain-walking function that formats the full error hierarchy
- Understand how `anyhow::Error` formats its chain by default
- Appreciate the difference between `to_string()` and the full source chain

## Rust Application

`src/lib.rs` builds a three-level error hierarchy: `IoError` (root), `ConfigError` (wraps `IoError`), and `AppError` (wraps `ConfigError`). Each implements `Error::source()` returning the wrapped error. A `format_error_chain` function walks the chain via repeated `source()` calls and produces a numbered list. The tests demonstrate that `to_string()` shows only the outermost message while the chain walker shows all layers.

The `Error::source()` chain is what `tracing` uses to attach all causal errors to a log event.

## OCaml Approach

OCaml's `Base.Error` is a lazy tree that records the full context automatically:

```ocaml
let config_error = Error.of_string "file not found: /etc/app.conf"
let app_error = Error.tag config_error ~tag:"config error"
let full = Error.tag app_error ~tag:"startup failed"
```

`Error.to_string_hum full` renders `"startup failed: config error: file not found: /etc/app.conf"`. The laziness means the string is only built when rendered.

## Key Differences

1. **Explicit vs automatic**: Rust requires manually implementing `Error::source()` for each wrapper; OCaml's `Error.tag` builds the chain automatically.
2. **Lazy rendering**: OCaml's `Error.t` is a lazy tree; Rust's `Display` is computed eagerly when called.
3. **Chain direction**: Rust's `source()` chain goes from outer to inner (you call `source()` repeatedly); OCaml's `Error` tree goes from inner to outer as tags are added.
4. **`anyhow` equivalence**: `anyhow::Error` wraps Rust errors and provides automatic source-chain display; it is conceptually similar to OCaml's `Base.Error`.

## Exercises

1. Add a fourth error level `DeploymentError` that wraps `AppError` and format the resulting four-level chain.
2. Write `error_chain_to_vec(err: &dyn Error) -> Vec<String>` that collects all messages from root to leaf.
3. Implement a `Display` that renders the chain inline as `"outer (caused by: middle (caused by: root))"`.

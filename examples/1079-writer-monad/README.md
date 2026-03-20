📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1079-writer-monad)**

---

# Example 1079: Writer Monad — Logging Computation

**Difficulty:** ⭐⭐⭐
**Category:** Monadic Patterns
**OCaml Source:** https://cs3110.github.io/textbook/chapters/ds/monads.html

## Problem Statement

Implement a Writer monad that accumulates a log of messages alongside a computation. Chain operations that produce both a result and log entries, combining them transparently.

## Learning Outcomes

- Writer monad as a pattern for structured logging without side effects
- Monadic `bind` in Rust via method chaining vs OCaml's `>>=` operator
- Generic monoid abstraction for the log type
- How Rust's ownership makes `bind` naturally consume the previous state

## OCaml Approach

OCaml defines Writer as a record with `value` and `log` fields. The `>>=` operator (bind) applies a function to the value and concatenates the logs. `tell` creates a log-only entry. The pipeline reads naturally with `>>=` and `fun` closures.

## Rust Approach

Rust implements Writer as a generic struct with `bind` and `map` methods. Method chaining (`.bind(half).bind(...)`) replaces OCaml's `>>=` operator. A generic version parameterized by a `Monoid` trait shows how the pattern generalizes beyond `Vec<String>`.

## Key Differences

1. **Operator overloading:** OCaml defines `>>=` easily; Rust uses method chaining instead (operator overloading is possible but less ergonomic for monads)
2. **Ownership:** Rust's `bind` consumes `self`, making it clear the old Writer is gone. OCaml's bind copies/shares the log via GC
3. **Monoid abstraction:** OCaml uses `@` (list append) directly; Rust can abstract over the log type with a `Monoid` trait
4. **Type inference:** OCaml infers everything from usage; Rust sometimes needs explicit type annotations on closures

## Exercises

1. Extend the `Writer` monad to use a generic log type that implements `Monoid` (not just `String`), and use it to accumulate a structured audit log as a `Vec<LogEntry>`.
2. Implement `censor` — a function that transforms the accumulated log entries using a provided function — and use it to redact sensitive values from a computation log.
3. Build a multi-step computation pipeline using the writer monad that tracks both a string log and a numeric performance metric (operation count) simultaneously, using a product monoid.

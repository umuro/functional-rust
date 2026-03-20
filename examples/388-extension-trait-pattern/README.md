📖 **[View on hightechmind.io →](https://hightechmind.io/rust/388-extension-trait-pattern)**

---

# 388: Extension Trait Pattern

## Problem Statement

You want to add methods to a type you don't own — `str`, `Vec`, an external library type. Rust's orphan rule prevents implementing foreign traits on foreign types directly, but the extension trait pattern works around this: define a new trait in your crate, implement it for the foreign type, and bring it into scope with `use`. This is how `itertools` adds hundreds of methods to `Iterator`, how `tokio` adds async utilities to `TcpStream`, and how domain-specific libraries enrich standard library types.

The extension trait pattern is foundational to Rust's "composable without modification" philosophy and powers `rayon`'s `ParallelIterator`, `bytes::BufMut`, and `serde`'s derived impls.

## Learning Outcomes

- Understand the extension trait pattern as a solution to the orphan rule constraint
- Learn how to implement traits for foreign types (`str`, `Vec<T>`) in your crate
- See how `use MyExt` brings extension methods into scope at the call site
- Understand the ergonomics: extension methods appear alongside native methods in IDE completion
- Learn when extension traits are appropriate vs. free functions

## Rust Application

In `src/lib.rs`, `StrExt` adds `word_count`, `capitalize_words`, and `is_palindrome` to `str`. The `impl StrExt for str` block provides the implementations. Similarly `VecExt<T>` adds `second()` to `Vec<T>`. Because these traits are defined in this crate, implementing them for foreign types (`str`, `Vec`) is allowed. Callers just write `"hello world".word_count()` after `use crate::StrExt`.

## OCaml Approach

OCaml achieves extension through module inclusion: `module StringExt = struct include String; let word_count s = ... end`. This creates a new module wrapping the original with added functions. Unlike Rust's extension traits, OCaml's approach produces a new module rather than patching the original type's method namespace. There are no orphan restrictions since OCaml's type system doesn't tie methods to types.

## Key Differences

1. **Scoping**: Rust extension methods only appear when the trait is in scope (`use StrExt`); OCaml module functions are always available when the module is open.
2. **Discoverability**: Rust IDEs show extension methods alongside native methods in completion; OCaml requires knowing which module to open.
3. **Conflict resolution**: If two extension traits add the same method name, Rust requires explicit disambiguation (`StrExt::word_count(&s)`); OCaml resolves by module shadowing (last `open` wins).
4. **Ownership of impl**: Rust requires your crate to own either the trait or the type; OCaml has no such restriction.

## Exercises

1. **Numeric extension**: Define `NumExt` trait and implement it for `i32` with methods `clamp_to_range(lo: i32, hi: i32) -> i32`, `digits() -> Vec<u8>`, and `is_prime() -> bool`.
2. **Option extension**: Create `OptionExt<T>` that adds `ok_or_log(msg: &str) -> Option<T>` (logs a warning when `None`) and `map_or_default<U: Default, F: Fn(T) -> U>(self, f: F) -> U`.
3. **Iterator extension**: Implement a `StatsExt` trait for `Iterator<Item = f64>` that adds `mean() -> Option<f64>`, `variance() -> Option<f64>`, and `histogram(buckets: usize) -> Vec<usize>` methods, consuming the iterator.

📖 **[View on hightechmind.io →](https://hightechmind.io/rust/515-closure-once-cell)**

---

# Lazy Evaluation with OnceLock

## Problem Statement

Lazy initialization solves a fundamental tension: some values are expensive to compute but not always needed, and global mutable state is unsafe in concurrent programs. Before `OnceLock`, Rust programs used `unsafe` code or external crates like `lazy_static` for program-global lazy values. `std::sync::OnceLock` (stabilized in Rust 1.70) provides a safe, lock-free, thread-safe cell initialized exactly once. This pattern is ubiquitous in database connection pools, configuration parsers, and compiled regex caches.

## Learning Outcomes

- How `OnceLock<T>` guarantees initialization happens exactly once, even under concurrent access
- Why lazy initialization avoids paying the cost of computation when a value is never used
- How to build structs with lazily computed derived fields using `OnceLock` members
- The difference between `OnceLock` (runtime init) and `const`/`static` (compile-time init)
- How the `Lazy<T, F>` pattern encapsulates initialization logic alongside the value

## Rust Application

`EXPENSIVE_VALUE: OnceLock<i64>` is a program-global static. The first call to `get_or_init` executes the closure and stores the result; subsequent calls return the cached value with no locking overhead. `LazyConfig` stores `OnceLock<Vec<String>>` and `OnceLock<usize>` fields — each initialized on first access via `get_or_init`, allowing struct construction to remain cheap. The custom `Lazy<T, F>` wraps both the cell and its initializer function, mimicking the `once_cell::Lazy` API.

Key patterns:
- `OnceLock::get_or_init(|| ...)` — run closure at most once, return reference to stored value
- Multiple `OnceLock` fields for independent lazy derivations in one struct
- `const fn new(init: F)` — allow `Lazy` to be used in static context

## OCaml Approach

OCaml uses `Lazy.t` — a built-in type for deferred computation. `lazy expr` creates a thunk; `Lazy.force` evaluates it on first call and memoizes the result. OCaml's garbage collector handles the memory, and the runtime ensures thread safety via a mutex per lazy cell in OCaml 5.x.

```ocaml
let expensive = lazy ((List.fold_left (+) 0 (List.init 1000 (fun i -> i + 1))))
let value = Lazy.force expensive  (* computed once *)
```

## Key Differences

1. **Language integration**: OCaml has `lazy`/`Lazy.force` as first-class syntax and stdlib types; Rust uses `std::sync::OnceLock` as a library type without special syntax.
2. **Thread safety model**: `OnceLock` uses atomic operations for lock-free initialization; OCaml's `Lazy.t` in 5.x uses a per-cell mutex, simpler but with more overhead.
3. **Failure handling**: OCaml's `Lazy.force` can raise exceptions from the initializer; Rust's `get_or_init` panics if the initializer panics (poison), and `get_or_try_init` returns `Result`.
4. **Struct fields**: Rust can have multiple independent `OnceLock` fields in one struct; OCaml wraps individual `Lazy.t` values in records with the same ergonomics.

## Exercises

1. **Lazy regex cache**: Build a struct `RegexCache` with a `OnceLock<Vec<String>>` field that lazily compiles a list of patterns from a raw comma-separated string on first access.
2. **Cached factorial**: Implement a `FactorialCache` that computes and caches `n!` for `n` up to 20 using an array of `OnceLock<u64>`, ensuring each entry is computed only once.
3. **Initialization error**: Modify `Lazy<T, F>` to use `OnceLock<Result<T, String>>` so initialization failures are stored and returned on every subsequent access instead of panicking.

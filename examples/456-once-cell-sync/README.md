📖 **[View on hightechmind.io →](https://hightechmind.io/rust/456-once-cell-sync)**

---

# 456: `OnceLock` and `OnceCell` — Once-Initialized Values
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Some values are expensive to initialize (parse config, compile regex, connect to DB) and should only be initialized once. Race conditions arise if multiple threads try to initialize simultaneously. `OnceLock<T>` provides thread-safe once-initialization: `get_or_init(|| expensive_computation())` guarantees the closure runs exactly once, even under concurrent access. `OnceCell<T>` is the single-threaded version. Both are simpler and more ergonomic than the `lazy_static!` macro they largely replace.

`OnceLock` is used for global singletons, global configuration, compiled regex caches, connection pools initialized once, and any value needing lazy evaluation with thread-safe initialization guarantee.

## Learning Outcomes

- Understand the difference between `OnceLock<T>` (thread-safe) and `OnceCell<T>` (single-thread)
- Learn how `get_or_init` guarantees exactly-once initialization under concurrent access
- See how `static CONFIG: OnceLock<HashMap<...>>` creates a global lazy singleton
- Understand that `OnceLock` is the modern replacement for `lazy_static!` for most use cases
- Learn when `OnceCell` suffices (no concurrent access to the cell itself)

## Rust Application

In `src/lib.rs`, `static CONFIG: OnceLock<HashMap<&str, &str>>` is a global that initializes on first call to `config()`. The `println!("init config")` inside `get_or_init` demonstrates single initialization. The test with `AtomicU32` counter verifies the closure runs exactly once even with concurrent callers. `static GREETING: OnceLock<String>` shows runtime-computed strings as statics. `OnceCell` is for single-threaded contexts.

## OCaml Approach

OCaml's `Lazy.t` is the standard lazy value: `let config = lazy (make_config ())` and `Lazy.force config` for access. `Lazy.t` is thread-safe in OCaml 5.x with the standard library's guarantee of single evaluation. In OCaml 4.x, `Lazy.t` uses a mutex internally for thread safety. The `Lazy.from_val` function creates an already-evaluated lazy value.

## Key Differences

1. **Language vs. library**: OCaml's `lazy` is a language keyword; Rust's `OnceLock` is a standard library type.
2. **Global statics**: Rust `static OnceLock` enables lazy global initialization; OCaml `let global_val = lazy (...)` achieves the same at module level.
3. **Panic safety**: Rust's `OnceLock` handles panics in `get_or_init` by allowing retry; OCaml's `Lazy.force` re-raises the exception on retry.
4. **`once_cell` crate**: Before stabilization in std, the `once_cell` crate provided `Lazy<T>` which initializes on first deref — slightly more ergonomic than `OnceLock`.

## Exercises

1. **Global logger**: Create `static LOGGER: OnceLock<Logger>` where `Logger` reads its configuration from environment variables. Verify that the environment is read only once and subsequent calls use the cached configuration.
2. **Regex cache**: Implement `fn is_valid_ipv4(s: &str) -> bool` using `static RE: OnceLock<Regex>`. Verify with 10 concurrent threads calling the function that the regex is compiled exactly once.
3. **OnceCell injection**: Use `OnceCell<Box<dyn Database>>` in a `struct AppState` to enable injecting different database implementations in tests vs. production, initializing once via `set` from the setup code.

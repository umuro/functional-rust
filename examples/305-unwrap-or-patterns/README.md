📖 **[View on hightechmind.io →](https://hightechmind.io/rust/305-unwrap-or-patterns)**

---

# 305: unwrap_or, unwrap_or_else, unwrap_or_default

**Difficulty:** 1  **Level:** Beginner

Extract a value from `Option` or `Result` with a safe fallback — never panic.

## The Problem This Solves

You call a function that returns `Option<T>` or `Result<T, E>`, but you don't want to propagate the error — you have a sensible default value to use instead. `unwrap()` would panic on `None` or `Err`. Writing a full `match` to return a literal default is verbose. You need something in between.

The `unwrap_or` family covers three patterns: an immediate fallback value, a lazily-computed fallback, and a type-default fallback. They're the idiomatic way to say "use this value if the operation succeeded, otherwise fall back."

Choosing between them matters more than it looks. `unwrap_or(x)` evaluates `x` *before* calling the function — so if `x` is an expensive allocation like `Vec::new()` or a string format, you're paying that cost even when the option is `Some`. `unwrap_or_else(|| x)` defers evaluation to a closure, so the cost is only paid when needed.

## The Intuition

`unwrap_or` variants are safe alternatives to `unwrap()` when you have a default: eager (`unwrap_or`), lazy (`unwrap_or_else`), or type-derived (`unwrap_or_default`).

## How It Works in Rust

```rust
// unwrap_or: always evaluates the default — fine for Copy types and literals
let port: u16 = Some(8080u16).unwrap_or(3000);   // => 8080
let port: u16 = None.unwrap_or(3000);             // => 3000

// unwrap_or_else: lazy — default closure only runs on None/Err
let name = std::env::var("APP_NAME")
    .unwrap_or_else(|_| "my-app".to_string());  // String only allocated if var is unset

// unwrap_or_default: uses the type's Default impl
let nums: Vec<i32> = None::<Vec<i32>>.unwrap_or_default();  // => []
let s: String = None::<String>.unwrap_or_default();         // => ""
let n: i32 = None::<i32>.unwrap_or_default();               // => 0

// Works on Result too
let n: i32 = "42".parse::<i32>().unwrap_or(0);   // => 42
let n: i32 = "bad".parse::<i32>().unwrap_or(0);  // => 0 (parse failed)

// Practical: parse config with defaults
let port: u16 = std::env::var("PORT")
    .ok()                          // Result<String, VarError> -> Option<String>
    .and_then(|s| s.parse().ok())  // Option<String> -> Option<u16>
    .unwrap_or(8080);              // Option<u16> -> u16
```

The `ok()` conversion (Result → Option) is a common companion: it discards the error, leaving `None` for `Err`, ready for `unwrap_or`.

## What This Unlocks

- **Configuration defaults** — `env::var("KEY").unwrap_or_else(|| default.to_string())` is idiomatic config loading
- **Fallback computations** — lazy evaluation in `unwrap_or_else` avoids unnecessary work on the happy path
- **Zero-value defaults** — `unwrap_or_default()` works whenever `T: Default` — vecs, strings, numbers, structs

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Eager default | `Option.value ~default:x` | `opt.unwrap_or(x)` — `x` always evaluated |
| Lazy default | `Option.value_map ~default:(fun () -> x) f` | `opt.unwrap_or_else(|| x)` — closure only called on `None` |
| Type default | Manual `Option.value ~default:(zero_of_type ())` | `opt.unwrap_or_default()` — requires `T: Default` |
| Panicking version | `Option.get` raises | `unwrap()` — avoid in production code |

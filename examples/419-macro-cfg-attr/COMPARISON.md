# OCaml vs Rust: Conditional Compilation

## Rust cfg!

```rust
// Compile-time condition check
if cfg!(target_os = "linux") {
    // Linux-specific code
}

// Conditional compilation
#[cfg(feature = "logging")]
fn log(msg: &str) { println!("{}", msg); }

#[cfg(not(feature = "logging"))]
fn log(_: &str) {}

// Conditional attributes
#[cfg_attr(feature = "serde", derive(Serialize))]
struct Data { ... }
```

## OCaml Conditional Compilation

```ocaml
(* Using preprocessor *)
#ifdef DEBUG
let debug = true
#else
let debug = false
#endif

(* Or dune features *)
(* In dune file: (enabled_if (= %{profile} dev)) *)
```

## 5 Takeaways

1. **`cfg!` returns bool at compile time.**
2. **`#[cfg(...)]` includes/excludes items.**
3. **`#[cfg_attr(...)]` adds conditional attributes.**
4. **Feature flags via `Cargo.toml` `[features]`.**
5. **OCaml uses preprocessor or dune configs.**

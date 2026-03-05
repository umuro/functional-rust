📖 **[View on hightechmind.io →](https://hightechmind.io/rust/407-default-trait)**

---

# 407: Default Trait and Initialization

**Difficulty:** 1  **Level:** Beginner

The `Default` trait provides a standard way to create "zero value" instances of your types.

## The Problem This Solves

Every type needs a sensible starting state. Counters start at zero. Strings start empty. Config structs have sane defaults (host: "localhost", port: 8080, retries: 3). Without a standard way to express this, every API that needs a default must either hardcode it internally or require the caller to provide all fields.

`Default` solves this by giving every type a canonical zero value. The standard library uses it everywhere: `Option::unwrap_or_default()`, `HashMap::entry().or_default()`, struct update syntax `Config { port: 9090, ..Config::default() }`. Without `Default`, all of these APIs would need separate workarounds.

For simple types, `#[derive(Default)]` works automatically: numbers get 0, booleans get false, strings get empty, Options get None, Vecs get empty. For types with domain-specific sensible defaults, you implement `Default` manually and set meaningful values.

## The Intuition

`Default` is a trait with a single method: `fn default() -> Self`. That's it. Implementing it says "this type has a canonical empty/zero/initial state." The `#[derive(Default)]` macro generates this automatically by calling `Default::default()` on each field — so every field type must also implement `Default`.

The real value is composability: once your type implements `Default`, the entire standard library ecosystem that uses `Default` bounds works with your type automatically.

## How It Works in Rust

```rust
// Derive: fields get language-level zero values
#[derive(Debug, Default)]
struct ServerConfig {
    host: String,        // ""
    port: u16,           // 0
    max_connections: u32, // 0
    debug: bool,         // false
}

// Manual: domain-specific sensible defaults
#[derive(Debug)]
struct AppConfig {
    host: String,
    port: u16,
    max_connections: u32,
    debug: bool,
    timeout_secs: f64,
    retry_count: u8,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            host: "localhost".to_string(),
            port: 8080,
            max_connections: 100,
            debug: false,
            timeout_secs: 30.0,
            retry_count: 3,
        }
    }
}

fn main() {
    // Struct update syntax: override only what changes
    let custom = AppConfig {
        port: 9090,
        debug: true,
        ..AppConfig::default()  // fill remaining fields from default
    };
    println!("{:?}", custom);

    // or_default() in collections — elegant, no unwrap needed
    use std::collections::HashMap;
    let mut word_count: HashMap<&str, u32> = HashMap::new();
    for word in ["hello", "world", "hello", "rust", "hello"] {
        *word_count.entry(word).or_default() += 1;
        // or_default() returns &mut u32, creating 0 if key absent
    }
    println!("{:?}", word_count); // {"hello": 3, "world": 1, "rust": 1}

    // unwrap_or_default: None becomes the type's default
    let opt: Option<Vec<i32>> = None;
    let v = opt.unwrap_or_default();
    println!("unwrap_or_default: {:?}", v); // []
}
```

A useful pattern — generic builders that fill missing fields with defaults:
```rust
fn configure(overrides: impl FnOnce(&mut AppConfig)) -> AppConfig {
    let mut cfg = AppConfig::default();
    overrides(&mut cfg);
    cfg
}

let cfg = configure(|c| { c.port = 9000; c.debug = true; });
```

## What This Unlocks

- **Struct update syntax** — `MyStruct { field: value, ..MyStruct::default() }` lets callers specify only what's non-default; essential for large config structs.
- **Collection ergonomics** — `entry().or_default()`, `unwrap_or_default()`, and `Option::get_or_insert_default()` eliminate boilerplate initialization patterns.
- **Generic zero-value construction** — `fn reset<T: Default>(&mut self) { self.state = T::default(); }` works for any `Default` type without knowing the concrete type.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Default value | `default_config` record literal — a value, not a trait | `Default` trait — standardized, works with generics and `#[derive]` |
| Struct update | `{ default_config with port = 9090 }` — same record-update syntax | `Config { port: 9090, ..Config::default() }` — identical idiom |
| Collection defaults | `Hashtbl.find_opt` + manual `Option.value` | `entry().or_default()` — single method, uses `Default` |
| Propagation | Manual in each module | `#[derive(Default)]` propagates — works if all fields are `Default` |

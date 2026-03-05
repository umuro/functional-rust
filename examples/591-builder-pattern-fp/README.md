📖 **[View on hightechmind.io →](https://hightechmind.io/rust/591-builder-pattern-fp)**

---

# 591: Builder Pattern (Functional Style)

**Difficulty:** 3  **Level:** Intermediate

Build complex values through method chains that consume and return `Self` — immutable, composable, and type-safe configuration.

## The Problem This Solves

Complex structs often have many optional fields with sensible defaults. The naive solution is a constructor with 12 parameters — callers must know the order, can't skip arguments, and the call site is unreadable. The classic OOP fix is a mutable builder, but mutable builders can't be shared, partially applied, or stored as a "template" to derive variants from.

The functional builder solves both problems. Each `with_*` method takes `self` by value, modifies one field, and returns `Self`. Because every step is a value transformation, you can store a base configuration and derive specializations from it. Tests can build a "default good config" and override one field per test case without mutation.

The pattern extends to type-state builders with phantom types, where `build()` only compiles when all required fields have been set — turning runtime panics into compile errors. The compiler enforces that you've configured everything mandatory.

## The Intuition

A functional builder is a series of value transformations: each `with_*` returns a new (or consumed) `Self` with one field changed, and `build()` produces the final value — the key difference from a mutable builder is composability: you can branch from any intermediate state. The trade-off: each step may clone/move the struct, which is negligible for configs but matters for large types.

## How It Works in Rust

```rust
#[derive(Default, Clone)]
struct ServerConfig {
    host: String,
    port: u16,
    max_connections: usize,
    tls: bool,
}

impl ServerConfig {
    fn new() -> Self { Self::default() }

    // Each method consumes self and returns Self — no mutation visible to caller
    fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self  // return moved self
    }
    fn with_port(mut self, port: u16) -> Self { self.port = port; self }
    fn with_max_connections(mut self, n: usize) -> Self { self.max_connections = n; self }
    fn with_tls(mut self) -> Self { self.tls = true; self }

    fn build(self) -> Result<Server, &'static str> {
        if self.host.is_empty() { return Err("host required"); }
        Ok(Server { config: self })
    }
}

// Base config — stored as template
let base = ServerConfig::new()
    .with_host("localhost")
    .with_max_connections(100);

// Derive two variants without mutation
let dev  = base.clone().with_port(8080);
let prod = base.clone().with_port(443).with_tls();
```

## What This Unlocks

- **Test fixtures**: store a base "valid config" and override one field per test — no mutation needed.
- **Configuration DSLs**: library users get a fluent API; library author controls which combinations are valid.
- **Type-state builders**: phantom type parameters enforce that required fields are set before `build()` compiles.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Chaining style | `\|>` pipe operator | `.method()` chains on `Self` |
| Immutable step | New record each step | `self`-consuming returns new `Self` |
| Mutable step | Explicit `ref` | `mut self` internally — immutable externally |
| Validation | `Result`-wrapped `build` | `build() -> Result<T, E>` |
| Defaults | Explicit record literal | `Default` trait + `#[derive(Default)]` |
| Type-state | GADTs / phantom types | Phantom type parameters on builder struct |

📖 **[View on hightechmind.io →](https://hightechmind.io/rust/524-closure-builder-closures)**

---

# 524: Builder Pattern with Closures

**Difficulty:** 3  **Level:** Intermediate

Accept closures as configuration steps in builder APIs.

## The Problem This Solves

The builder pattern is idiomatic Rust for constructing complex objects. Usually each setter takes a value and returns `Self`. But sometimes you want to pass a block of configuration at once, or supply a callback that will be invoked later (on connect, on error, on request). Accepting closures in the builder API enables both: batch configuration via `configure(|c| { c.port = 3000; c.timeout = 1000; })`, and stored callbacks via `on_connect(|addr| println!("connected: {}", addr))`.

This matters because stored callbacks let you inject behavior into a struct without subclassing or trait objects on the caller's side. The caller just passes a closure. The struct stores it as `Box<dyn Fn(&str)>`. When the event fires, the stored closure runs. This is a common pattern in Rust servers, GUI libraries, and test harnesses.

The `configure` step — accepting a `FnOnce(&mut Config)` — is particularly useful: it gives the caller direct access to the partial config to set multiple fields at once, without needing a method for each one.

## The Intuition

Each builder method either takes a value (`.port(9090)`) or takes a closure (`.on_connect(|addr| ...)` or `.configure(|c| { ... })`). Value setters are simple. Closure setters either store the closure for later invocation or run it immediately against the partial config. The builder chain reads like a configuration DSL.

## How It Works in Rust

1. **Stored callback** — `on_connect(mut self, handler: impl Fn(&str) + 'static) -> Self` boxes the closure into `Box<dyn Fn(&str)>` and stores it in the config.
2. **`configure` closure** — `configure(mut self, f: impl FnOnce(&mut ServerConfig)) -> Self` runs `f` against the partial config immediately; lets the caller set multiple fields in one step.
3. **`'static` bound** — stored callbacks must be `'static` (no references to local variables) since the config outlives the builder call site.
4. **Function-arg builder** — `fn build_server(configure: impl FnOnce(ServerBuilder) -> ServerBuilder) -> ServerConfig` lets callers write the entire chain as a lambda: `build_server(|b| b.port(3000).on_connect(...))`.
5. **Captured environment** — closures passed to `on_connect` can capture local variables: `let log_prefix = "SERVER"; .on_connect(move |addr| println!("[{}] {}", log_prefix, addr))`.

## What This Unlocks

- Register behavior (connection handlers, error hooks, middleware) inline in builder chains.
- Let callers batch-configure multiple fields with a single `configure(|c| { ... })` step.
- Write test harnesses and mock servers where behavior is injected entirely via closures.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Builder pattern | Not idiomatic; usually labeled arguments | Method-chaining builder with `Self` return is standard |
| Stored callbacks | Function references in records | `Box<dyn Fn(...)>` field; `'static` lifetime bound |
| Batch configuration | Record update syntax `{ cfg with port=3000 }` | `configure(FnOnce(&mut Config))` — runs closure against partial config |
| Closure capture in config | Transparent; GC handles | `move` closure must be `'static` when stored in a struct |

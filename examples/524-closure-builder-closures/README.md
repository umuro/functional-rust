📖 **[View on hightechmind.io →](https://hightechmind.io/rust/524-closure-builder-closures)**

---

# Builder Pattern with Closures
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

The builder pattern addresses the "telescoping constructor" problem: when a type has many optional fields, constructors become unwieldy and error-prone. Rust's builder idiom (fluent API) is widespread in production code — `reqwest::ClientBuilder`, `tokio::runtime::Builder`, `std::thread::Builder` all use it. Adding closures to builders enables behavior injection: instead of just configuring data fields, callers can inject callbacks for connection events, error handlers, or transformation pipelines. This makes APIs both configurable and extensible without requiring trait implementations.

## Learning Outcomes

- How to combine the builder pattern with closure callbacks for behavior injection
- Why `Box<dyn Fn(&str)>` stores callbacks in builder structs without generics on the struct itself
- How method chaining (`self -> Self`) works with closure-accepting methods
- How `Default` provides sensible no-op closures for optional callbacks
- Where this pattern appears: HTTP client builders, async runtime builders, test harnesses

## Rust Application

`ServerConfig` stores `on_connect: Box<dyn Fn(&str)>` alongside data fields. `ServerBuilder` wraps `ServerConfig` and provides fluent methods returning `Self`: `host(s)`, `port(n)`, `on_connect(f)`. The `Default` impl provides a no-op `Box::new(|_| {})` callback so builds without a callback compile and run silently. `on_connect` accepts `impl Fn(&str) + 'static`, boxing it internally so callers pass raw closures without explicit boxing.

Key patterns:
- `Box<dyn Fn(&str)>` on the struct — no generic parameter needed on `ServerConfig`
- `pub fn on_connect(mut self, f: impl Fn(&str) + 'static) -> Self` — fluent builder method
- `Box::new(|_| {})` default no-op callback in `Default` impl

## OCaml Approach

OCaml builders are typically records with optional fields using `option` types for callbacks. A builder function takes a record and returns an updated copy using functional update syntax. Callbacks are plain functions stored in `option` fields:

```ocaml
type server_config = {
  host: string; port: int;
  on_connect: (string -> unit) option;
}
let with_on_connect f cfg = { cfg with on_connect = Some f }
```

## Key Differences

1. **Struct generics**: Rust builders often avoid generics on the struct by boxing callbacks (`Box<dyn Fn>`); OCaml records store functions directly without boxing annotation.
2. **Fluent chaining**: Rust `self -> Self` enables `Builder::new().host("x").port(80).build()` in idiomatic Rust; OCaml achieves this with function composition or `|>` pipelines.
3. **No-op defaults**: Rust's `Default` trait provides a no-op closure; OCaml uses `option` to represent absence, calling `Option.iter on_connect addr` at use time.
4. **Ownership model**: Rust builders consume `self` on each step (`mut self` pattern) preventing reuse after building; OCaml records are immutable by default — functional update creates a new record each step.

## Exercises

1. **Retry builder**: Add an `on_retry(f: impl Fn(u32, &str) + 'static)` callback to `ServerBuilder` that receives the attempt number and error message, and a `max_retries(n: u32)` field.
2. **Transform pipeline**: Add `add_transform(f: impl Fn(String) -> String + 'static)` to the builder that accumulates multiple transforms, applied in registration order when a request arrives.
3. **Validation in build**: Make `ServerBuilder::build()` return `Result<ServerConfig, String>` that validates the port is in range `1..=65535` and the host is non-empty before returning the config.

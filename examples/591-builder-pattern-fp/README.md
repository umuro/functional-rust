📖 **[View on hightechmind.io →](https://hightechmind.io/rust/591-builder-pattern-fp)**

---

# Functional Builder Pattern
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

The builder pattern addresses the "telescoping constructor" problem — many optional fields make constructors unwieldy. The functional variant uses consuming methods (`self -> Self`) instead of `&mut self`, creating an immutable chain. Each method returns a new value with one field changed. This style is prevalent in Rust's standard library (`std::thread::Builder`) and ecosystem (`reqwest::ClientBuilder`). It is also related to OCaml's record functional update syntax `{ record with field = new_val }`.

## Learning Outcomes

- How consuming builder methods `fn field(mut self, val: T) -> Self` enable method chaining
- How `Default` provides sensible starting configuration for builders
- How functional update eliminates the need for separate `Builder` and `Config` types
- How to implement `build()` that validates the configuration before returning it
- Where the functional builder pattern appears: HTTP clients, runtime builders, parser configs

## Rust Application

`Config` implements `Default` with sensible values. Consuming methods: `fn host(mut self, h: impl Into<String>) -> Self { self.host = h.into(); self }`. Chaining: `Config::default().host("example.com").port(443).tls(true)`. The `Default` + consuming methods pattern is zero-overhead — the compiler inlines and optimizes away the intermediate values.

Key patterns:
- `fn method(mut self, val: T) -> Self` — consuming builder method
- `impl Default for Config` — starting point for building
- `.host("x").port(443).tls(true)` — method chain
- `fn build(self) -> Result<Self, String>` — validated construction

## OCaml Approach

OCaml uses functional record update syntax directly:

```ocaml
type config = { host: string; port: int; tls: bool; timeout: float }
let default_config = { host = "localhost"; port = 80; tls = false; timeout = 30.0 }
let with_host h c = { c with host = h }
let with_port p c = { c with port = p }
(* Usage: default_config |> with_host "example.com" |> with_port 443 *)
```

## Key Differences

1. **Consuming vs functional update**: Rust consumes `self` at each step (ownership transfer); OCaml creates a new record at each step (GC-managed copy).
2. **Method syntax**: Rust `.host("x")` is a method call; OCaml `with_host "x" config` is a function call, typically chained with `|>`.
3. **Validation**: Rust `build()` can return `Result<Config, Error>` for validation; OCaml typically validates in a separate `check` function.
4. **Type safety**: Both enforce that missing fields have defaults through the `Default` trait or default record value.

## Exercises

1. **Validation**: Add `fn build(self) -> Result<Config, String>` that returns `Err` if `host` is empty or `port` is 0.
2. **With-fn style**: Rewrite the builder as free functions `fn with_host(h: &str, c: Config) -> Config` that can be chained with `.pipe(|c| with_host("x", c))`.
3. **Builder struct**: Create a separate `ConfigBuilder` struct with `&mut self` methods and a `build() -> Config` that consumes the builder — compare API ergonomics with the consuming-self style.

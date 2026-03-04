# 735: Typestate Builder: Required Fields at Compile Time

**Difficulty:** 4  **Level:** Expert

A builder where each required field advances the type — `Builder<Unset, Unset>` → `Builder<Set, Unset>` → `Builder<Set, Set>` — so `build()` only exists when every required field has been provided.

## The Problem This Solves

The classic builder pattern uses `Option` fields and validates at `build()` time: if a required field is `None`, return an error or panic. This pushes a programming mistake — forgetting to call `.host()` — into a runtime failure. The compiler can't help you. Your tests catch it if you write them; your users catch it if you don't.

Every production Rust codebase has at least one builder that returns `Result<T, BuildError>` or panics with "field X is required." Both are worse than a compile error. A compile error is instant, zero-cost, and impossible to silence at the call site.

The typestate builder moves the constraint into the type system. `build()` is defined only on `HttpClientBuilder<Set, Set>` — both required fields provided. Call it on `HttpClientBuilder<Unset, Set>` and you get `error[E0599]: method not found`. The mistake is caught before the code compiles, let alone runs.

## The Intuition

The builder struct is generic over phantom type parameters — one per required field. Each parameter is either `Unset` or `Set`, zero-sized marker types that carry no data. They're purely for the type checker.

Setting a required field is a consuming transition: `fn host(self, h: ...) -> HttpClientBuilder<Set, HasPort>`. The `HasHost` type parameter changes from `Unset` to `Set`. Optional fields work on any state (`impl<H, P> HttpClientBuilder<H, P>`). `build()` is only `impl`-ed on `HttpClientBuilder<Set, Set>`.

The magic: `PhantomData<(HasHost, HasPort)>` occupies zero bytes at runtime. The whole builder collapses to just its field values; the state tracking is purely a compile-time construct that the compiler optimises away completely.

## How It Works in Rust

```rust
use std::marker::PhantomData;

pub struct Set;    // zero-sized marker — field was provided
pub struct Unset;  // zero-sized marker — field not yet provided

pub struct HttpClientBuilder<HasHost, HasPort> {
    host:        Option<String>,
    port:        Option<u16>,
    timeout_ms:  u64,          // optional — has a default
    max_retries: u32,          // optional — has a default
    _phantom:    PhantomData<(HasHost, HasPort)>,  // zero bytes
}

// Entry point — both required fields start as Unset
impl HttpClientBuilder<Unset, Unset> {
    pub fn new() -> Self {
        HttpClientBuilder { host: None, port: None,
                            timeout_ms: 5_000, max_retries: 3,
                            _phantom: PhantomData }
    }
}

// Setting host: HasHost transitions Unset → Set; HasPort unchanged
impl<HasPort> HttpClientBuilder<Unset, HasPort> {
    pub fn host(self, h: impl Into<String>) -> HttpClientBuilder<Set, HasPort> {
        HttpClientBuilder { host: Some(h.into()), port: self.port,
                            timeout_ms: self.timeout_ms, max_retries: self.max_retries,
                            _phantom: PhantomData }
    }
}

// Setting port: HasPort transitions Unset → Set; HasHost unchanged
impl<HasHost> HttpClientBuilder<HasHost, Unset> {
    pub fn port(self, p: u16) -> HttpClientBuilder<HasHost, Set> {
        HttpClientBuilder { host: self.host, port: Some(p),
                            timeout_ms: self.timeout_ms, max_retries: self.max_retries,
                            _phantom: PhantomData }
    }
}

// Optional setters available in any state (no type transition)
impl<H, P> HttpClientBuilder<H, P> {
    pub fn timeout_ms(mut self, ms: u64) -> Self { self.timeout_ms = ms; self }
    pub fn max_retries(mut self, n: u32) -> Self { self.max_retries = n; self }
}

// build() ONLY exists when BOTH are Set — missing a required field = compile error
impl HttpClientBuilder<Set, Set> {
    pub fn build(self) -> HttpClient {
        HttpClient { host: self.host.unwrap(), port: self.port.unwrap(),
                     timeout_ms: self.timeout_ms, max_retries: self.max_retries }
    }
}

// ── Valid — compiles ───────────────────────────────────────────────────────────
let client = HttpClientBuilder::new()
    .host("api.example.com")   // type becomes Builder<Set, Unset>
    .port(8080)                // type becomes Builder<Set, Set>
    .timeout_ms(3_000)         // type stays Builder<Set, Set>
    .build();                  // only available on Builder<Set, Set>

// ── Invalid — compile errors ───────────────────────────────────────────────────
// HttpClientBuilder::new().build()              // missing host AND port
// HttpClientBuilder::new().host("x").build()    // missing port
// HttpClientBuilder::new().port(80).build()     // missing host
```

## What This Unlocks

- **Impossible-to-misuse APIs** — users physically cannot call `build()` without providing every required field; no documentation required, no runtime validation needed.
- **Order independence** — `.host().port()` and `.port().host()` both produce `Builder<Set, Set>`; the type system doesn't care about order, only completeness.
- **Scalable to many required fields** — add a third phantom parameter `HasTimeout` and repeat the pattern; complexity scales linearly.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Compile-time required fields | GADTs with witness types; complex; not idiomatic | Phantom type parameters (`Set`/`Unset`) — idiomatic, zero overhead |
| Builder pattern | Named/labeled arguments for simple cases; GADT builder for strict enforcement | `Builder<Set, Set>` — `build()` only when all required |
| Runtime validation | `failwith "name is required"` in `build` | No runtime check needed — compiler enforces it |
| Cost of state tracking | N/A (runtime type) | `PhantomData` — zero bytes, zero runtime cost |

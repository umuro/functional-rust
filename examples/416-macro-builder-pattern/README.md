📖 **[View on hightechmind.io →](https://hightechmind.io/rust/416-macro-builder-pattern)**

---

# 416: Builder Pattern via Macro

**Difficulty:** 4  **Level:** Expert

Generate the builder boilerplate — struct, setter methods, and `build()` — from a macro, reducing repetitive code to a single declaration.

## The Problem This Solves

The builder pattern is extremely common in Rust for constructing complex objects with optional fields and validation. But it's verbose: you need the struct itself, a separate builder struct with `Option<T>` fields, individual setter methods for each field, and a `build()` method with validation. For a struct with 10 fields, that's 40+ lines of boilerplate that's mechanically derived from the field list.

Macros can generate all this from a field declaration. Instead of writing the full builder by hand, you define the fields once and let the macro expand it into the complete builder implementation. The macro guarantees consistency — adding a field in one place updates the struct, builder, and setter simultaneously. No risk of a setter being forgotten or the builder having different field names than the struct.

This example focuses on the realistic approach: a `setters!` macro that generates the repetitive setter methods, combined with hand-written `build()` logic that the macro can't know (required vs. optional fields, validation, defaults).

## The Intuition

The full builder macro is an advanced application of everything in the macro section: repetition to generate N setter methods from N fields, `ident` fragment to use the field name as both the method name and the field being set, `ty` fragment to use the correct type in the setter signature.

A practical pattern: generate only the setters (the most repetitive part) while writing `build()` by hand. This gives you the best of both worlds — less boilerplate, full control over validation logic.

## How It Works in Rust

```rust
// Generate setter methods from a field list
// Each setter: takes `self` by value, sets Option<T>, returns Self
macro_rules! setters {
    ($($field:ident : $ty:ty),* $(,)?) => {
        $(
            pub fn $field(mut self, val: $ty) -> Self {
                self.$field = Some(val);  // stores in Option<T> wrapper
                self
            }
        )*
    };
}

#[derive(Debug)]
struct HttpRequest {
    url: String,
    method: String,
    timeout_ms: u32,
    max_retries: u8,
    headers: Vec<(String, String)>,
}

#[derive(Default)]
struct HttpRequestBuilder {
    url: Option<String>,
    method: Option<String>,
    timeout_ms: Option<u32>,
    max_retries: Option<u8>,
    headers: Vec<(String, String)>,
}

impl HttpRequestBuilder {
    // Generate: fn url(mut self, val: String) -> Self
    //           fn method(mut self, val: String) -> Self
    //           fn timeout_ms(mut self, val: u32) -> Self
    //           fn max_retries(mut self, val: u8) -> Self
    setters!(url: String, method: String, timeout_ms: u32, max_retries: u8);

    // Special setter for Vec — manual because it accumulates
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    // build() is always manual — validation and defaults are domain logic
    pub fn build(self) -> Result<HttpRequest, String> {
        Ok(HttpRequest {
            url: self.url.ok_or("url is required")?,  // required field
            method: self.method.unwrap_or_else(|| "GET".to_string()),  // optional with default
            timeout_ms: self.timeout_ms.unwrap_or(5000),
            max_retries: self.max_retries.unwrap_or(3),
            headers: self.headers,
        })
    }
}

impl HttpRequest {
    fn builder() -> HttpRequestBuilder { HttpRequestBuilder::default() }
}

fn main() {
    // Fluent API — each setter returns Self, enabling chaining
    let req = HttpRequest::builder()
        .url("https://api.example.com/data".to_string())
        .method("POST".to_string())
        .timeout_ms(10_000)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .build()
        .unwrap();

    println!("URL: {}", req.url);
    println!("Method: {}", req.method);
    println!("Headers: {:?}", req.headers);

    // Missing required field — compile to Ok/Err
    let err = HttpRequest::builder().build();
    println!("Missing url: {:?}", err); // Err("url is required")
}
```

For full auto-generation, the `derive_builder` crate generates the entire builder from `#[derive(Builder)]`. Procedural macros (proc-macros) handle this more elegantly than `macro_rules!` because they have full access to the struct's AST.

## What This Unlocks

- **Ergonomic configuration APIs** — HTTP clients, database connection pools, CLI commands — all use builder pattern; the macro halves the code.
- **Consistent field naming** — setter names match field names by construction; no typos, no drift between struct and builder.
- **Scalable maintenance** — add a field to the list; the setter appears automatically. Remove it; the setter disappears. No hunting through implementation code.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Builder pattern | Mutable record `{ mutable field }` with setters, or pipe `\|>` composition | Builder struct with consuming setters that return `Self` — fluent chaining |
| Code generation | PPX derivers — `[@@deriving make]` generates constructors | `macro_rules!` or `derive_builder` proc-macro — generates builder type |
| Required vs optional | Optional args via `?port` in constructor | `Option<T>` in builder + `ok_or()?` in `build()` |
| Error on missing | Compile-time with optional labeled args | Runtime `Result` from `build()` — or type-state pattern for compile-time |

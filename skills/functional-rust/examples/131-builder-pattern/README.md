# 131: Builder Pattern with Typestate

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Construct complex objects step by step with a fluent API, where `build()` only compiles after all required fields have been set.

## The Problem This Solves

A struct with many fields — some required, some optional — is annoying to construct. You could add a `new(name, email, ...)` function, but with many fields it becomes unwieldy and easy to mix up. You could use a mutable builder with `Option` fields and check them in `build()`, but then you get runtime panics or `Result` errors for things you know at compile time: "did I call `.name()`?"

The typical builder pattern solves ergonomics but not safety. Nothing stops you from writing `UserBuilder::new().age(30).build()` — forgetting both `name` and `email`. You discover the error at runtime.

Typestate builders solve this: the builder type tracks which fields have been set. `UserBuilder<Optional, Optional>` doesn't have a `build()` method. `UserBuilder<Required, Required>` does. If you forget to call `.name()`, the types don't add up and you get a compile error before your code ever runs. Required fields are documented *in the type*, not just in doc comments.

## The Intuition

Take the typestate pattern (example 130) and apply it to a builder. Each "slot" in the builder — name, email, URL, method — is a phantom type parameter that starts as `Optional` and transitions to `Required` when you provide the value.

The builder struct holds the same fields throughout. Only the type changes. `UserBuilder<Optional, Optional>` and `UserBuilder<Required, Required>` are both the same struct in memory — but different types to the compiler. The `build()` method is in an `impl` block that only applies when all phantom parameters are `Required`.

Each setter method takes `self`, sets the field, and returns `self` with the relevant phantom parameter flipped to `Required`. The chain builds up state in types, not in runtime flags.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// Phantom markers — no data, just type labels
struct Required;
struct Optional;

// Builder: two phantom params track whether name and email have been set
struct UserBuilder<Name, Email> {
    name: Option<String>,
    email: Option<String>,
    age: Option<u32>,
    _phantom: PhantomData<(Name, Email)>,
}

// Start: neither field set
impl UserBuilder<Optional, Optional> {
    fn new() -> Self {
        UserBuilder { name: None, email: None, age: None, _phantom: PhantomData }
    }
}

// .name() is available on any builder where Name = Optional
// Returns a builder with Name = Required (Email stays the same)
impl<E> UserBuilder<Optional, E> {
    fn name(self, name: &str) -> UserBuilder<Required, E> {
        UserBuilder {
            name: Some(name.to_string()),
            email: self.email,
            age: self.age,
            _phantom: PhantomData,
        }
    }
}

// .email() is available on any builder where Email = Optional
impl<N> UserBuilder<N, Optional> {
    fn email(self, email: &str) -> UserBuilder<N, Required> {
        UserBuilder {
            name: self.name,
            email: Some(email.to_string()),
            age: self.age,
            _phantom: PhantomData,
        }
    }
}

// .age() is always available — it's optional, so no phantom transition needed
impl<N, E> UserBuilder<N, E> {
    fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }
}

// build() ONLY exists when BOTH required fields are set
// impl<Required, Optional> does NOT have build()
impl UserBuilder<Required, Required> {
    fn build(self) -> User {
        User {
            name: self.name.unwrap(),    // safe: Required means we set it
            email: self.email.unwrap(),  // safe: Required means we set it
            age: self.age,
        }
    }
}
```

Usage:
```rust
// All of these compile ✓ — order doesn't matter
let user = UserBuilder::new().name("Alice").email("a@b.com").build();
let user = UserBuilder::new().email("a@b.com").name("Alice").age(30).build();

// These do NOT compile:
// UserBuilder::new().build();              // missing name and email
// UserBuilder::new().name("Alice").build(); // missing email
```

With const bool generics (alternative approach):
```rust
// Same idea using `const HAS_URL: bool` instead of marker structs
struct HttpRequestBuilder<const HAS_URL: bool, const HAS_METHOD: bool> { ... }

impl HttpRequestBuilder<true, true> {
    fn build(self) -> HttpRequest { ... }  // only when both are true
}
```

## What This Unlocks

- **SDK clients** — AWS SDK, reqwest, and similar libraries use builders so you can't construct a request without required credentials or endpoints.
- **Configuration objects** — distinguish "required config" from "optional tuning" in types; `AppConfig<Initialized>` only exists after all required setup is done.
- **Type-safe DSLs** — query builders, email composers, document constructors where the type system documents and enforces the construction protocol.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Builder state | Phantom type in record `('name, 'email) user_builder` | Phantom type params `UserBuilder<Name, Email>` |
| Field setting | `set_name : (unset, 'e) t -> (set, 'e) t` | `fn name(self, ...) -> UserBuilder<Required, E>` |
| build() gating | `build : (set, set) t -> user` | `impl UserBuilder<Required, Required> { fn build }` |
| Memory overhead | None — phantom types erased | None — `PhantomData` is zero-sized |

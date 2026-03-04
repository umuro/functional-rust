# 396: Simulating Trait Specialization

**Difficulty:** 4  **Level:** Expert

Rust's specialization is unstable — but you can simulate it at compile time using wrapper types and trait dispatch.

## The Problem This Solves

Specialization lets you provide a generic implementation of a trait for all types, then override it with a more efficient (or different) implementation for specific types. C++ has it. Haskell has it via overlapping instances. Rust's RFC 1210 proposed it, but it's been stuck in nightly for years due to soundness concerns around lifetimes.

In practice, the need arises constantly: a `Serialize` trait that uses a fast numeric path for integers but a debug-format fallback for everything else. A `Display` trait that treats `Vec<u8>` as bytes but everything else as debug output. Without specialization, you're forced to either use `Any`/downcasting (dynamic, runtime cost) or write separate concrete impls with no shared interface.

The wrapper-type simulation achieves compile-time dispatch by routing types through different newtype wrappers — each wrapper gets its own trait impl, and the selection happens at the call site. It's more explicit than true specialization, but it's stable, zero-cost, and clearly expresses intent.

## The Intuition

Instead of "implement the trait differently for different types," you implement "a different wrapper type" for each behavior. `Default(x)` gets the generic fallback. `Specialized(x)` gets the specific fast path. The compiler resolves which impl to call at compile time based on the wrapper, with no runtime overhead.

This is the "newtype dispatch" pattern — you move the specialization decision from the impl resolver to the call site.

## How It Works in Rust

```rust
struct Generic<T>(T);
struct Fast<T>(T);

trait Serialize {
    fn serialize(&self) -> String;
}

// Blanket fallback for any Debug type
impl<T: std::fmt::Debug> Serialize for Generic<T> {
    fn serialize(&self) -> String { format!("{:?}", self.0) }
}

// Specific fast path for i32
impl Serialize for Fast<i32> {
    fn serialize(&self) -> String { self.0.to_string() }
}

// At the call site, choose explicitly:
println!("{}", Fast(42i32).serialize());          // "42"
println!("{}", Generic(vec![1, 2, 3]).serialize()); // "[1, 2, 3]"
```

For more automatic dispatch, a marker trait or macro can select the wrapper, but the mechanism is always the same: move the decision to compile time via types, not runtime via `match`.

## What This Unlocks

- **Compile-time performance dispatch** — different code paths for different types with zero runtime cost, all resolved by the monomorphization step
- **Progressive enhancement** — ship a generic impl first, add fast-path specializations for important types later without changing the API
- **Explicit over implicit** — unlike true specialization (where overriding can be surprising), wrapper dispatch is visible at every call site

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Default behavior | Functor with default implementations | Blanket `impl<T: Bound> Trait for Wrapper<T>` |
| Override | Module inclusion + shadow | Concrete `impl Trait for Wrapper<ConcreteType>` |
| Selection | Functor application | Wrapper type at call site (or macro) |
| Compile-time | Yes (monomorphic modules) | Yes (monomorphization) |
| True specialization | N/A in OCaml | Unstable (RFC 1210, nightly only) |

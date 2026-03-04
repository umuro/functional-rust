# 395: Default Methods

**Difficulty:** 2  **Level:** Intermediate

Provide method implementations in a trait so implementors get them free — override only what needs customization.

## The Problem This Solves

Traits are interfaces: they declare what a type can do, but normally leave the "how" entirely to the implementor. Every type must implement every method. This works until your trait grows. Add ten utility methods — `is_empty`, `len`, `contains`, `any`, `all`, `first`, `last` — and every implementor must write them all, usually with the same logic.

This is the interface tax. In Java, it led to `AbstractXxx` base classes — ugly workarounds to share default behavior. In Rust, default methods solve it cleanly: you provide an implementation in the trait definition. Implementors that are happy with the default skip it. Those with better implementations for their specific type can override it. The minimal surface is small; the full API is rich.

The standard library uses this heavily. `Iterator` requires only `next()` — then provides 70+ default methods. `Read` requires only `read()` then provides `read_exact`, `read_to_end`, `bytes`, and more. One method to implement; a full API to use.

## The Intuition

Default methods are fallback implementations living in the trait itself. They're written in terms of the required (abstract) methods — anything the implementor must provide. When you call a default method, it delegates to the required methods, which dispatch to the concrete type's implementation.

It's like a mixin: define the core operations abstractly, then build higher-level operations on top. The higher-level operations work correctly for any type that properly implements the core.

## How It Works in Rust

```rust
use std::fmt;

trait Collection {
    type Item: PartialEq + fmt::Debug + Clone;

    // Required: implementors MUST provide this
    fn items(&self) -> &[Self::Item];

    // Default methods: free for all implementors
    fn is_empty(&self) -> bool { self.items().is_empty() }
    fn len(&self) -> usize { self.items().len() }
    fn contains(&self, item: &Self::Item) -> bool { self.items().contains(item) }
    fn any(&self, predicate: impl Fn(&Self::Item) -> bool) -> bool {
        self.items().iter().any(predicate)
    }
    fn all(&self, predicate: impl Fn(&Self::Item) -> bool) -> bool {
        self.items().iter().all(predicate)
    }
    fn first(&self) -> Option<&Self::Item> { self.items().first() }
}

struct IntVec(Vec<i32>);

// Minimal impl: just the one required method
impl Collection for IntVec {
    type Item = i32;
    fn items(&self) -> &[i32] { &self.0 }
    // is_empty, len, contains, any, all, first — all come for free
}

fn main() {
    let v = IntVec(vec![1, 2, 3, 4, 5]);
    println!("len: {}", v.len());          // default method
    println!("contains 3: {}", v.contains(&3)); // default method
    println!("any > 4: {}", v.any(|x| *x > 4)); // default method
}
```

Override a default when you have a more efficient implementation:
```rust
struct SortedVec(Vec<i32>);

impl Collection for SortedVec {
    type Item = i32;
    fn items(&self) -> &[i32] { &self.0 }

    // Binary search is O(log n) vs default's O(n) linear scan
    fn contains(&self, item: &i32) -> bool {
        self.0.binary_search(item).is_ok()
    }
}
```

## What This Unlocks

- **Implement once, use everywhere** — `Iterator::next()` is all you write; `.map()`, `.filter()`, `.fold()`, `.zip()`, and 65 others come for free.
- **API evolution without breaking changes** — add new default methods to a published trait; existing implementors continue to compile and automatically gain the new behavior.
- **Focused required surface** — types only need to implement the core semantic operation; boilerplate utilities are inherited.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Default methods | Functor `MakeCollection` fills in defaults from base operations | Default methods directly in `trait` body — no extra module indirection |
| Required vs default | Functor parameter must supply all base operations | Methods without `{}` body are required; those with `{}` are optional to override |
| Override | Functor always uses provided base — no override | Implementor can shadow any default with a more efficient version |
| Tooling | Each functor application creates a new module | Single trait; auto-complete shows all methods as available |

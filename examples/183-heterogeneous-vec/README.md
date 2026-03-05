# 183: Heterogeneous Vec with Safe Downcast

**Difficulty:** 4  **Level:** Advanced

Store values of different types in one `Vec` and retrieve them by their original type — safely, without crashes.

## The Problem This Solves

Sometimes you need a collection that holds values of genuinely different types: a `Vec` that contains both `i64` and `String` and `bool`. An enum works when the set of types is fixed and known at compile time. But what if you want an open-ended collection that accepts any type at insertion time and returns it at retrieval time?

The `Any` trait provides runtime type identity. You can store `Box<dyn Any>` and later ask "is this an `i64`?" via `downcast_ref::<i64>()`. It returns `None` on type mismatch — safe, no panics, no undefined behavior.

This is the Rust equivalent of OCaml's GADT type witnesses: a mechanism for type-safe dynamic dispatch where the concrete type is recovered at runtime rather than enforced at compile time.

## The Intuition

Every Rust type has a `TypeId` — a unique identifier. The `Any` trait enables two operations: get the `TypeId` of a value, and attempt a downcast to a concrete type. `downcast_ref::<T>()` checks the stored `TypeId` against `TypeId::of::<T>()` and returns `Some(&T)` if they match.

This is different from type erasure: with `Box<dyn Trait>`, you can call trait methods but can't recover the type. With `Box<dyn Any>`, you don't have trait methods, but you *can* recover the type.

Combine both via a custom `AnyDisplay` supertrait if you want both display capabilities and downcasting.

## How It Works in Rust

```rust
use std::any::Any;

struct HeteroVec {
    items: Vec<Box<dyn Any>>,
}

impl HeteroVec {
    fn push<T: 'static>(&mut self, val: T) {
        self.items.push(Box::new(val));  // type is erased to Box<dyn Any>
    }

    fn get<T: 'static>(&self, index: usize) -> Option<&T> {
        // downcast_ref checks the TypeId and returns Some if it matches
        self.items.get(index)?.downcast_ref::<T>()
    }
}

let mut hv = HeteroVec::new();
hv.push(42i64);
hv.push(String::from("hello"));
hv.push(true);

hv.get::<i64>(0)    // Some(&42)
hv.get::<i64>(1)    // None — position 1 holds a String, not i64
hv.get::<String>(1) // Some(&"hello")
```

Combine `Any` with `Display` for both downcasting and formatted output:

```rust
trait AnyDisplay: Any + std::fmt::Display {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any + std::fmt::Display> AnyDisplay for T {
    fn as_any(&self) -> &dyn Any { self }
}

struct DisplayVec {
    items: Vec<Box<dyn AnyDisplay>>,
}

impl DisplayVec {
    fn display_all(&self) -> Vec<String> {
        self.items.iter().map(|x| format!("{}", x)).collect()
    }

    fn get<T: 'static>(&self, i: usize) -> Option<&T> {
        self.items.get(i)?.as_any().downcast_ref::<T>()
    }
}
```

When the type set is closed, prefer enum — it's cleaner and faster:

```rust
#[derive(Debug)]
enum Value { Int(i64), Str(String), Bool(bool), Float(f64) }

// Exhaustive pattern matching at extraction — no runtime TypeId lookup
let vals = vec![Value::Int(1), Value::Str("x".into()), Value::Bool(false)];
```

## What This Unlocks

- **Dynamic registries** — store handlers/plugins of arbitrary types, retrieve by type key
- **Scripting interop** — bridge between a typed Rust core and a dynamic scripting layer
- **Test harnesses** — collect test results of different types in a single runner output

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type witness | GADT: `type _ ty = TInt : int ty | TStr : string ty` | `TypeId` via `Any` trait |
| Type-safe retrieval | Pattern match on GADT witness refines type | `downcast_ref::<T>()` — returns `Option<&T>` |
| Open vs closed | GADT is closed (fixed constructors) | `Box<dyn Any>` is open (any `'static` type) |
| Combined capabilities | First-class module with witness + operations | Custom supertrait: `trait AnyDisplay: Any + Display` |
| Enum alternative | Polymorphic variant / ADT | `enum Value { Int(i64), Str(String), ... }` |

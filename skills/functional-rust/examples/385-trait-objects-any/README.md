# 385: Any Trait for Runtime Type Info

**Difficulty:** 3  **Level:** Advanced

Type-erased values with runtime type recovery via `Box<dyn Any>` and `.downcast::<T>()`.

## The Problem This Solves

`dyn Trait` lets you erase a type behind a trait interface, but you can only call the methods the trait defines. Sometimes you need to go further: store completely arbitrary values of unknown type, then recover the original type at runtime. Think of a heterogeneous map (`HashMap<String, Box<dyn Any>>`), a dependency injection container, an event system where each event type is different, or a scripting language's value type.

This is what Java's `Object` does — everything can be cast to `Object` and downcast back. Python and JavaScript are dynamically typed and do this implicitly. In Rust, `std::any::Any` provides this capability explicitly, with a runtime check that gives you `None` instead of a panic if the type doesn't match.

## The Intuition

`std::any::Any` is a trait with one method: `type_id(&self) -> TypeId`. Every `'static` type automatically implements it. When you have a `Box<dyn Any>`, the concrete type is erased — but the vtable contains the `TypeId` of the original type.

Calling `.downcast::<T>()` checks: "is the stored `TypeId` equal to `TypeId::of::<T>()`?" If yes, it reinterprets the raw pointer as `Box<T>` and returns `Ok(Box<T>)`. If no, it returns `Err(Box<dyn Any>)` — your original value, unchanged, so you haven't lost it.

The `'static` bound is important: `Box<dyn Any>` requires `T: 'static`. You can't store a borrowed value, because the type ID system can't reason about lifetimes.

## How It Works in Rust

```rust
use std::any::Any;

// Heterogeneous storage
let mut bag: Vec<Box<dyn Any>> = Vec::new();
bag.push(Box::new(42i32));
bag.push(Box::new("hello".to_string()));
bag.push(Box::new(3.14f64));

// Runtime type recovery
for item in &bag {
    if let Some(n) = item.downcast_ref::<i32>() {
        println!("i32: {}", n);
    } else if let Some(s) = item.downcast_ref::<String>() {
        println!("String: {}", s);
    } else if let Some(f) = item.downcast_ref::<f64>() {
        println!("f64: {}", f);
    }
}

// Consuming downcast (Box<dyn Any> → Box<T>)
let owned: Box<dyn Any> = Box::new(100i32);
match owned.downcast::<i32>() {
    Ok(n)   => println!("Got i32: {}", n),
    Err(v)  => println!("Wrong type: {:?}", v.type_id()),
}
```

For typed maps (`HashMap<String, Box<dyn Any>>`), the `anymap` or `typemap` crates give you type-safe heterogeneous maps keyed by type.

## What This Unlocks

- **Plugin architectures** — pass context objects between components without knowing concrete types.
- **Dependency injection** — store services by type, retrieve by type at runtime.
- **Dynamic configuration** — parse heterogeneous config values and recover types on use.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Dynamic typing | `Obj.t` (runtime value wrapper) | `Box<dyn Any>` |
| Type-safe downcast | Not idiomatic — avoid `Obj` in user code | `.downcast_ref::<T>()` → `Option<&T>` |
| Type identity | Not a concept | `std::any::TypeId` — unique per type |
| Heterogeneous map | `Hashtbl.t` with `Obj.t` values | `HashMap<String, Box<dyn Any>>` or `anymap` |

📖 **[View on hightechmind.io →](https://hightechmind.io/rust/385-trait-objects-any)**

---

# 385: Trait Objects and `Any` (Runtime Type Information)

## Problem Statement

Statically typed languages normally cannot store values of unknown type and later recover their original type. Rust's `std::any::Any` trait provides controlled runtime type information: any `'static` type automatically implements `Any`, and `dyn Any` enables downcasting back to the original type with `downcast_ref::<T>()`. This enables heterogeneous containers, scripting engine value types, dependency injection containers, and event systems where the payload type varies.

`Any` and `TypeId` appear in the `anymap` crate, Bevy's ECS component storage, `actix`'s actor message system, and any system requiring type-erased storage with safe recovery.

## Learning Outcomes

- Understand how `std::any::Any` provides runtime type information in a statically typed language
- Learn how `TypeId::of::<T>()` creates a unique type identity usable as a map key
- See how `downcast_ref::<T>()` safely recovers the concrete type from `dyn Any`
- Understand the `'static` bound requirement on `Any` and why lifetime-bearing types cannot implement it
- Learn the type-safe heterogeneous map pattern using `HashMap<TypeId, Box<dyn Any>>`

## Rust Application

In `src/lib.rs`, the `describe` function takes `&dyn Any` and uses `downcast_ref::<T>()` in a chain of if-let expressions to pattern-match on the runtime type. The `TypeMap` struct stores one value per type using `TypeId::of::<T>()` as the key. The `insert<T: Any>` method type-erases to `Box<dyn Any>`, and `get<T: Any>` recovers the value with `downcast_ref::<T>()`. This is safe because the `TypeId` key guarantees the stored value is exactly type `T`.

## OCaml Approach

OCaml handles heterogeneous storage through existential types (first-class modules with `type t`) or the `Obj` module for unsafe runtime values. A type-safe heterogeneous map in OCaml uses GADT witnesses: `type 'a key = Key : TypeId * ('a -> int) -> 'a key`. The `hmap` library provides this. Unlike Rust's explicit `TypeId`, OCaml hides type representations behind the GC.

## Key Differences

1. **Downcast safety**: Rust's `downcast_ref` returns `Option<&T>` — explicit failure handling; OCaml's `Obj.magic` is unsafe and unchecked.
2. **Type identity**: Rust uses `TypeId` (opaque hash of type); OCaml uses GADT witnesses or `Hashtbl` with type-indexed keys from libraries.
3. **Lifetime restriction**: Rust's `Any` requires `T: 'static`; OCaml has no equivalent restriction since the GC manages all lifetimes.
4. **Ergonomics**: Rust requires explicit `downcast_ref::<T>()` at every use site; OCaml's GADT approach can sometimes infer the type from context.

## Exercises

1. **Event bus**: Build a type-safe event bus using `TypeId` keys and `Vec<Box<dyn Any>>` values. Implement `publish<E: Any>(event: E)` and `subscribe<E: Any, F: Fn(&E)>(handler: F)`, dispatching stored events to matching handlers.
2. **Typed arena**: Create an arena allocator that stores values of any type and retrieves them by a typed handle. The handle encodes the type at compile time, so `get::<String>(handle)` only compiles if the handle was created with a `String`.
3. **Dynamic dispatch benchmark**: Compare three approaches for a heterogeneous collection: `Vec<Box<dyn Any>>` with downcasting, `Vec<Box<dyn Trait>>` with vtables, and an enum-based closed set. Measure access time for 1 million elements.

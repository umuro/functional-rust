📖 **[View on hightechmind.io →](https://hightechmind.io/rust/147-type-erasure)**

---

# Type Erasure

## Problem Statement

Type erasure deliberately discards type information to enable heterogeneous collections, plugin systems, and dynamic dispatch. Storing values of different concrete types together requires erasing their specific types and retaining only a common interface. This is the mechanism behind Java's generics, Rust's `Box<dyn Trait>`, OCaml's existential types, and Go's interface values. The cost is one pointer indirection per method call; the benefit is open extension and runtime flexibility.

## Learning Outcomes

- Understand what type erasure means and when it is the right tool
- Learn Rust's primary erasure mechanisms: `Box<dyn Trait>`, `Arc<dyn Trait>`, and `Box<dyn Any>`
- See how `Any + downcast_ref` provides safe type recovery after erasure
- Compare Rust's explicit erasure with Java's implicit erasure and OCaml's approach

## Rust Application

`Box<dyn Trait>` erases the concrete type behind a vtable. `Box<dyn Any>` erases everything except the ability to downcast: `any.downcast_ref::<ConcreteType>()` returns `Option<&ConcreteType>`, safely recovering the concrete type at runtime. `TypeId::of::<T>()` provides the runtime type identity that makes downcasting safe. A plugin system stores `Box<dyn Plugin>` and calls methods polymorphically without knowing the concrete plugin type.

## OCaml Approach

OCaml erases types through:
1. GADT existentials: `type any = Any : 'a -> any` packs any value with its type erased
2. First-class modules: `(module M : INTERFACE)` erases the module identity
3. `Obj.magic` (unsafe, equivalent to Rust's `transmute`) — never use in production

OCaml's GC means erased values are always safely accessible; Rust's ownership system requires `Box` or `Arc` to heap-allocate erased values.

## Key Differences

1. **Explicit vs. implicit**: Rust requires explicit `Box<dyn Trait>` to erase types; Java erases generics silently at compile time (unsound without reification).
2. **Safe downcast**: Rust's `downcast_ref::<T>()` checks `TypeId` at runtime; OCaml's GADT pattern match handles this structurally without runtime type IDs.
3. **Method availability**: Rust's vtable only exposes the trait's methods; `Box<dyn Any>` exposes just `downcast`; OCaml's first-class modules expose whatever the module signature declares.
4. **Ownership**: Rust's `Box<dyn Any>` has unique ownership; `Arc<dyn Any>` enables shared ownership across threads; OCaml's GC handles all sharing automatically.

## Exercises

1. Build a `TypeMap` that stores at most one value per type: `insert<T: 'static>(&mut self, val: T)` and `get<T: 'static>(&self) -> Option<&T>`.
2. Implement a plugin system with `trait Plugin { fn name(&self) -> &str; fn run(&self); }` and a registry that stores `Vec<Box<dyn Plugin>>`.
3. Write `fn erase_and_recover<T: Any>(val: T) -> Option<T>` that boxes a value as `Box<dyn Any>` and then recovers it via downcast.

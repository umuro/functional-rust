# 260: Functor Comparable Set

**Difficulty:** 3  **Level:** Advanced

Build a generic ordered set for any comparable type — OCaml's `Set.Make` functor translated to Rust generics.

## The Problem This Solves

OCaml's standard library doesn't have a single polymorphic set type. Instead, you use a *functor*: `Set.Make(Int)` produces an `IntSet` module; `Set.Make(String)` produces a `StringSet` module. Each is a separate type with its own `empty`, `add`, `mem`, and `to_list`. The functor takes a module that provides a comparison function and returns a full set implementation.

This is a design choice: the functor captures comparison at the type level, enabling the compiler to verify that elements are comparable before any runtime code runs. The trade-off is that you can't write a function that works on any set — you'd need a functor parameter.

In Rust, generic structs with trait bounds achieve the same guarantee at compile time. `ComparableSet<T: Ord>` works for any type that implements `Ord` — integers, strings, custom structs with `#[derive(Ord)]` — without creating separate types. Monomorphisation generates specialized code per type, identical to what OCaml's functor produces.

## The Intuition

A comparable set is a sorted, deduplicated collection. "Comparable" means elements have a total ordering — you can say whether any element is less than, equal to, or greater than any other. Without that guarantee, you can't maintain sorted order.

OCaml enforces this via the `COMPARABLE` module type: the functor refuses to compile if the argument module doesn't provide `compare`. Rust enforces it via the `Ord` trait bound: the compiler refuses to instantiate `ComparableSet<T>` if `T` doesn't implement `Ord`.

The sorted `Vec` representation gives O(log n) membership via `binary_search` — better than OCaml's list-based version which scans linearly. Insert maintains sorted order by finding the correct position before inserting.

## How It Works in Rust

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparableSet<T: Ord> {
    items: Vec<T>,  // always sorted, always deduplicated
}

impl<T: Ord> ComparableSet<T> {
    pub fn new() -> Self { ComparableSet { items: Vec::new() } }

    // O(log n) membership test via binary search
    pub fn contains(&self, x: &T) -> bool {
        self.items.binary_search(x).is_ok()
    }

    // Consume self, return new set with x inserted (or unchanged if duplicate)
    #[must_use]
    pub fn insert(mut self, x: T) -> Self {
        match self.items.binary_search(&x) {
            Ok(_) => self,            // already present — deduplicate
            Err(pos) => {
                self.items.insert(pos, x);  // insert at sorted position
                self
            }
        }
    }

    pub fn to_list(&self) -> &[T] { &self.items }
}

// Usage — works for any Ord type, no module boilerplate
let int_set = ComparableSet::new().insert(3).insert(1).insert(4).insert(1).insert(5);
// → [1, 3, 4, 5]  (sorted, deduplicated)

let str_set = ComparableSet::new().insert("fox").insert("cat").insert("ant");
// → ["ant", "cat", "fox"]
```

The builder pattern (`insert` consumes `self` and returns `Self`) mirrors OCaml's immutable-update semantics while staying idiomatic Rust.

## What This Unlocks

- **Type-safe generic collections** — one `ComparableSet<T>` works for any ordered type; no code duplication across element types.
- **Understanding OCaml functors** — this example makes the OCaml functor pattern concrete: trait bounds are the Rust equivalent of `COMPARABLE` module types.
- **Sorted containers** — the sorted-Vec pattern (binary_search + insert at position) applies to priority queues, deduplication, and ordered caches.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Abstraction mechanism | Functor: `MakeSet(Int)` → new module | Generic: `ComparableSet<i32>` — monomorphised |
| Comparable constraint | Module type `COMPARABLE` with `compare` | `Ord` trait — already implemented by primitives |
| Membership test | `List.exists` O(n) (OCaml list version) | `binary_search` O(log n) |
| Insert | Returns new value (functional) | Consumes `self`, returns `Self` |
| Separate types | `IntSet` ≠ `StringSet` — distinct modules | Same type `ComparableSet<T>` — different monomorphisations |

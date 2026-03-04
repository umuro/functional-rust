# 143: Associated Type Bounds

**Difficulty:** 4  **Level:** Advanced

Traits with associated types that carry their own bounds — so generic code can reason about `Key` and `Value` without knowing their concrete types.

## The Problem This Solves

Traits with associated types express "this trait has a related type, and different implementors will choose different concrete types." A `Collection` might say "I have a `Key` type." But generic code that works with any `Collection` needs to *do something* with keys — sort them, print them, compare them. Without bounds on the associated type, none of that is possible.

OCaml solves this with module type sharing constraints: `with type key = int` or `with type key = string`. Rust's equivalent is putting trait bounds directly on the associated type: `type Key: Ord + Clone + Display`. Now any generic function that accepts `impl Collection` can call `key1.cmp(&key2)`, `format!("{}", key)`, or `key.clone()` — because the trait guarantees these capabilities regardless of which concrete `Key` type is used.

This pattern appears everywhere in real Rust: `std::collections::HashMap`'s key type must be `Hash + Eq`, `BTreeMap`'s must be `Ord`, iterators' `Item` can carry bounds. It's the mechanism that makes generic data structure APIs possible without sacrificing type safety.

## The Intuition

Put bounds on a trait's associated type (`type Key: Ord + Clone`) so generic functions working with that trait can use those capabilities without knowing the concrete type.

## How It Works in Rust

```rust
use std::fmt::{Debug, Display};

pub trait Collection: Sized {
    // Associated types with bounds — any implementor must satisfy these
    type Key: Ord + Clone + Display + Debug;
    type Value: Clone + Debug;

    fn empty() -> Self;
    fn insert(self, key: Self::Key, value: Self::Value) -> Self;
    fn find(&self, key: &Self::Key) -> Option<&Self::Value>;
    fn sorted_keys(&self) -> Vec<Self::Key>;  // possible because Key: Ord
}

// Generic function: works with any Collection
// Can call .sorted_keys() because Key: Ord, format!("{}", key) because Key: Display
fn print_sorted_keys<C: Collection>(coll: &C) {
    let keys = coll.sorted_keys();
    for k in &keys {
        print!("{} ", k);  // works because Key: Display
    }
}

// Another generic function — uses the Value bound
fn lookup_display<C: Collection>(coll: &C, key: &C::Key)
where
    C::Value: Display,  // extra bound on Value for this specific function
{
    match coll.find(key) {
        Some(v) => println!("find({}) = {}", key, v),
        None    => println!("find({}) = not found", key),
    }
}

// Two different implementations with different Key types:
// IntMap<V>:    Key = i32     (implements Collection)
// StringMap<V>: Key = String  (implements Collection)
// Both work with print_sorted_keys — the bounds are what matters, not the type.

let m = IntMap::<&str>::empty().insert(3, "three").insert(1, "one");
print_sorted_keys(&m);   // prints "1 3" — sorted via Ord on i32

let sm = StringMap::<i32>::empty().insert("banana".into(), 3).insert("apple".into(), 1);
print_sorted_keys(&sm);  // prints "apple banana" — sorted via Ord on String
```

## What This Unlocks

- **Parameterized data structure abstractions** — a `Collection` trait that works for any ordered key type, without duplicating code.
- **Generic algorithms on custom containers** — sort, print, merge, diff: any operation that needs `Ord`, `Display`, or `Clone` on keys works once, for all implementors.
- **OCaml module type analogues** — the OCaml `with type key = ...` constraint maps directly to this pattern.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Constraining associated types | `with type key = int` sharing constraint | `type Key: Ord + Clone + Display` bound |
| Generic code over any collection | Functor over module type | Generic fn `<C: Collection>` |
| Multiple key types | Separate modules or functors | Different `impl Collection` blocks |
| Adding extra bounds per function | Type annotation in signature | `where C::Value: Display` clause |

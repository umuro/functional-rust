# 064: Map.Make Functor — String→Int Dictionary

**Difficulty:** ⭐⭐  **Level:** Intermediate

How OCaml's module-level "functors" generate type-safe dictionary types — and what Rust's generics achieve instead.

## The Problem This Solves

You want a dictionary (key → value map) where keys are strings and values are integers. Simple. But then you need another map where keys must be sorted differently, or keys are a custom type. In many languages you end up copy-pasting your map implementation and tweaking it.

OCaml solved this at the *module* level: `Map.Make(String)` is a function that takes a module (describing your key type and how to compare it) and produces an entire, specialized, type-safe `Map` module. This is OCaml's "functor" — a function from modules to modules. It's a different use of the word "functor" than the `Option::map` kind.

In Rust, this same problem is solved by **generics with trait bounds**:

```rust
// OCaml: module StringMap = Map.Make(String)
// Rust:  BTreeMap<String, V>  — the key type is a generic parameter
```

Rust's generics let you say "this collection works for any key type `K` that implements `Ord`" — which covers the same ground as OCaml's `Map.Make`, just with different syntax. The concept exists because "I need a sorted, type-safe dictionary parameterized by key type" is a fundamental need that both languages must address.

## The Intuition

OCaml's `Map.Make` works like a cookie cutter factory. You hand it a cookie-cutter shape (a module that says "here's my key type and how to compare two keys"), and it stamps out a complete, ready-to-use map module tailored to that shape. Every cookie (map operation) it produces is type-safe for that exact key type.

Rust achieves the same result differently: instead of generating a specialized module, it uses a single generic `BTreeMap<K, V>` that works for *any* `K: Ord`. The compiler specializes it at compile time for each concrete key type you use — effectively doing what `Map.Make` does, just implicitly.

The end result is the same: a sorted, type-safe, immutable-friendly map. The mechanism differs:
- OCaml: explicit module-level parameterization (the "functor" pattern)
- Rust: generic type parameters with trait bounds

**One important note on terminology:** In this example, "functor" means OCaml's module-level parameterization (`Map.Make`), *not* the `Option::map` style functor from examples 051–052. OCaml overloads the word. Rust sidesteps this: generics handle both jobs.

## How It Works in Rust

**Step 1 — Build a word-length dictionary:**

```rust
use std::collections::BTreeMap;

// BTreeMap = sorted by key (like OCaml's Map.Make)
// HashMap  = faster lookup, but unordered
fn word_lengths(words: &[&str]) -> BTreeMap<String, usize> {
    words.iter()
        .map(|w| (w.to_string(), w.len()))
        .collect()   // iterator of (key, value) pairs → BTreeMap
}
```

**Step 2 — The map operations OCaml gives you for free:**

```rust
// Filter by value (like OCaml's Map.filter)
fn filter_by_value<K: Ord + Clone, V: Clone>(
    map: &BTreeMap<K, V>,
    pred: impl Fn(&V) -> bool,
) -> BTreeMap<K, V> {
    map.iter()
        .filter(|(_, v)| pred(v))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

// Transform values (like OCaml's Map.map — a functor-style operation!)
fn map_values<K: Ord + Clone, V, U>(
    map: &BTreeMap<K, V>,
    f: impl Fn(&V) -> U,
) -> BTreeMap<K, U> {
    map.iter()
        .map(|(k, v)| (k.clone(), f(v)))
        .collect()
}
```

Notice that `map_values` *is* a functor-style operation: it transforms the values inside the map without changing the keys or structure. The map is a container; `map_values` maps over its contents.

**Step 3 — BTreeMap preserves insertion-independent ordering:**

```rust
let words = vec!["zebra", "apple", "mango"];
let m = word_lengths(&words);
let keys: Vec<_> = m.keys().collect();
assert_eq!(keys, vec!["apple", "mango", "zebra"]); // always sorted
```

OCaml's `Map.Make` produces a balanced BST with the same property. Rust's `BTreeMap` is a B-tree with the same guarantee.

## What This Unlocks

- **Type-safe, sorted dictionaries with zero boilerplate.** `BTreeMap<String, V>` handles any value type; the compiler enforces that keys implement `Ord`.
- **Composable map transformations.** `map_values`, `filter_by_value`, and similar functions compose over any `BTreeMap<K, V>` — generic code over all dictionary types at once.
- **Ordered iteration for free.** Unlike `HashMap`, `BTreeMap` always iterates in key order — useful for rendering sorted output, binary searching, range queries.

Real codebases where this pattern appears: `serde` (key ordering in JSON serialization), `cargo` (dependency resolution uses sorted maps), language servers (symbol tables keyed by name), and any code that needs deterministic iteration order.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Creating a typed map module | `module StringMap = Map.Make(String)` — module-level functor | `BTreeMap<String, V>` — generic type parameter |
| What "functor" means here | A function from modules to modules (parameterized module) | Not called a functor — it's just generics |
| Key ordering requirement | Provided by the module argument (must have `compare`) | `K: Ord` trait bound |
| Sorted map type | `Map.Make(String).t` | `BTreeMap<K, V>` |
| Unordered/hash map | `Hashtbl` (mutable) | `HashMap<K, V>` |
| Immutability | Maps are persistent/immutable by default | Controlled by `mut` keyword; maps are mutable by default |
| `map` over values | `StringMap.map f m` | `map_values(&m, f)` (not built-in, but trivial via iterators) |
| Lookup | `StringMap.find_opt key m` → `option` | `m.get(key)` → `Option<&V>` |

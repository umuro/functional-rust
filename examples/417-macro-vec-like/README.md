📖 **[View on hightechmind.io →](https://hightechmind.io/rust/417-macro-vec-like)**

---

# 417: Vec-like Collection Macros
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

The `vec![1, 2, 3]` literal macro is so convenient that the absence of equivalent literals for `HashSet`, `BTreeSet`, and `HashMap` is a constant friction point. Initializing these collections requires `let mut s = HashSet::new(); s.insert(1); s.insert(2);` — three lines per item. Collection literal macros (`set!`, `map!`, `btree_set!`) bring the same ergonomics to all standard collections, making initialization as concise as `vec!` while maintaining type safety and supporting trailing commas.

Collection literal macros are so commonly needed that third-party crates like `maplit` and `im` provide them, and they're one of the most commonly written first macros.

## Learning Outcomes

- Understand how `vec!` works internally and how to replicate the pattern for other collections
- Learn how `::std::collections::HashSet::new()` fully-qualified paths prevent name resolution issues in macros
- See how trailing comma support (`$(,)?`) improves ergonomic macro usage
- Understand how to handle both empty and non-empty collection initialization
- Learn how collection literal macros compose with type inference

## Rust Application

In `src/lib.rs`, `set!()` handles the empty case returning `HashSet::new()`. `set!($($elem:expr),+ $(,)?)` creates a set, inserts each element in a repetition block, and returns it. The block `{{ ... }}` creates a temporary scope, and the last expression is the result. `map!` uses `$key:expr => $val:expr` pairs. The `::std::` prefix ensures macros work when `use std::collections::*` is not in scope.

## OCaml Approach

OCaml uses module functions for collection creation: `let s = List.fold_left (fun acc x -> Set.add x acc) Set.empty [1; 2; 3]`. `Base.Set.of_list` and `Base.Map.of_alist_exn` provide one-liner initialization. OCaml's list literal syntax `[1; 2; 3]` is built-in and convenient; other collections require explicit construction. No macro infrastructure is needed since the module functions are expressive enough.

## Key Differences

1. **Built-in vs. macro**: OCaml's list literal is syntax; Rust's `vec!` and equivalent macros are library code that expands to `push` calls.
2. **Empty case**: Rust macros must handle `set!()` explicitly (no elements means no insert calls); OCaml's `Set.empty` is a value.
3. **Type inference**: Rust's collection macros infer element types from the provided values; OCaml's typed modules require the element type to be known from the module.
4. **Homogeneity**: Both Rust and OCaml collection literals require all elements to have the same type; Rust enforces this via the type system, OCaml via the module's type parameter.

## Exercises

1. **Ordered map**: Implement `omap!{ key => val, ... }` creating a `BTreeMap` (ordered map). Verify that iteration produces keys in sorted order.
2. **Default dict**: Implement `default_map!{ key => val, ... ; default: expr }` that creates a `HashMap` with a default value, returning the default for missing keys via a `get_or_default(key)` method on a wrapper.
3. **Multimap**: Implement `multimap!{ key1 => [v1, v2], key2 => [v3] }` creating a `HashMap<K, Vec<V>>` where multiple values per key are supported via list syntax.

📖 **[View on hightechmind.io →](https://hightechmind.io/rust/406-hash-eq-ord-traits)**

---

# 406: Hash, Eq, and Ord Traits
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

To use a type as a `HashMap` key or in a `HashSet`, it needs `Eq + Hash`. To use it in a `BTreeMap` or sorted collection, it needs `Ord`. These traits form a coherence hierarchy: `Eq: PartialEq`, `Ord: Eq + PartialOrd`. Implementing them incorrectly — hashing a value differently than how it compares for equality — leads to incorrect collection behavior and subtle bugs (items can be inserted but never found). Rust's type system and derive macros make the common case correct, but custom implementations require careful adherence to the mathematical laws.

These traits underpin every collection in `std`: `HashMap`, `HashSet`, `BTreeMap`, `BTreeSet`, and sorting via `sort_by`.

## Learning Outcomes

- Understand the trait hierarchy: `PartialEq → Eq`, `PartialOrd → Ord`, and the `Hash` requirement for hash maps
- Learn the critical law: if `a == b` then `hash(a) == hash(b)` (but not vice versa)
- See how `#[derive(PartialEq, Eq, Hash)]` satisfies the laws automatically for structs
- Understand how to implement custom `Ord` for domain-specific ordering (e.g., priority levels)
- Learn how `BTreeMap` (sorted) and `HashMap` (hashed) have different key requirements

## Rust Application

In `src/lib.rs`, `Point` derives all comparison and hash traits for free — the derive macro generates implementations that satisfy all laws. `Priority` implements custom `Ord` via `value()` mapping to `u8`, delegating to `u8`'s comparison. The `partial_cmp` implementation delegates to `cmp` (a common pattern when `PartialOrd` and `Ord` agree). `HashMap<Point, String>` and `BTreeSet<Priority>` demonstrate the traits in action.

## OCaml Approach

OCaml uses the polymorphic `compare : 'a -> 'a -> int` for structural comparison and `Hashtbl.hash : 'a -> int` for hashing. Custom types get comparison for free since OCaml's comparison is structural by default. `Map.Make(Ord)` and `Set.Make(Ord)` create sorted collections using a provided comparator module. The `ppx_compare` and `ppx_hash` derivers generate type-specific compare/hash functions that OCaml's standard library uses.

## Key Differences

1. **Structural comparison**: OCaml's `compare` works structurally on any type without annotation; Rust requires explicit derives or implementations.
2. **Hash coherence**: Rust's type system doesn't enforce the `Eq → Hash` law — incorrect impls compile but cause runtime bugs; OCaml has the same risk.
3. **Collection requirements**: Rust's `HashMap` requires `Eq + Hash`; OCaml's `Hashtbl` uses polymorphic hash by default, requiring no annotation.
4. **Ordered collections**: Rust's `BTreeMap` requires `Ord`; OCaml's `Map.Make` takes an `Ord` module parameter at compile time.

## Exercises

1. **Case-insensitive key**: Create `CaseInsensitiveStr(String)` implementing `Eq` and `Hash` based on the lowercase version. Use it as a `HashMap` key and verify that `"Foo"` and `"foo"` resolve to the same entry.
2. **Ranked task**: Implement a `Task { priority: Priority, name: String, id: u64 }` with `Ord` that orders by priority (descending), then name (alphabetical), then id. Use it in a `BTreeSet` to maintain a sorted task queue.
3. **Law verification**: Write property-based tests (or manual tests) that verify the `Hash` coherence law for your custom type: generate pairs of equal values and assert their hashes are equal, and generate unequal values to estimate the hash distribution.

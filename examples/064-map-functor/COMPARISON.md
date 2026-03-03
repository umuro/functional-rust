# Map.Make Functor — String→Int Dictionary: OCaml vs Rust

## The Core Insight
Both languages provide immutable, ordered dictionary types backed by balanced trees. The fascinating difference is how they're parameterized: OCaml uses functors (functions from modules to modules) while Rust uses generics with trait bounds. Same goal, fundamentally different abstraction mechanisms.

## OCaml Approach
`Map.Make(String)` is a functor application — it takes the `String` module (which satisfies the `OrderedType` signature with a `compare` function) and produces a new module `StringMap` with all map operations specialized to string keys. The resulting map is an immutable balanced BST. Operations like `add`, `find_opt`, `filter`, and `map` all return new maps without mutating the original. This functor pattern is central to OCaml's module system.

## Rust Approach
Rust's `BTreeMap<K, V>` is a generic type parameterized by key and value types. The key must implement `Ord` (for ordering) — this is checked at compile time via trait bounds. Unlike OCaml's functor, you don't create a specialized module; you just use the generic type directly. Rust also offers `HashMap<K, V>` for O(1) average-case lookups (requires `Hash + Eq`). Iterator adapters (`filter`, `map`, `collect`) provide the functional transformation style.

## Side-by-Side
| Concept | OCaml | Rust |
|---------|-------|------|
| Ordered map | `Map.Make(String)` functor | `BTreeMap<String, V>` generic |
| Hash map | Third-party / Hashtbl (mutable) | `HashMap<K, V>` (in std) |
| Key constraint | `OrderedType` signature | `Ord` trait bound |
| Parameterization | Functor (module → module) | Generics (type → type) |
| Immutability | All operations return new map | Methods take `&self`, return new collections |
| Lookup | `find_opt : key -> 'a t -> 'a option` | `.get(&key) -> Option<&V>` |

## What Rust Learners Should Notice
- `BTreeMap` is Rust's closest equivalent to OCaml's `Map` — both are balanced BSTs with O(log n) operations
- Rust's `.collect()` is incredibly powerful — it can build any collection from an iterator, guided by type inference
- OCaml functors create new modules at compile time; Rust generics monomorphize at compile time — both are zero-cost
- `HashMap` is usually preferred in Rust for performance unless you need ordered iteration
- Rust maps own their keys and values — `.get()` returns `Option<&V>` (a borrow), not a copy

## Further Reading
- [The Rust Book — Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [Rust std::collections::BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
- [OCaml Map module](https://ocaml.org/docs/maps)

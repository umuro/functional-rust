# Word Count with Map: OCaml vs Rust

## The Core Insight
Word counting combines string processing with map operations — a practical pattern that reveals how each language handles mutable vs immutable data structures. OCaml builds a new map on every insert; Rust mutates a HashMap in place. Both are correct; the performance characteristics differ significantly.

## OCaml Approach
OCaml uses `StringMap` (from `Map.Make(String)`) — an immutable balanced BST. Each `StringMap.add` creates a new map sharing most of its structure with the old one. The fold accumulates by threading the map through each word. String tokenization uses a mutable `Buffer` and `ref` — one of the few places where mutation is pragmatic in OCaml. `Option.value ~default:0` handles the missing-key case.

## Rust Approach
Rust's `HashMap` is a mutable hash table with O(1) amortized insert/lookup. The `entry` API (`map.entry(word).or_insert(0)`) is a Rust-specific ergonomic pattern that handles both insert and update in one call. The fold variant passes `mut map` through the closure. `BTreeMap` is available when ordered output is needed (like OCaml's `Map`). Tokenization uses iterators and `String::push` for building words.

## Side-by-Side
| Concept | OCaml | Rust |
|---------|-------|------|
| Map type | `StringMap` (immutable BST) | `HashMap` (mutable hash table) |
| Insert | `StringMap.add k v m` → new map | `map.entry(k).or_insert(0) += 1` |
| Lookup | `StringMap.find_opt k m` | `map.get(&k)` |
| Ordered | Yes (BST) | `BTreeMap` for ordered, `HashMap` for fast |
| Tokenization | `Buffer` + `ref` (mutable) | `String::push` + `Vec` |
| Complexity | O(n log n) total | O(n) amortized total |

## What Rust Learners Should Notice
- The `entry` API is uniquely powerful: it returns a mutable reference to the value (inserting a default if missing), avoiding double-lookup
- `*map.entry(word).or_insert(0) += 1` is the idiomatic one-liner for frequency counting in Rust
- `HashMap` is unordered; use `BTreeMap` if you need sorted keys (like OCaml's `Map`)
- OCaml's immutable map naturally supports persistent snapshots; Rust's mutable map is faster but requires explicit cloning for snapshots
- String normalization (`to_lowercase()`) returns a new `String` in Rust — strings are always UTF-8, never mutated in place

## Further Reading
- [The Rust Book — Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [Exercism Word Count](https://exercism.org/tracks/ocaml/exercises/word-count)

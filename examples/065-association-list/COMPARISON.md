# Association List — Functional Key-Value Store: OCaml vs Rust

## The Core Insight
Association lists are the "hello world" of functional data structures — just a list of pairs. They reveal how OCaml's persistent linked lists and Rust's owned Vecs handle the same abstraction with very different performance characteristics and ownership semantics.

## OCaml Approach
In OCaml, `insert k v lst = (k, v) :: lst` is a single cons operation — O(1) — and the old list is still intact (structural sharing). Lookup with `rec lookup k = function | (k', v) :: t -> ...` is clean pattern matching with O(n) scanning. The `remove` function reconstructs the list up to the matching element. Everything is garbage-collected, so the old list persists as long as anyone references it.

## Rust Approach
Rust's `Vec<(K, V)>` owns its elements. Insert at the front requires shifting all elements (O(n)) or we can push to the back and search in reverse. The implementation here uses `vec![(k, v)]` + `extend` for clarity. Lookup returns `Option<&V>` — a borrow, not a copy. The `remove` function consumes the Vec and produces a new one, making ownership transfer explicit. Iterator methods like `find`, `find_map`, and `filter` provide the functional style.

## Side-by-Side
| Concept | OCaml | Rust |
|---------|-------|------|
| Insert | `(k,v) :: lst` — O(1) cons | `vec![(k,v)]; extend(old)` — O(n) |
| Lookup | `rec lookup` pattern match | `.iter().find()` — iterator |
| Remove | Recursive rebuild | `.into_iter().filter().collect()` |
| Persistence | Free (structural sharing) | Requires cloning |
| Memory | GC handles sharing | Vec owns all elements |
| Return type | `'a option` | `Option<&V>` (borrowed) |

## What Rust Learners Should Notice
- OCaml lists are perfect for association lists because cons is O(1) and sharing is free; Rust Vecs don't share, so insert-at-front is expensive
- `find_map` is a Rust iterator gem — it combines `find` and `map` in one pass, equivalent to OCaml's recursive pattern match
- Rust's `into_iter()` consumes the collection, `iter()` borrows it — choosing correctly is key to ergonomic functional-style code
- For real code, use `HashMap` or `BTreeMap` — association lists are pedagogical, not practical for large datasets
- The `'a` lifetime in `lookup`'s return type ties the returned reference to the input slice — Rust makes this data dependency explicit

## Further Reading
- [Rust Iterator trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [OCaml Association Lists](https://cs3110.github.io/textbook/chapters/data/assoc_list.html)

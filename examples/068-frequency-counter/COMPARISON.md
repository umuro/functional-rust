# Frequency Counter — OCaml vs Rust Comparison

## Core Insight

Counting frequencies is the classic map use case. OCaml's `Map.Make` functor creates an immutable balanced tree map — each `add` creates a new map. Rust's `HashMap` is mutable with an `entry` API that makes update-or-insert a one-liner. For sorted output, Rust offers `BTreeMap`.

## OCaml Approach

`Map.Make(String)` creates a module with an ordered map backed by balanced binary trees. Each update via `SMap.add` returns a new immutable map (structural sharing keeps this efficient). The `find` + `Not_found` exception pattern is common but slightly awkward.

## Rust Approach

`HashMap::entry()` returns an `Entry` enum (Occupied or Vacant) — calling `.or_insert(0)` provides a mutable reference to the count, which can be incremented in place. No exception handling needed. For ordered iteration, use `BTreeMap` instead.

## Comparison Table

| Aspect        | OCaml                          | Rust                              |
|---------------|--------------------------------|-----------------------------------|
| **Memory**    | Immutable tree (shared nodes)  | Mutable hash table                |
| **Null safety** | `Not_found` exception       | `entry` API (no missing key panic)|
| **Errors**    | Exception on missing key       | `Entry` enum handles presence     |
| **Iteration** | `SMap.iter` (sorted by key)    | Unordered (HashMap) or sorted (BTreeMap) |
| **Update**    | `find` + `add` (new map)      | `entry().or_insert()` (in-place)  |

## Things Rust Learners Should Notice

1. **`entry()` API** — the killer feature for maps; avoids double-lookup (check then insert)
2. **`*freq.entry(k).or_insert(0) += 1`** — dereference the mutable ref, then increment
3. **`HashMap` vs `BTreeMap`** — hash is O(1) average but unordered; BTree is O(log n) but sorted
4. **OCaml's Map is immutable** — functional purity at the cost of creating new nodes on each update
5. **`split_whitespace()`** — handles multiple spaces, tabs, newlines; better than `split(' ')`

## Further Reading

- [HashMap::entry](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry)
- [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)

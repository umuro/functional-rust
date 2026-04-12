📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1050-interning)**

---

# 1050-interning — String Interning
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

When a program repeatedly stores and compares the same set of strings — identifiers in a compiler, field names in a serializer, CSS class names in a renderer — storing each string as a separate allocation wastes memory and makes comparison O(|string|) instead of O(1).

String interning maps each unique string to a small integer "symbol". Comparing two symbols is a single integer comparison. The interner maintains a bidirectional map between strings and IDs, ensuring each unique string is stored exactly once.

## Learning Outcomes

- Implement a string interner with `HashMap<String, Symbol>` and `Vec<String>`
- Understand that interned symbols compare in O(1) instead of O(|string|)
- Return the same symbol for repeated insertions of the same string
- Look up the original string from a symbol in O(1)
- Connect to compiler symbol tables and CSS parser use cases

## Rust Application

`src/lib.rs` defines `Symbol(usize)` as a newtype wrapper around an index. The `Interner` struct maintains `to_id: HashMap<String, Symbol>` for forward lookup and `to_str: Vec<String>` for reverse lookup. `intern(&str) -> Symbol` returns the existing ID or assigns a new one. `resolve(Symbol) -> Option<&str>` looks up the original string by index.

Symbol comparison is `symbol1 == symbol2` — a single integer comparison. The `lasso` and `string-interner` crates provide production implementations with thread-safety and arena allocation.

## OCaml Approach

OCaml uses `Hashtbl` for mutable interning:

```ocaml
let interner = Hashtbl.create 256
let next_id = ref 0

let intern s =
  match Hashtbl.find_opt interner s with
  | Some id -> id
  | None ->
    let id = !next_id in
    Hashtbl.add interner s id;
    incr next_id;
    id
```

OCaml's polymorphic equality already compares strings by value — interning is more about memory deduplication than comparison speed. The `Camomile` and `Zarith` libraries use interning for performance-critical string handling.

## Key Differences

1. **Motivation**: In Rust, string comparison is O(|string|) even with `==`; interning reduces it to O(1). In OCaml, `String.equal` is O(n) but physical equality `==` is O(1) and can be used after interning.
2. **Memory deduplication**: Both languages benefit from storing each unique string once; the memory saving is language-independent.
3. **Thread safety**: Rust's single-threaded interner uses `HashMap`; a thread-safe version needs `Mutex` or `DashMap`. OCaml's `Hashtbl` is single-threaded; `Domain`-safe versions need explicit locking.
4. **Arena allocation**: Production interners use arena allocation to avoid individual `String` heap allocations; Rust's `typed-arena` crate supports this.

## Exercises

1. Make the interner thread-safe by wrapping it in `Arc<Mutex<Interner>>` and benchmark the contention.
2. Implement `Interner::merge(other: Interner) -> Interner` that combines two interners, remapping the IDs of one to avoid conflicts.
3. Write a simple parser that interns all identifiers during lexing and compares symbols (not strings) during semantic analysis.

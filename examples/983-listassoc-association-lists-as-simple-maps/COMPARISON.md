# OCaml vs Rust: List.assoc — Association Lists as Simple Maps

## Side-by-Side Code

### OCaml

```ocaml
let phonebook = [("Alice", "555-1234"); ("Bob", "555-5678"); ("Carol", "555-9012")]

(* List.assoc raises Not_found on missing key; List.assoc_opt returns option *)
let bobs_number   = List.assoc "Bob" phonebook
let has_dave      = List.mem_assoc "Dave" phonebook
let without_bob   = List.remove_assoc "Bob" phonebook
```

### Rust (idiomatic — iterator combinators)

```rust
pub fn assoc<'a, K: PartialEq, V>(key: &K, pairs: &'a [(K, V)]) -> Option<&'a V> {
    pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v)
}

pub fn mem_assoc<K: PartialEq, V>(key: &K, pairs: &[(K, V)]) -> bool {
    pairs.iter().any(|(k, _)| k == key)
}

pub fn remove_assoc<'a, K: PartialEq, V>(key: &K, pairs: &'a [(K, V)]) -> Vec<&'a (K, V)> {
    let mut removed = false;
    pairs.iter().filter(|(k, _)| {
        if !removed && k == key { removed = true; false } else { true }
    }).collect()
}
```

### Rust (functional/recursive — mirrors OCaml structure)

```rust
pub fn assoc_recursive<'a, K: PartialEq, V>(key: &K, pairs: &'a [(K, V)]) -> Option<&'a V> {
    match pairs {
        []                  => None,
        [(k, v), ..]  if k == key => Some(v),
        [_, rest @ ..] => assoc_recursive(key, rest),
    }
}

pub fn mem_assoc_recursive<K: PartialEq, V>(key: &K, pairs: &[(K, V)]) -> bool {
    match pairs {
        []             => false,
        [(k, _), ..]   if k == key => true,
        [_, rest @ ..] => mem_assoc_recursive(key, rest),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Lookup | `val assoc : 'a -> ('a * 'b) list -> 'b` | `fn assoc<'a,K,V>(key:&K, pairs:&'a [(K,V)]) -> Option<&'a V>` |
| Membership | `val mem_assoc : 'a -> ('a * 'b) list -> bool` | `fn mem_assoc<K,V>(key:&K, pairs:&[(K,V)]) -> bool` |
| Removal | `val remove_assoc : 'a -> ('a * 'b) list -> ('a * 'b) list` | `fn remove_assoc<'a,K,V>(key:&K, pairs:&'a [(K,V)]) -> Vec<&'a (K,V)>` |
| Key constraint | implicit `=` polymorphism | `K: PartialEq` |
| Missing key | raises `Not_found` | `None` |
| Association list | `('a * 'b) list` | `&[(K, V)]` slice |

## Key Insights

1. **Option vs exception:** OCaml's `List.assoc` raises `Not_found` — a runtime exception you must catch. `List.assoc_opt` is the safe variant. Rust makes safety mandatory: the return type is always `Option<&V>`, forcing callers to handle the missing-key case at compile time.

2. **Lifetime annotations surface the borrow:** When Rust returns `&'a V` from a slice `&'a [(K,V)]`, the `'a` lifetime explicitly states "this reference lives only as long as the input data." OCaml's garbage collector handles this silently — Rust makes the dependency visible and verifiable.

3. **PartialEq vs structural equality:** OCaml's polymorphic `=` works on any type automatically. Rust requires the explicit bound `K: PartialEq`, which is stricter but prevents accidental equality on types that shouldn't support it (e.g., floating-point with NaN).

4. **Iterator combinators vs recursion:** `.find()` and `.any()` capture the same traversal logic as OCaml's `match [] | (k,v)::rest` pattern but read more declaratively. The recursive Rust versions show the OCaml structure directly — useful when teaching the translation.

5. **Allocation model for removal:** OCaml's `List.remove_assoc` allocates a fresh list. The Rust idiomatic version returns `Vec<&(K,V)>` — a vector of references into the original slice — avoiding duplication of the data. For owned output, callers can `.cloned()` or `.to_owned()` as needed.

## When to Use Each Style

**Use idiomatic Rust (iterators) when:** you want concise, readable code in production. `.find()`, `.any()`, `.filter()` compose naturally with the rest of Rust's iterator ecosystem.

**Use recursive Rust when:** you are teaching the OCaml→Rust translation explicitly, or when the problem has complex structural recursion that maps better to pattern matching than to a flat iterator pipeline.

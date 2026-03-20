📖 **[View on hightechmind.io →](https://hightechmind.io/rust/357-entry-api)**

---

# 357: Entry API
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A common map operation is "if the key exists, update the value; if not, insert a default." Naively this requires two lookups: `get` then `insert`. Rust's entry API solves this with a single lookup that returns an `Entry` enum — `Occupied` or `Vacant` — allowing you to atomically inspect and modify the map without the borrow checker complications of holding a reference to the value while also modifying the map. This pattern, introduced in Rust 1.0, avoids cloning keys unnecessarily and is the idiomatic way to implement counting, memoization, and conditional initialization.

## Learning Outcomes

- Use `entry(key).or_insert(default)` for insert-if-absent
- Use `entry(key).or_insert_with(f)` for lazily computed defaults
- Use `entry(key).and_modify(|v| ...).or_insert(default)` for conditional update
- Use `entry(key).or_default()` when `V: Default`
- Understand that `entry()` takes ownership of the key only when inserting (Vacant)
- Implement memoization and lazy initialization using the entry API

## Rust Application

```rust
use std::collections::HashMap;

pub fn count_chars(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.chars() {
        map.entry(c).and_modify(|n| *n += 1).or_insert(1);
    }
    map
}

pub fn get_or_compute<K, V, F>(map: &mut HashMap<K, V>, key: K, compute: F) -> &V
where
    K: Eq + std::hash::Hash,
    F: FnOnce() -> V,
{
    map.entry(key).or_insert_with(compute)
    // compute() only called if key is absent
}

pub fn update_with_default<K, V, F>(map: &mut HashMap<K, V>, key: K, default: V, update: F)
where
    K: Eq + std::hash::Hash,
    F: FnOnce(&mut V),
{
    map.entry(key).and_modify(update).or_insert(default);
    // if present: call update; if absent: insert default (update NOT called)
}
```

`or_insert_with(compute)` is critical for memoization — `compute()` is only called when the key is absent, making it suitable for expensive computations. `and_modify(...).or_insert(default)` reads as "modify if present, otherwise insert default" — the order matters: `and_modify` runs only on `Occupied` entries.

## OCaml Approach

OCaml's `Hashtbl` requires explicit `find` + `replace`:

```ocaml
let get_or_compute tbl key compute =
  match Hashtbl.find_opt tbl key with
  | Some v -> v
  | None ->
    let v = compute () in
    Hashtbl.add tbl key v; v

(* With Map (functional): *)
module M = Map.Make(String)
let count_chars s =
  String.fold_left (fun m c ->
    M.update (String.make 1 c) (function
      | None -> Some 1
      | Some n -> Some (n + 1)) m
  ) M.empty s
```

`Map.update` in OCaml (4.06+) is the closest equivalent to Rust's entry API — it passes `None` or `Some` to a function and uses the return value as the new binding. Still requires two logical branches vs Rust's fluent `and_modify().or_insert()` chain.

## Key Differences

| Aspect | Rust entry API | OCaml `Map.update` / `Hashtbl` |
|--------|---------------|-------------------------------|
| Lookup count | One (single hash computation) | One (Map) or two (Hashtbl) |
| Key ownership | Taken only if inserting | Always cloned/allocated |
| Chaining | `.and_modify().or_insert()` fluent | Nested `match` |
| Lazy default | `or_insert_with(f)` | `match None -> f ()` |
| Return value | `&mut V` reference | Value (not reference) |

## Exercises

1. **Fibonacci memoization**: Implement recursive Fibonacci with memoization using `entry(n).or_insert_with(|| compute_fib(n, memo))` in a `HashMap<u64, u64>` passed through recursion.
2. **Word adjacency**: Build a `HashMap<String, Vec<String>>` of word→next-words from a text corpus using `entry(word).or_default().push(next)`; sample random sentences from the resulting Markov chain.
3. **Configuration defaults**: Implement `apply_defaults(config: &mut HashMap<&str, String>, defaults: &[(&str, &str)])` using `entry(k).or_insert(v.to_string())` — only insert if key is absent.

# 357: Entry API — Insert or Update Without Double Lookup

**Difficulty:** 2  **Level:** Intermediate

A single-lookup handle to a map slot that lets you initialize, update, or conditionally modify — the idiomatic Rust alternative to "get then insert".

## The Problem This Solves

The naive way to "insert if missing, update if present" in a map involves two lookups: `if map.contains_key(&k) { map.get_mut(&k)... } else { map.insert(k, default) }`. This is not only verbose — it's also subtly wrong with the borrow checker. The immutable borrow from `contains_key` and the mutable borrow from `get_mut` conflict if you're not careful.

The entry API resolves this. `map.entry(key)` returns an `Entry` enum — either `Occupied` (key exists) or `Vacant` (key missing). Either way, you hold a handle to that slot in the map without paying for a second lookup. The methods on `Entry` — `or_insert`, `or_insert_with`, `and_modify`, `or_default` — cover all the common patterns cleanly.

This matters most in hot loops. Building a frequency map over a billion tokens, accumulating a graph's edge weights, or maintaining per-user session state — every `.entry()` call does exactly one hash and one comparison. The double-lookup version does two.

## The Intuition

No direct Python equivalent — Python dicts don't expose a "slot handle". The closest idiom is `d.setdefault(key, default)` (insert if missing) or `d[key] = d.get(key, 0) + 1` (counter pattern). Rust's entry API is more powerful: it chains initialization and modification, and you can run arbitrary closures on the value in one pass.

Think of `Entry` as a "pointer to a slot" — it keeps the map locked to that slot until you're done. No one else can touch that slot between `entry()` and the method call. That's the guarantee that lets the borrow checker be happy and lets you avoid the double-lookup.

## How It Works in Rust

```rust
use std::collections::HashMap;

let mut map: HashMap<&str, Vec<i32>> = HashMap::new();

// or_insert: insert a default if the key is missing
// Returns a mutable reference to the value (new or existing)
map.entry("alice").or_insert_with(Vec::new).push(1);
map.entry("alice").or_insert_with(Vec::new).push(2); // appends to existing

// or_insert: for Copy types, insert a literal
let mut counts: HashMap<&str, u32> = HashMap::new();
*counts.entry("rust").or_insert(0) += 1; // idiomatic counter

// or_default: uses the type's Default implementation
let mut groups: HashMap<&str, Vec<&str>> = HashMap::new();
groups.entry("fruits").or_default().push("apple"); // Vec::default() = vec![]

// and_modify: run a closure only when key already exists
// Chains with or_insert for full insert-or-update
let mut scores: HashMap<&str, i32> = HashMap::new();
scores
    .entry("alice")
    .and_modify(|s| *s += 10) // only called if key exists
    .or_insert(10);            // only called if key was missing

// The Entry enum directly (for advanced branching)
use std::collections::hash_map::Entry;
match map.entry("bob") {
    Entry::Occupied(mut e) => {
        e.get_mut().push(99);  // key exists — mutate in place
    }
    Entry::Vacant(e) => {
        e.insert(vec![99]);    // key missing — insert
    }
}
```

## What This Unlocks

- **Frequency maps and counters**: `*map.entry(k).or_insert(0) += 1` — the single idiomatic line for any counting task.
- **Graph construction**: build adjacency lists with `entry(node).or_default().push(neighbor)` without checking existence.
- **Session state management**: lazily initialize per-user or per-connection state on first access, O(1) lookup on subsequent accesses.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Insert-or-update | `find` + `replace` (two ops) | `.entry().or_insert()` (one lookup) |
| Insert with factory | `find` + conditional `add` | `.entry().or_insert_with(|| ...)` |
| Update if present | `find` + `replace` | `.entry().and_modify(|v| ...)` |
| Full insert-or-update | manual match | `.entry().and_modify(f).or_insert(v)` |
| Access slot directly | N/A | `Entry::Occupied` / `Entry::Vacant` |
| Works on | `Hashtbl`, `Map` | `HashMap`, `BTreeMap` |

📖 **[View on hightechmind.io →](https://hightechmind.io/rust/547-lifetime-polonius)**

---

# Polonius Borrow Checker Concepts
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

NLL (Non-Lexical Lifetimes) dramatically improved Rust's borrow checker but still rejects some provably safe code. The most famous example: looking up a key in a map, and if absent, inserting and returning a reference to the new value — the classic "get-or-insert" pattern. NLL conservatively rejects this because the mutable borrow for the lookup overlaps with the mutable borrow for the insert, even though they cannot both be active simultaneously. Polonius, the next-generation borrow checker based on Datalog constraints, accepts this pattern and other currently-rejected safe code.

## Learning Outcomes

- What patterns NLL conservatively rejects that are provably safe
- The classic "get-or-insert" pattern and why NLL's two-phase borrow check fails on it
- How to work around NLL limitations using `contains_key` + separate `get`, or the `entry` API
- What Polonius is and how it improves over NLL using Datalog-based constraint solving
- Where Polonius matters most: HashMap get-or-insert, conditional borrow returns

## Rust Application

`get_or_insert<'a>` shows the NLL workaround: check `contains_key` first (shared borrow ends), then `insert` (mutable borrow), then `get` (shared borrow again). This is more verbose than the direct pattern but safe. `get_or_insert_entry` shows the idiomatic `Entry` API that avoids the problem entirely. `find_or_create` returns an index — the workaround that Polonius would handle: search with an iterator borrow, push after the borrow ends, return the index of the new element.

Key patterns:
- `if !map.contains_key(&k) { map.insert(...) } map.get(&k)` — NLL workaround
- `map.entry(k).or_insert_with(|| ...)` — idiomatic entry API avoiding the problem
- `for (i, item) in items.iter().enumerate()` + `items.push()` after loop

## OCaml Approach

OCaml's `Hashtbl` and functional `Map` have no borrow checker limitations — get-or-insert is direct:

```ocaml
let get_or_insert tbl key default =
  match Hashtbl.find_opt tbl key with
  | Some v -> v
  | None -> Hashtbl.add tbl key default; default
```

No workaround needed — the hash table is always mutably accessible through its reference.

## Key Differences

1. **NLL limitation**: Rust's NLL rejects get-or-insert patterns even when provably safe; OCaml's lack of borrow checking means no such limitation exists.
2. **Entry API**: Rust's `HashMap::entry` was specifically designed to work around NLL limitations — it is the idiomatic solution; OCaml has no equivalent API pattern needed.
3. **Polonius timeline**: Polonius is a research project that will replace NLL — it accepts all NLL-accepted programs plus additional safe programs; OCaml's model is orthogonal.
4. **Workaround cost**: NLL workarounds for get-or-insert require double lookups (one `contains_key`, one `get`) — a performance cost that Polonius would eliminate; OCaml pays no such cost.

## Exercises

1. **Entry API mastery**: Rewrite `get_or_insert` using only the `Entry` API with `or_insert`, `or_insert_with`, and `or_default` variants — compare the verbosity of each.
2. **Two-pass workaround**: Implement a `fn get_or_compute<'a>(map: &'a mut HashMap<i32, i32>, key: i32, f: impl Fn(i32) -> i32) -> &'a i32` using the NLL workaround (two lookups).
3. **Find-or-add dedup**: Write `fn dedup_insert(v: &mut Vec<String>, s: String) -> usize` that returns the index of `s` if already present, or inserts it and returns the new index — without cloning `s` unnecessarily.

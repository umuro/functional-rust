📖 **[View on hightechmind.io →](https://hightechmind.io/rust/289-iterator-extend)**

---

# 289: Extending Collections with extend()

**Difficulty:** 2  **Level:** Intermediate

Append elements from any iterator into an existing collection in place — the in-place alternative to `chain().collect()`.

## The Problem This Solves

You have an existing collection and you want to add more elements to it. The naive approach: `let new_vec = existing.into_iter().chain(more).collect()` — but that consumes the original, creates a new allocation, and loses the variable binding. If you're appending in a loop (building up a result incrementally), each iteration creates and destroys a vec.

`extend()` adds elements to an existing collection in place, reusing its allocation when possible. It's more efficient than `chain + collect` when you already have a collection and just want to grow it. It also works on any collection that implements `Extend<T>` — `Vec`, `String`, `HashMap`, `HashSet`, `BTreeSet`, and more.

OCaml's lists are immutable, so there's no in-place equivalent — you prepend with `::` or use `Buffer.add_string` for strings. Rust's `extend()` is the imperative counterpart to functional `append`.

## The Intuition

`collection.extend(iterator)` drains the iterator and pushes each element into the collection — equivalent to calling `push`/`insert` in a loop, but expressed as a single declarative call.

```rust
let mut v = vec![1, 2, 3];
v.extend([4, 5, 6]);
// v is now [1, 2, 3, 4, 5, 6]
```

## How It Works in Rust

```rust
use std::collections::{HashMap, HashSet};

// Extend a Vec
let mut base = vec![1i32, 2, 3];
base.extend([4, 5, 6]);               // from array
base.extend(7..=9);                   // from range
// → [1, 2, 3, 4, 5, 6, 7, 8, 9]

// Extend String with chars (String implements Extend<char>)
let mut s = String::from("Hello");
s.extend(", world!".chars());
// → "Hello, world!"

// Extend HashMap — duplicate keys overwrite
let mut map: HashMap<&str, i32> = HashMap::new();
map.insert("a", 1);
map.extend([("b", 2), ("c", 3)]);
// → {"a": 1, "b": 2, "c": 3}

// Extend HashSet — duplicates are silently ignored
let mut set: HashSet<i32> = [1, 2, 3].iter().copied().collect();
set.extend([3, 4, 5]);  // 3 already present — no duplicate
// → {1, 2, 3, 4, 5}

// Extend with a transformed iterator
let mut evens = vec![2i32, 4];
evens.extend((1..20).filter(|x| x % 2 == 0).take(3));
// appends [2, 4, 6] to existing [2, 4]
// → [2, 4, 2, 4, 6]

// Incremental building — more efficient than repeated chain+collect
let mut result: Vec<i32> = Vec::new();
for batch in [[1, 2], [3, 4], [5, 6]] {
    result.extend(batch);  // reuses allocation when possible
}
```

`extend()` calls `reserve()` internally when it can determine the size hint, pre-allocating space to avoid repeated reallocation.

## What This Unlocks

- **Incremental collection building** — accumulate results across loop iterations without creating intermediate collections.
- **Multi-source merging** — extend a vec from several different iterators, each adding a slice or range.
- **In-place set/map population** — add entries to an existing `HashMap` or `HashSet` from a sequence of key-value pairs.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Append to collection | `lst @ more` (new list) | `vec.extend(iter)` (in-place) |
| String append | `Buffer.add_string` | `string.extend(chars)` or `string.push_str` |
| HashMap insert many | `List.fold_left` + `Hashtbl.add` | `map.extend(pairs)` |
| Allocation | Always new allocation | Reuses existing allocation (amortized) |
| vs. `chain + collect` | Equivalent output | `extend` avoids creating a new collection |

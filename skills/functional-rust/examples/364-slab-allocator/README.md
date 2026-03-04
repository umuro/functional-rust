# 364: Slab Pattern for Indexed Storage

**Difficulty:** 3  **Level:** Advanced

Pre-allocated pool with stable integer indices — insert returns an index, not a reference.

## The Problem This Solves

Sometimes you want a collection of objects where each object has a stable, reusable identity even as items come and go. A game engine with entities being created and destroyed. A network server tracking open connections. A graph implementation where nodes need stable IDs.

You could use `Vec` with a "tombstone" marker for deleted slots, but you'd need to manage free-list recycling yourself. You could use `HashMap<usize, T>`, but that's heap allocation per entry plus hash overhead. You could use `Box<T>` handles, but then the borrow checker makes passing handles around painful.

The `slab` crate gives you a pre-allocated pool that returns integer keys on insert, recycles slots on remove, and guarantees `O(1)` insert, remove, and lookup. No heap allocation per element after the initial slab creation. No fragmentation.

## The Intuition

A slab maintains a `Vec<Slot<T>>` where each slot is either `Occupied(T)` or `Vacant(next_free_index)`. The vacant slots form a linked list via their stored indices — the "free list." Insert finds the next free slot (or grows the Vec), puts your value in, and returns the slot index. Remove marks the slot vacant and prepends it to the free list. Lookup is just `vec[index]`.

The key property: indices are stable. When you remove entry 5, entry 6's index doesn't change. This makes slabs ideal for graph nodes, entity IDs, and any structure where external code holds references by integer key.

## How It Works in Rust

```rust
use slab::Slab;

let mut slab = Slab::new();

// Insert returns a stable key
let alice = slab.insert("Alice");   // key: 0
let bob   = slab.insert("Bob");     // key: 1
let carol = slab.insert("Carol");   // key: 2

// O(1) lookup by key
println!("{}", slab[bob]);  // "Bob"

// Remove — key 1 is now free
slab.remove(bob);

// Next insert reuses key 1
let dave = slab.insert("Dave");
assert_eq!(dave, 1);

// Iterate occupied entries
for (key, name) in &slab {
    println!("{key}: {name}");
}
```

## What This Unlocks

- **Entity-component systems** — entities are slab keys; components stored in parallel slabs indexed by same key.
- **Connection pools** — each connection gets a stable ID; closed connections' slots are recycled.
- **Graph representations** — nodes stored in slabs, edges stored as `(usize, usize)` pairs — no lifetime juggling.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Object pool | Manual with array + free list | `slab::Slab<T>` — handles free list automatically |
| Stable identity | Integer index into array | `slab.insert(v)` returns `usize` key |
| Slot recycling | Manual | Automatic via internal free list |
| Access | `array.(index)` | `slab[key]` or `slab.get(key)` |

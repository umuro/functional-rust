📖 **[View on hightechmind.io →](https://hightechmind.io/rust/364-slab-allocator)**

---

# 364: Slab Allocator
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Graph nodes, AST nodes, and ECS (Entity Component System) game entities all need stable references that don't invalidate when other items are added or removed. Raw indices into a `Vec` are unstable — removing element 5 shifts all subsequent elements, invalidating stored indices. A slab allocator solves this: it maintains a `Vec<Option<T>>` where items occupy stable slots identified by integer keys. Removed slots are tracked in a free list and reused for future allocations. Keys remain valid across insertions and removals of other elements. The `slab` crate is the production implementation; this example shows the pattern from scratch.

## Learning Outcomes

- Implement a slab with `Vec<Option<T>>` for stable-key storage
- Maintain a `Vec<usize>` free list to reuse vacated slots
- Return stable integer keys from `insert` that remain valid after other removals
- Retrieve, remove, and iterate entries using the key-based API
- Understand that slab allocation is O(1) amortized for insert and O(1) for get/remove
- Recognize slab as the foundation for ECS, graph adjacency, and memory pools

## Rust Application

```rust
pub struct Slab<T> {
    entries: Vec<Option<T>>,
    free: Vec<usize>,
}

impl<T> Slab<T> {
    pub fn new() -> Self {
        Self { entries: Vec::new(), free: Vec::new() }
    }

    pub fn insert(&mut self, val: T) -> usize {
        if let Some(key) = self.free.pop() {
            self.entries[key] = Some(val); // reuse freed slot
            key
        } else {
            let key = self.entries.len();
            self.entries.push(Some(val)); // grow
            key
        }
    }

    pub fn get(&self, key: usize) -> Option<&T> {
        self.entries.get(key)?.as_ref()
    }

    pub fn remove(&mut self, key: usize) -> Option<T> {
        let val = self.entries.get_mut(key)?.take()?;
        self.free.push(key); // reclaim slot for future inserts
        Some(val)
    }

    pub fn contains(&self, key: usize) -> bool {
        self.entries.get(key).map_or(false, |e| e.is_some())
    }
}
```

`take()` replaces `Some(val)` with `None` in-place and returns the value — a single method that removes and returns atomically. The free list is a LIFO stack (`pop`/`push`) — most recently freed keys are reused first, which tends to improve cache locality.

## OCaml Approach

OCaml's garbage collector provides automatic stable references (object identity). For explicit slab behavior with integer keys, use an array with an option type:

```ocaml
type 'a slab = {
  mutable entries: 'a option array;
  mutable free: int list;
  mutable next: int;
}

let insert s val =
  match s.free with
  | k :: rest -> s.free <- rest; s.entries.(k) <- Some val; k
  | [] ->
    let k = s.next in
    s.next <- k + 1;
    (* resize if needed *)
    s.entries.(k) <- Some val; k

let remove s k =
  let v = s.entries.(k) in
  s.entries.(k) <- None;
  s.free <- k :: s.free;
  v
```

In OCaml, stable object references are just values tracked by the GC — the slab pattern is mainly useful when you need integer keys for serialization, inter-process communication, or C FFI.

## Key Differences

| Aspect | Rust `Slab<T>` | OCaml `'a option array` |
|--------|---------------|------------------------|
| Key stability | Integer key (stable across mutations) | Object reference (GC-stable) |
| Free list | `Vec<usize>` LIFO | `int list` |
| Memory | Contiguous array (cache-friendly) | Array of boxed options |
| Iteration | Filter `Some` entries | Filter `Some` entries |
| Production | `slab` crate | No standard equivalent |

## Exercises

1. **Iteration**: Implement `fn iter(&self) -> impl Iterator<Item = (usize, &T)>` that yields `(key, &value)` pairs for all occupied slots, filtering out `None` entries.
2. **Graph with slab**: Represent a directed graph where nodes are slab-allocated; `remove_node(key)` removes the node and all edges referencing it from other nodes' adjacency lists.
3. **Generation counter**: Add a generation counter per slot to detect use-after-free: keys become `(index, generation)` pairs; removing increments the generation, and `get` checks that the stored generation matches.

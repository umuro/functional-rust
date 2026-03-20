📖 **[View on hightechmind.io →](https://hightechmind.io/rust/358-indexmap-ordered)**

---

# 358: IndexMap Ordered (Insertion-Order Map)

## Problem Statement

`HashMap` iterates in arbitrary order; `BTreeMap` iterates in key-sorted order. But sometimes you need to iterate in insertion order — preserving the sequence in which entries were added. This is how Python dicts work since 3.7, how JSON objects are commonly expected to behave, and how HTTP headers must be processed. The `indexmap` crate provides this natively; this example demonstrates the pattern using a `HashMap` + `Vec<K>` combination to illustrate the mechanism, which you can replace with the real `IndexMap` crate in production.

## Learning Outcomes

- Build an insertion-order-preserving map using `HashMap<K, V>` + `Vec<K>` for order tracking
- Insert maintaining both the hash map (O(1) lookup) and the order vector (O(1) push)
- Iterate in insertion order by walking the `Vec<K>` and looking up values
- Understand why skipping the order vector loses insertion order guarantees
- Recognize that the `indexmap` crate is the production-quality solution
- Know the cost: `remove` is O(n) due to Vec compaction (use `swap_remove` for O(1))

## Rust Application

```rust
use std::collections::HashMap;

pub struct OrderedMap<K, V> {
    map: HashMap<K, V>,
    order: Vec<K>,
}

impl<K: Clone + Eq + std::hash::Hash, V> OrderedMap<K, V> {
    pub fn new() -> Self {
        Self { map: HashMap::new(), order: Vec::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.map.insert(key.clone(), value).is_none() {
            self.order.push(key); // track insertion order only for new keys
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key) // O(1)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.order.iter()
            .filter_map(|k| self.map.get(k).map(|v| (k, v)))
    }
}
```

The key insight: `map.insert(key.clone(), value).is_none()` returns `true` only if the key was absent, so we push to `order` only once per unique key. Updates (`insert` on existing key) don't duplicate the order entry.

## OCaml Approach

OCaml's association lists (`(key * value) list`) naturally preserve insertion order:

```ocaml
(* Association list: insertion-ordered, O(n) lookup *)
let insert lst k v = (k, v) :: lst  (* prepends — reverse insertion order *)
let get lst k = List.assoc_opt k lst
let iter lst f = List.iter (fun (k, v) -> f k v) (List.rev lst)

(* For production: use the ordered-hashtbl or sequence package *)
```

Association lists are the OCaml idiom for small ordered maps. For larger maps, the `orderedhashtbl` package or `CCHashtbl.Poly` with order tracking mirrors the HashMap+Vec approach.

## Key Differences

| Aspect | Rust `HashMap` + `Vec` | OCaml assoc list |
|--------|----------------------|------------------|
| Lookup | O(1) | O(n) |
| Insertion | O(1) amortized | O(1) prepend |
| Iteration | Insertion order | Insertion order (reversed) |
| Remove | O(n) for ordered remove | O(n) filter |
| Production solution | `indexmap` crate | `orderedhashtbl` package |

## Exercises

1. **`remove` with order**: Implement `remove(&mut self, key: &K) -> Option<V>` that removes the key from both the `HashMap` and the `Vec<K>`; use `Vec::retain` for correctness or `swap_remove` for O(1) (unordered removal).
2. **JSON object ordering**: Use `OrderedMap<String, String>` to represent a JSON object and serialize it with keys in insertion order; compare output with `HashMap`-based serialization.
3. **Indexmap crate**: Replace `OrderedMap` with the `indexmap::IndexMap` crate; verify that `get_index(0)` returns the first inserted key-value pair, and that iteration order matches insertion order.

[lru-cache on hightechmind.io](https://hightechmind.io/posts/functional-rust/lru-cache)

---

## Problem Statement

Implement an LRU (Least Recently Used) cache with a fixed capacity. When the cache is full, evict the least recently accessed entry on `put`. Maintain access order using `HashMap` for O(1) lookup and `VecDeque` as a recency queue. Both `get` and `put` update the recency order.

## Learning Outcomes

- Combine `HashMap<K, V>` for O(1) key lookup with `VecDeque<K>` for recency tracking
- Implement `get(&mut self, key: &K) -> Option<&V>` that promotes the key to the back (most-recently-used position) via `retain`
- Implement `put(key, value)` that evicts the front of the deque (LRU end) when at capacity
- Understand the `K: Eq + Hash + Clone` bounds required for hash map keys and deque membership
- Recognize the O(n) deque `retain` cost and contrast with O(1) doubly-linked-list alternatives

## Rust Application

```rust
pub struct LruCache<K, V> {
    capacity: usize,
    table: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: Eq + std::hash::Hash + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be > 0");
        LruCache {
            capacity,
            table: HashMap::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.table.contains_key(key) {
            self.order.retain(|k| k != key);  // remove old position
            self.order.push_back(key.clone()); // move to MRU end
            self.table.get(key)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.table.contains_key(&key) {
            self.order.retain(|k| k != &key);
        } else if self.table.len() >= self.capacity {
            if let Some(lru) = self.order.pop_front() {
                self.table.remove(&lru);
            }
        }
        self.order.push_back(key.clone());
        self.table.insert(key, value);
    }
}
```

The `VecDeque` acts as an order log: back = most recently used (MRU), front = least recently used (LRU). On access, `retain` removes the key from its current position and `push_back` inserts it at the MRU end — O(n) but simple.

`K: Clone` is required because the key is stored in both the `HashMap` (as the map key) and the `VecDeque` (as the order record). The two structures must independently own copies.

The production-grade approach uses a doubly linked list + HashMap for O(1) get/put (see `lru` crate). This implementation prioritizes clarity over performance.

## OCaml Approach

```ocaml
module type LRU = sig
  type ('k, 'v) t
  val create : int -> ('k, 'v) t
  val get : ('k, 'v) t -> 'k -> 'v option
  val put : ('k, 'v) t -> 'k -> 'v -> unit
end

(* Simple association-list LRU (O(n) all operations, pedagogical) *)
let create cap = { capacity = cap; entries = [] }

let get cache key =
  match List.assoc_opt key cache.entries with
  | None -> None
  | Some v ->
    cache.entries <-
      (key, v) :: List.filter (fun (k, _) -> k <> key) cache.entries;
    Some v

let put cache key value =
  let filtered = List.filter (fun (k, _) -> k <> key) cache.entries in
  let trimmed =
    if List.length filtered >= cache.capacity then
      List.filteri (fun i _ -> i < cache.capacity - 1) filtered
    else filtered
  in
  cache.entries <- (key, value) :: trimmed
```

OCaml's association list approach mirrors the deque approach but uses the list head as MRU. Both have O(n) update cost. For production, OCaml's `Hashtbl` combined with a `Queue` gives O(1) amortized.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Lookup structure | `HashMap` — O(1) | `Hashtbl` or assoc list |
| Order structure | `VecDeque` — O(n) retain | List — O(n) filter |
| `K: Clone` bound | Required for dual ownership | GC shares automatically |
| O(1) production | `lru` crate (linked list + map) | External library needed |
| `assert!` on capacity | Panics on capacity = 0 | Equivalent `assert false` |

## Exercises

1. Implement `len(&self) -> usize` and `is_empty(&self) -> bool`.
2. Implement `peek(&self, key: &K) -> Option<&V>` that looks up without updating recency order.
3. Refactor to use a doubly linked list + HashMap for O(1) get/put (or use the `lru` crate and benchmark the difference).
4. Add `evict_callback: Option<Box<dyn Fn(K, V)>>` that fires when an entry is evicted.
5. Implement `resize(new_capacity)` that evicts entries from the LRU end until the cache fits the new capacity.

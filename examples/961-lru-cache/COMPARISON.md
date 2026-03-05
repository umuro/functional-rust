# LRU Cache — Comparison

## Core Insight
LRU cache = hash map (fast lookup) + ordered queue (eviction order). Both OCaml and Rust use the same two-structure approach. The `get` operation must update recency — removing the key from the middle of the queue and reinserting at the back. Both use O(n) queue cleanup (sufficient for most uses; production LRU uses a doubly-linked list).

## OCaml Approach
- `Hashtbl.t` + `Queue.t` in a record `{ capacity; table; order }`
- `Queue.pop` — dequeue front (LRU victim)
- `Queue.add k q` — enqueue back (most recent)
- `remove_from_queue` — filter queue to remove a key from the middle
- `Queue.iter` + temporary queue for key removal
- Mutable structure passed by reference (OCaml records are mutable when fields are `mutable`)

## Rust Approach
- `HashMap<K, V>` + `VecDeque<K>` in a struct
- `VecDeque::pop_front()` — remove LRU
- `VecDeque::push_back(key)` — insert as MRU
- `order.retain(|k| k != key)` — elegant middle removal (O(n))
- `K: Eq + Hash + Clone` — trait bounds for HashMap + cloning into deque
- `&mut self` on `get` — Rust enforces mutation is explicit

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Hash store | `Hashtbl.t` | `HashMap<K, V>` |
| Order queue | `Queue.t` | `VecDeque<K>` |
| Evict front | `Queue.pop q` | `order.pop_front()` |
| Add to back | `Queue.add k q` | `order.push_back(k)` |
| Remove from middle | Manual filter with temp queue | `order.retain(\|k\| k != key)` |
| Get mutates (recency) | Yes (mutable record fields) | `&mut self` required |
| Generic types | `('k, 'v) lru` | `LruCache<K, V>` |
| Trait bounds | None (structural) | `K: Eq + Hash + Clone` |

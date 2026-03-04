# 375: LRU Cache — Evict the Least Recently Used

**Difficulty:** 3  **Level:** Advanced

O(1) get and put with automatic eviction of the least-recently-used entry when the cache is full.

## The Problem This Solves

You're caching the results of expensive computations — database queries, rendered pages, API responses. You have limited memory, so you can't keep everything. You need an eviction policy. The most principled simple policy is LRU: when the cache is full and you need to insert something new, evict the entry that was accessed least recently. The intuition: if you haven't used something in a while, you're less likely to need it soon.

The challenge is doing this in O(1) for every operation. A `HashMap` gives O(1) lookup but has no concept of access order. A doubly-linked list tracks access order (most recent at front, least recent at back) and lets you move any node to the front in O(1) — but finding a node by key is O(n).

The classic solution: combine both. `HashMap<Key, Pointer to ListNode>` for O(1) lookup, doubly-linked list for O(1) order tracking. On every get/put, move the accessed node to the front of the list. On eviction, remove the tail.

## The Intuition

Python's `functools.lru_cache` does this for you, and `collections.OrderedDict` exposes the primitives to build it manually. Rust has neither in the standard library — but the `lru` crate wraps this pattern behind a clean API.

The tricky part in safe Rust: a doubly-linked list requires nodes with two mutable pointers (prev and next), which conflicts with Rust's aliasing rules. The standard solutions are: use `HashMap` + `VecDeque` with index-based addressing (good enough for most cases), use `unsafe` for raw pointers, or reach for the `lru` crate which handles this correctly.

## How It Works in Rust

```rust
// Idiomatic implementation using HashMap + ordered index tracking
// For production use, prefer the `lru` crate
use std::collections::HashMap;

struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: Vec<K>, // front = most recently used, back = LRU
}

impl<K: Clone + Eq + std::hash::Hash, V> LruCache<K, V> {
    fn new(capacity: usize) -> Self {
        LruCache { capacity, map: HashMap::new(), order: Vec::new() }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Move to front of order (mark as recently used)
            self.order.retain(|k| k != key);
            self.order.insert(0, key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        } else if self.map.len() >= self.capacity {
            // Evict the LRU entry (last in order)
            if let Some(lru_key) = self.order.pop() {
                self.map.remove(&lru_key);
            }
        }
        self.order.insert(0, key.clone());
        self.map.insert(key, value);
    }

    fn len(&self) -> usize { self.map.len() }
}

// Usage
let mut cache: LruCache<i32, &str> = LruCache::new(3);
cache.put(1, "one");
cache.put(2, "two");
cache.put(3, "three");
// Order: [3, 2, 1] (3 most recent)

cache.get(&1); // access 1 → moves to front
// Order: [1, 3, 2]

cache.put(4, "four"); // capacity full → evict 2 (LRU)
// Order: [4, 1, 3]

assert!(cache.get(&2).is_none()); // 2 was evicted

// With the `lru` crate (production-ready, O(1) everything):
// use lru::LruCache;
// use std::num::NonZeroUsize;
// let mut cache = LruCache::new(NonZeroUsize::new(3).unwrap());
// cache.put("key", "value");
// cache.get("key");
```

## What This Unlocks

- **Web server page caching**: cache rendered HTML pages keyed by URL; the N most recently visited pages stay hot in memory.
- **Database query result caching**: expensive JOIN results cached by query hash; LRU ensures rarely-run queries don't crowd out frequent ones.
- **OS page table management**: the fundamental algorithm behind virtual memory page replacement — the OS evicts the page that hasn't been accessed longest.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| LRU cache | not in stdlib | custom or `lru` crate |
| O(1) lookup | `Hashtbl` | `HashMap` |
| O(1) order tracking | manual doubly-linked list | `VecDeque` (O(n) move-to-front) or `unsafe` list |
| Production-ready impl | `lru-cache` opam package | `lru` crate |
| Python equivalent | `functools.lru_cache` / `OrderedDict` | `lru` crate |
| Eviction policy | varies | LRU (least recently used) |

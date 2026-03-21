📖 **[View on hightechmind.io →](https://hightechmind.io/rust/375-lru-cache)**

---

# 375: LRU Cache
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Cache memory is finite. When CPU caches, web caches, and database buffer pools fill up, they must decide which entries to evict. The Least Recently Used (LRU) policy discards the entry that has not been accessed for the longest time, exploiting temporal locality — the observation that recently accessed data is more likely to be accessed again soon. The challenge is achieving O(1) time for both lookup and eviction simultaneously.

LRU caches appear in operating system page replacement, CPU cache controllers, database buffer managers (PostgreSQL's shared_buffers), DNS resolvers, web CDNs, and Redis's `maxmemory-policy lru` mode.

## Learning Outcomes

- Understand why LRU approximates optimal cache replacement policy
- Learn how to combine `HashMap` and `VecDeque` to achieve O(1) average operations
- Understand the trade-off between peek (non-promoting) and get (promoting) access
- See how Rust's ownership model handles the dual-indexing structure required for LRU
- Understand eviction logic and capacity enforcement

## Rust Application

The implementation in `src/lib.rs` uses `HashMap<K, V>` for O(1) key lookup combined with `VecDeque<K>` to track recency order. The `get` method calls `order.retain()` to remove the key from its current position, then `push_front` to mark it most-recently-used. The `put` method evicts `order.pop_back()` when at capacity. A `peek` method allows reading without affecting LRU order — important for monitoring and introspection use cases.

The generic bounds `K: Clone + Eq + Hash` reflect exactly what operations the data structure needs: cloning keys for dual storage, equality for deduplication, and hashing for the map.

## OCaml Approach

The OCaml version in `example.ml` uses a mutable association list (`('k * 'v) list`) stored most-recent-first. The `get` function uses `List.assoc_opt` for lookup, then reconstructs the list with the found pair at the front. The `put` function filters out any existing key, prepends the new pair, then truncates to capacity with `List.filteri`. This is idiomatic functional style — immutable-feeling operations on mutable record fields — but O(n) per operation due to list traversal.

## Key Differences

1. **Complexity**: Rust's `HashMap + VecDeque` achieves O(1) average for get/put; OCaml's association list is O(n) per operation due to linear scan.
2. **Mutation model**: Rust uses `&mut self` methods that mutate in place; OCaml uses a mutable record field (`lru.data <- ...`) with reconstructed lists, blending functional and imperative styles.
3. **Type safety**: Rust's generic bounds `K: Clone + Eq + Hash` are checked at compile time; OCaml uses polymorphic types with structural equality by default.
4. **Memory**: Rust's dual-structure (map + deque) has stable memory layout; OCaml's list allocates cons cells on the GC heap with pointer chasing.

## Exercises

1. **Reimplement with a doubly-linked list**: Replace `VecDeque` + `HashMap<K,V>` with a proper doubly-linked list and `HashMap<K, NodePtr>` to achieve true O(1) without the O(n) `retain` call in the current implementation.
2. **Add TTL expiration**: Extend the cache so entries expire after a configurable duration. Use `std::time::Instant` stored alongside each value and check on access.
3. **Thread-safe LRU**: Wrap the cache in `Arc<Mutex<...>>` and write a test that spawns 8 threads concurrently calling `get` and `put`, verifying no data races occur and capacity is never exceeded.
